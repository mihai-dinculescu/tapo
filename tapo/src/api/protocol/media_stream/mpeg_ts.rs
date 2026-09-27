//! Just enough MPEG-TS parsing to know how much playback time a stream
//! covers, which a download reports as the length of the media it wrote.

use std::time::Duration;

/// MPEG-TS packets are 188 bytes and start with this sync byte.
const PACKET_SIZE: usize = 188;
const SYNC_BYTE: u8 = 0x47;
/// The PCR base and the PTS tick at 90 kHz and wrap after 33 bits.
const PCR_HZ: u64 = 90_000;
const PCR_MODULUS: u64 = 1 << 33;

/// Follows the clock of a transport stream fed in arbitrary chunks: the
/// playback time between its first and its last PCR.
///
/// The hub ends a download at the end of the clip, so the stream carries a
/// single clock that only moves forward.
#[derive(Debug, Default)]
pub(super) struct StreamClock {
    /// Bytes of an incomplete packet left over from the previous chunk.
    pending: Vec<u8>,
    first_base: Option<u64>,
    last_base: Option<u64>,
}

impl StreamClock {
    /// Feeds the next chunk of the stream.
    pub fn observe(&mut self, data: &[u8]) {
        self.pending.extend_from_slice(data);

        let (packets, rest) = self.pending.as_chunks::<PACKET_SIZE>();
        for packet in packets {
            if let Some(base) = pcr_base(packet) {
                self.first_base.get_or_insert(base);
                self.last_base = Some(base);
            }
        }

        let consumed = self.pending.len() - rest.len();
        self.pending.drain(..consumed);
    }

    /// The playback time covered so far, once a PCR has been seen.
    pub fn elapsed(&self) -> Option<Duration> {
        let ticks = (self.last_base? + PCR_MODULUS - self.first_base?) % PCR_MODULUS;
        Some(Duration::from_micros(ticks * 1_000_000 / PCR_HZ))
    }
}

/// The 33-bit PCR base of a packet, when the packet carries a PCR.
fn pcr_base(packet: &[u8; PACKET_SIZE]) -> Option<u64> {
    if packet[0] != SYNC_BYTE {
        return None;
    }

    let adaptation_field_control = (packet[3] >> 4) & 0x03;
    if adaptation_field_control & 0x02 == 0 {
        return None;
    }

    let length = usize::from(packet[4]);
    // Flags plus the 6-byte PCR must fit in the adaptation field.
    if length < 7 {
        return None;
    }

    let flags = packet[5];
    if flags & 0x10 == 0 {
        return None;
    }

    let pcr = &packet[6..12];
    let base = (u64::from(pcr[0]) << 25)
        | (u64::from(pcr[1]) << 17)
        | (u64::from(pcr[2]) << 9)
        | (u64::from(pcr[3]) << 1)
        | (u64::from(pcr[4]) >> 7);

    Some(base)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet_with_pcr(base: u64) -> [u8; PACKET_SIZE] {
        let mut packet = [0xffu8; PACKET_SIZE];
        packet[0] = SYNC_BYTE;
        packet[1] = 0x40;
        packet[2] = 0x00;
        packet[3] = 0x30; // adaptation field + payload
        packet[4] = 7; // adaptation field length
        packet[5] = 0x10;
        packet[6] = (base >> 25) as u8;
        packet[7] = (base >> 17) as u8;
        packet[8] = (base >> 9) as u8;
        packet[9] = (base >> 1) as u8;
        packet[10] = ((base & 1) << 7) as u8 | 0x7e;
        packet[11] = 0x00;
        packet
    }

    fn packet_without_pcr() -> [u8; PACKET_SIZE] {
        let mut packet = [0x00u8; PACKET_SIZE];
        packet[0] = SYNC_BYTE;
        packet[3] = 0x10; // payload only
        packet
    }

    #[test]
    fn test_pcr_base_round_trip() {
        let base = (1 << 33) - 12_345;
        assert_eq!(pcr_base(&packet_with_pcr(base)), Some(base));

        assert_eq!(pcr_base(&packet_without_pcr()), None);
    }

    #[test]
    fn test_elapsed_spans_first_to_last_pcr_across_chunks() {
        let mut clock = StreamClock::default();
        assert_eq!(clock.elapsed(), None);

        let mut stream = Vec::new();
        stream.extend_from_slice(&packet_with_pcr(0));
        stream.extend_from_slice(&packet_without_pcr());
        stream.extend_from_slice(&packet_with_pcr(90_000)); // +1 s
        stream.extend_from_slice(&packet_with_pcr(90_000 * 3)); // +2 s

        // Feed in chunks that do not align with packets.
        for chunk in stream.chunks(100) {
            clock.observe(chunk);
        }

        assert_eq!(clock.elapsed(), Some(Duration::from_secs(3)));
    }

    #[test]
    fn test_elapsed_handles_wrap() {
        let mut clock = StreamClock::default();

        clock.observe(&packet_with_pcr((1 << 33) - 90_000));
        clock.observe(&packet_with_pcr(90_000)); // wraps: +2 s

        assert_eq!(clock.elapsed(), Some(Duration::from_secs(2)));
    }
}
