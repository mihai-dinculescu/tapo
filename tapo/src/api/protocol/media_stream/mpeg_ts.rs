//! Just enough MPEG-TS parsing to know how much playback time a stream
//! covers: the Program Clock Reference (PCR) carried in adaptation fields.
//!
//! The hub plays a recording on through the following footage rather than
//! stopping at the requested end time, so a download has to stop itself once
//! the clock has advanced by the clip's length.

use std::time::Duration;

pub(super) const PACKET_SIZE: usize = 188;
pub(super) const SYNC_BYTE: u8 = 0x47;
/// The PCR base ticks at 90 kHz and wraps after 33 bits.
const PCR_HZ: u64 = 90_000;
const PCR_MODULUS: u64 = 1 << 33;
/// Forward steps above this (one hour) are treated as discontinuities and not
/// counted, as are backward steps.
const MAX_STEP_TICKS: u64 = 3600 * PCR_HZ;

/// Accumulates the playback time covered by the PCR of a transport stream,
/// fed in arbitrary chunks.
#[derive(Debug, Default)]
pub(super) struct PcrClock {
    /// Bytes of an incomplete packet left over from the previous chunk.
    pending: Vec<u8>,
    last_base: Option<u64>,
    elapsed_ticks: u64,
    seen: bool,
}

impl PcrClock {
    pub fn observe(&mut self, data: &[u8]) {
        self.pending.extend_from_slice(data);

        let mut offset = 0;
        while self.pending.len() - offset >= PACKET_SIZE {
            let packet = &self.pending[offset..offset + PACKET_SIZE];
            if packet[0] != SYNC_BYTE {
                // Resynchronise on the next sync byte.
                offset += packet[1..]
                    .iter()
                    .position(|byte| *byte == SYNC_BYTE)
                    .map(|position| position + 1)
                    .unwrap_or(PACKET_SIZE);
                continue;
            }

            if let Some((base, discontinuity)) = pcr_base(packet) {
                self.record(base, discontinuity);
            }
            offset += PACKET_SIZE;
        }

        self.pending.drain(..offset.min(self.pending.len()));
    }

    fn record(&mut self, base: u64, discontinuity: bool) {
        if let Some(last) = self.last_base
            && !discontinuity
        {
            let delta = (base + PCR_MODULUS - last) % PCR_MODULUS;
            if delta < MAX_STEP_TICKS {
                self.elapsed_ticks += delta;
            }
        }
        self.last_base = Some(base);
        self.seen = true;
    }

    /// The playback time covered so far, once a PCR has been seen.
    pub fn elapsed(&self) -> Option<Duration> {
        self.seen
            .then(|| Duration::from_micros(self.elapsed_ticks * 1_000_000 / PCR_HZ))
    }
}

/// The 33-bit PCR base of a packet and its discontinuity indicator, when the
/// packet carries a PCR.
fn pcr_base(packet: &[u8]) -> Option<(u64, bool)> {
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
    let discontinuity = flags & 0x80 != 0;

    let pcr = &packet[6..12];
    let base = (u64::from(pcr[0]) << 25)
        | (u64::from(pcr[1]) << 17)
        | (u64::from(pcr[2]) << 9)
        | (u64::from(pcr[3]) << 1)
        | (u64::from(pcr[4]) >> 7);

    Some((base, discontinuity))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet_with_pcr(base: u64, discontinuity: bool) -> [u8; PACKET_SIZE] {
        let mut packet = [0xffu8; PACKET_SIZE];
        packet[0] = SYNC_BYTE;
        packet[1] = 0x40;
        packet[2] = 0x00;
        packet[3] = 0x30; // adaptation field + payload
        packet[4] = 7; // adaptation field length
        packet[5] = 0x10 | if discontinuity { 0x80 } else { 0 };
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
        let (parsed, discontinuity) = pcr_base(&packet_with_pcr(base, true)).unwrap();
        assert_eq!(parsed, base);
        assert!(discontinuity);

        assert_eq!(pcr_base(&packet_without_pcr()), None);
    }

    #[test]
    fn test_elapsed_accumulates_across_chunks_and_packet_boundaries() {
        let mut clock = PcrClock::default();
        assert_eq!(clock.elapsed(), None);

        let mut stream = Vec::new();
        stream.extend_from_slice(&packet_with_pcr(0, false));
        stream.extend_from_slice(&packet_without_pcr());
        stream.extend_from_slice(&packet_with_pcr(90_000, false)); // +1 s
        stream.extend_from_slice(&packet_with_pcr(90_000 * 3, false)); // +2 s

        // Feed in chunks that do not align with packets.
        for chunk in stream.chunks(100) {
            clock.observe(chunk);
        }

        assert_eq!(clock.elapsed(), Some(Duration::from_secs(3)));
    }

    #[test]
    fn test_elapsed_handles_wrap_and_ignores_discontinuities_and_backward_steps() {
        let mut clock = PcrClock::default();

        clock.observe(&packet_with_pcr((1 << 33) - 90_000, false));
        clock.observe(&packet_with_pcr(90_000, false)); // wraps: +2 s
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(2)));

        clock.observe(&packet_with_pcr(90_000 * 5000, true)); // discontinuity: not counted
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(2)));

        clock.observe(&packet_with_pcr(90_000 * 5001, false)); // +1 s after the jump
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(3)));

        clock.observe(&packet_with_pcr(90_000 * 4000, false)); // backward: not counted
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(3)));
    }

    #[test]
    fn test_observe_resynchronises_on_garbage() {
        let mut clock = PcrClock::default();
        let mut stream = vec![0x00u8; 10];
        stream.extend_from_slice(&packet_with_pcr(0, false));
        stream.extend_from_slice(&packet_with_pcr(45_000, false));

        clock.observe(&stream);

        assert_eq!(clock.elapsed(), Some(Duration::from_millis(500)));
    }
}
