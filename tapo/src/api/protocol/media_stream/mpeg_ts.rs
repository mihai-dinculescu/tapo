//! Just enough MPEG-TS parsing to know how much playback time a stream
//! covers: the Program Clock Reference (PCR) carried in adaptation fields.
//!
//! The hub plays a recording on through the following footage rather than
//! stopping at the requested end time, so a download has to stop itself: at
//! the first jump of the clock, which is where the hub splices in the next
//! recording, or once the clock has advanced by the clip's length.

use std::time::Duration;

use log::debug;

/// MPEG-TS packets are 188 bytes and start with this sync byte.
const PACKET_SIZE: usize = 188;
const SYNC_BYTE: u8 = 0x47;
/// The PCR base ticks at 90 kHz and wraps after 33 bits.
const PCR_HZ: u64 = 90_000;
const PCR_MODULUS: u64 = 1 << 33;
/// Forward steps above this (one hour) are treated as splices, as are backward
/// steps and packets flagged as discontinuities.
const MAX_STEP_TICKS: u64 = 3600 * PCR_HZ;

/// Accumulates the playback time covered by the PCR of a transport stream,
/// fed in arbitrary chunks, until the clock jumps to another recording.
#[derive(Debug, Default)]
pub(super) struct PcrClock {
    /// Bytes of an incomplete packet left over from the previous chunk.
    pending: Vec<u8>,
    last_base: Option<u64>,
    elapsed_ticks: u64,
    spliced: bool,
}

impl PcrClock {
    /// Feeds the next chunk of the stream. Returns `None` while the chunk
    /// continues the recording, or `Some(keep)` once the clock jumps: only the
    /// first `keep` bytes of `data` belong to the recording, and the rest is
    /// the one the hub spliced in after it. Every later call returns
    /// `Some(0)`.
    pub fn observe(&mut self, data: &[u8]) -> Option<usize> {
        if self.spliced {
            return Some(0);
        }

        let pending_before = self.pending.len();
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

            if let Some((base, discontinuity)) = pcr_base(packet)
                && self.record(base, discontinuity)
            {
                self.spliced = true;
                self.pending.clear();
                // A packet that started in the previous chunk has already
                // been handed out with it; the file then ends on a partial
                // packet, which demuxers ignore.
                return Some(offset.saturating_sub(pending_before));
            }
            offset += PACKET_SIZE;
        }

        self.pending.drain(..offset);
        None
    }

    /// Adds the step from the previous PCR. Returns `true` when the step is
    /// a jump to another recording instead.
    fn record(&mut self, base: u64, discontinuity: bool) -> bool {
        let Some(last) = self.last_base else {
            self.last_base = Some(base);
            return false;
        };

        let delta = (base + PCR_MODULUS - last) % PCR_MODULUS;
        if discontinuity || delta >= MAX_STEP_TICKS {
            debug!(
                "The MPEG-TS clock jumped from {last} to {base} (discontinuity flag: {discontinuity}) after {:?}; another recording follows",
                ticks_to_duration(self.elapsed_ticks)
            );
            return true;
        }

        self.elapsed_ticks += delta;
        self.last_base = Some(base);
        false
    }

    /// The playback time covered so far, once a PCR has been seen.
    pub fn elapsed(&self) -> Option<Duration> {
        self.last_base
            .is_some()
            .then(|| ticks_to_duration(self.elapsed_ticks))
    }
}

fn ticks_to_duration(ticks: u64) -> Duration {
    Duration::from_micros(ticks * 1_000_000 / PCR_HZ)
}

/// Whether `body` starts with MPEG-TS packets: a sync byte at every packet
/// boundary that falls inside the body.
pub(super) fn looks_like_mpeg_ts(body: &[u8]) -> bool {
    !body.is_empty()
        && body
            .iter()
            .step_by(PACKET_SIZE)
            .all(|byte| *byte == SYNC_BYTE)
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
            assert_eq!(clock.observe(chunk), None);
        }

        assert_eq!(clock.elapsed(), Some(Duration::from_secs(3)));
    }

    #[test]
    fn test_elapsed_handles_wrap() {
        let mut clock = PcrClock::default();

        assert_eq!(
            clock.observe(&packet_with_pcr((1 << 33) - 90_000, false)),
            None
        );
        assert_eq!(clock.observe(&packet_with_pcr(90_000, false)), None); // wraps: +2 s
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(2)));
    }

    #[test]
    fn test_observe_stops_at_a_flagged_discontinuity() {
        let mut clock = PcrClock::default();

        // A discontinuity flag on the first PCR seen is not a jump.
        assert_eq!(clock.observe(&packet_with_pcr(0, true)), None);
        assert_eq!(clock.observe(&packet_with_pcr(90_000, false)), None);

        let mut chunk = Vec::new();
        chunk.extend_from_slice(&packet_without_pcr());
        chunk.extend_from_slice(&packet_with_pcr(90_000 * 2, true));
        chunk.extend_from_slice(&packet_with_pcr(90_000 * 3, false));

        // Only the packet before the flagged one belongs to the recording.
        assert_eq!(clock.observe(&chunk), Some(PACKET_SIZE));
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(1)));

        // Nothing after the splice is kept.
        assert_eq!(clock.observe(&packet_with_pcr(90_000 * 4, false)), Some(0));
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(1)));
    }

    #[test]
    fn test_observe_stops_at_backward_and_large_forward_steps() {
        let mut clock = PcrClock::default();
        assert_eq!(clock.observe(&packet_with_pcr(90_000 * 10, false)), None);
        assert_eq!(clock.observe(&packet_with_pcr(90_000 * 11, false)), None);
        assert_eq!(clock.observe(&packet_with_pcr(90_000 * 5, false)), Some(0)); // backward
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(1)));

        let mut clock = PcrClock::default();
        assert_eq!(clock.observe(&packet_with_pcr(0, false)), None);
        assert_eq!(
            clock.observe(&packet_with_pcr(MAX_STEP_TICKS, false)), // one hour ahead
            Some(0)
        );
        assert_eq!(clock.elapsed(), Some(Duration::ZERO));
    }

    #[test]
    fn test_observe_cuts_before_a_packet_that_started_in_the_previous_chunk() {
        let mut clock = PcrClock::default();
        let mut stream = Vec::new();
        stream.extend_from_slice(&packet_with_pcr(0, false));
        stream.extend_from_slice(&packet_with_pcr(90_000, false));
        stream.extend_from_slice(&packet_with_pcr(45_000, false)); // backward: a splice

        // The splicing packet is split across two chunks: its first 100 bytes
        // go out with the first chunk, so the second chunk keeps nothing.
        assert_eq!(clock.observe(&stream[..PACKET_SIZE * 2 + 100]), None);
        assert_eq!(clock.observe(&stream[PACKET_SIZE * 2 + 100..]), Some(0));
        assert_eq!(clock.elapsed(), Some(Duration::from_secs(1)));
    }

    #[test]
    fn test_observe_resynchronises_on_garbage() {
        let mut clock = PcrClock::default();
        let mut stream = vec![0x00u8; 10];
        stream.extend_from_slice(&packet_with_pcr(0, false));
        stream.extend_from_slice(&packet_with_pcr(45_000, false));

        assert_eq!(clock.observe(&stream), None);

        assert_eq!(clock.elapsed(), Some(Duration::from_millis(500)));
    }

    #[test]
    fn test_looks_like_mpeg_ts() {
        let mut packets = vec![0u8; 188 * 3];
        for index in [0, 188, 376] {
            packets[index] = 0x47;
        }
        assert!(looks_like_mpeg_ts(&packets));
        // A partial trailing packet is still MPEG-TS.
        assert!(looks_like_mpeg_ts(&packets[..300]));

        assert!(!looks_like_mpeg_ts(&[]));
        assert!(!looks_like_mpeg_ts(&[0x00; 188]));
        packets[188] = 0x00;
        assert!(!looks_like_mpeg_ts(&packets));
    }
}
