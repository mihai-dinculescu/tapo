//! TSLP, the framing that a device in setup mode takes its requests in over
//! Bluetooth: a 24-byte big-endian header followed by the payload.

use crate::Error;

pub(super) const HEADER_LENGTH: usize = 24;

const VERSION: u8 = 1;
/// What the CRC field holds while the CRC is computed.
const CRC_PLACEHOLDER: u32 = 0x92BF_30ED;
const CHANNEL_LENGTH: usize = 8;

/// The channel of a business packet, whose payload is encrypted.
pub(super) const BUSINESS_CHANNEL: &str = "RAW";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    /// The login, whose payload is JSON.
    Auth = 1,
    /// Everything after the login, whose payload is encrypted.
    Business = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Packet {
    /// `1` for a request and `2` for a reply. The Tapo app sends business
    /// requests with `2` as well.
    pub direction: u8,
    pub kind: Kind,
    pub channel: String,
    pub sequence: i32,
    pub payload: Vec<u8>,
}

impl Packet {
    /// Encodes the packet, with the CRC32 of the whole packet computed with
    /// the CRC field set to its placeholder.
    pub fn encode(&self) -> Vec<u8> {
        let mut channel = [0u8; CHANNEL_LENGTH];
        let channel_bytes = self.channel.as_bytes();
        let channel_length = channel_bytes.len().min(CHANNEL_LENGTH);
        channel[..channel_length].copy_from_slice(&channel_bytes[..channel_length]);

        let mut packet = Vec::with_capacity(HEADER_LENGTH + self.payload.len());
        packet.extend_from_slice(&[VERSION, self.direction, self.kind as u8, 0]);
        packet.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        packet.extend_from_slice(&channel);
        packet.extend_from_slice(&self.sequence.to_be_bytes());
        packet.extend_from_slice(&CRC_PLACEHOLDER.to_be_bytes());
        packet.extend_from_slice(&self.payload);

        let crc = crc32fast::hash(&packet);
        packet[20..24].copy_from_slice(&crc.to_be_bytes());

        packet
    }

    /// The length of the whole packet that starts with `header`, or `None`
    /// while fewer than [`HEADER_LENGTH`] bytes have arrived.
    pub fn total_length(header: &[u8]) -> Option<usize> {
        let length: [u8; 4] = header.get(4..8)?.try_into().ok()?;

        (header.len() >= HEADER_LENGTH)
            .then(|| HEADER_LENGTH.saturating_add(u32::from_be_bytes(length) as usize))
    }

    /// Decodes a whole packet.
    ///
    /// # Errors
    ///
    /// Returns an error if the packet is shorter than its header says, if
    /// its CRC does not match, or if its version or kind is not known.
    pub fn decode(packet: &[u8]) -> Result<Self, Error> {
        let Some(total_length) = Self::total_length(packet) else {
            return Err(error(format!(
                "The packet of {} bytes is shorter than its {HEADER_LENGTH}-byte header",
                packet.len()
            )));
        };
        if packet.len() != total_length {
            return Err(error(format!(
                "The packet is {} bytes long, but its header says {total_length}",
                packet.len()
            )));
        }

        if packet[0] != VERSION {
            return Err(error(format!(
                "The packet has version {}, which is not supported",
                packet[0]
            )));
        }

        let kind = match packet[2] {
            1 => Kind::Auth,
            2 => Kind::Business,
            kind => return Err(error(format!("The packet has the unknown kind {kind}"))),
        };

        let crc = u32::from_be_bytes(packet[20..24].try_into().expect("4 bytes"));
        let mut unsigned = packet.to_vec();
        unsigned[20..24].copy_from_slice(&CRC_PLACEHOLDER.to_be_bytes());
        if crc32fast::hash(&unsigned) != crc {
            return Err(error("The packet's CRC does not match".to_string()));
        }

        let channel = &packet[8..8 + CHANNEL_LENGTH];
        let channel_length = channel
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(CHANNEL_LENGTH);

        Ok(Self {
            direction: packet[1],
            kind,
            channel: String::from_utf8_lossy(&channel[..channel_length]).into_owned(),
            sequence: i32::from_be_bytes(packet[16..20].try_into().expect("4 bytes")),
            payload: packet[HEADER_LENGTH..].to_vec(),
        })
    }
}

fn error(description: String) -> Error {
    Error::Tapo(crate::TapoResponseError::ResponseError { description })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth_packet() -> Packet {
        Packet {
            direction: 1,
            kind: Kind::Auth,
            channel: String::new(),
            sequence: 0x0102_0304,
            payload: br#"{"method":"login"}"#.to_vec(),
        }
    }

    #[test]
    fn encode_matches_a_known_answer() {
        let packet = Packet {
            direction: 2,
            kind: Kind::Business,
            channel: BUSINESS_CHANNEL.to_string(),
            sequence: 7,
            payload: vec![0xAA, 0xBB],
        };

        assert_eq!(
            base16ct::lower::encode_string(&packet.encode()),
            "01020200000000025241570000000000000000075576be6daabb"
        );
    }

    #[test]
    fn encode_decode_round_trip() {
        for packet in [
            auth_packet(),
            Packet {
                direction: 2,
                kind: Kind::Business,
                channel: BUSINESS_CHANNEL.to_string(),
                sequence: -5,
                payload: vec![1; 300],
            },
        ] {
            let encoded = packet.encode();

            assert_eq!(Packet::total_length(&encoded), Some(encoded.len()));
            assert_eq!(Packet::decode(&encoded).unwrap(), packet);
        }
    }

    #[test]
    fn total_length_waits_for_the_whole_header() {
        let encoded = auth_packet().encode();

        assert_eq!(Packet::total_length(&encoded[..HEADER_LENGTH - 1]), None);
        assert_eq!(Packet::total_length(&[]), None);
    }

    #[test]
    fn decode_rejects_a_short_buffer() {
        let encoded = auth_packet().encode();

        assert!(Packet::decode(&encoded[..10]).is_err());
        assert!(Packet::decode(&encoded[..encoded.len() - 1]).is_err());
    }

    #[test]
    fn decode_rejects_a_wrong_crc() {
        let mut encoded = auth_packet().encode();
        let last = encoded.len() - 1;
        encoded[last] ^= 1;

        assert!(Packet::decode(&encoded).is_err());
    }
}
