//! Just enough MPEG-TS parsing to know how much playback time a stream
//! covers and where the next recording starts.
//!
//! The hub plays a recording on through the following footage rather than
//! stopping at the requested end time, and it restamps the clock so that the
//! PCR and PTS run on without a jump across the switch (checked on an H200
//! capture on 2026-09-16). What does mark the switch is a keyframe: the next
//! recording always starts on one. The hub's recording index is in whole
//! seconds, so a download stops at the first keyframe within a second of the
//! clip's length, or once the clock has advanced by the clip's length.

use std::time::Duration;

use log::debug;

/// MPEG-TS packets are 188 bytes and start with this sync byte.
const PACKET_SIZE: usize = 188;
const SYNC_BYTE: u8 = 0x47;
/// The PCR base and the PTS tick at 90 kHz and wrap after 33 bits.
const PCR_HZ: u64 = 90_000;
const PCR_MODULUS: u64 = 1 << 33;
/// Forward steps above this (one hour) are treated as discontinuities and not
/// counted, as are backward steps.
const MAX_STEP_TICKS: u64 = 3600 * PCR_HZ;
/// How far before the clip's length a keyframe counts as the start of the
/// next recording: the hub's recording index is in whole seconds.
const KEYFRAME_WINDOW: Duration = Duration::from_secs(1);

/// Follows the clock of a transport stream fed in arbitrary chunks: the
/// playback time covered by its PCR, and the keyframe that starts the
/// recording after the clip.
#[derive(Debug, Default)]
pub(super) struct StreamClock {
    clip_length: Option<Duration>,
    /// Bytes of an incomplete packet left over from the previous chunk.
    pending: Vec<u8>,
    last_base: Option<u64>,
    elapsed_ticks: u64,
    /// The PTS of the first video frame; positions are relative to it.
    first_pts: Option<u64>,
    ended: bool,
}

impl StreamClock {
    /// With `clip_length`, the clock also looks for the keyframe that starts
    /// the next recording.
    pub fn new(clip_length: Option<Duration>) -> Self {
        Self {
            clip_length,
            ..Self::default()
        }
    }

    /// Feeds the next chunk of the stream. Returns `None` while the chunk
    /// continues the recording, or `Some(keep)` at the keyframe that starts
    /// the next one: only the first `keep` bytes of `data` belong to the
    /// clip. Every later call returns `Some(0)`.
    pub fn observe(&mut self, data: &[u8]) -> Option<usize> {
        if self.ended {
            return Some(0);
        }

        let pending_before = self.pending.len();
        self.pending.extend_from_slice(data);

        let mut offset = 0;
        while self.pending.len() - offset >= PACKET_SIZE {
            let mut packet = [0u8; PACKET_SIZE];
            packet.copy_from_slice(&self.pending[offset..offset + PACKET_SIZE]);
            if packet[0] != SYNC_BYTE {
                // Resynchronise on the next sync byte.
                offset += packet[1..]
                    .iter()
                    .position(|byte| *byte == SYNC_BYTE)
                    .map(|position| position + 1)
                    .unwrap_or(PACKET_SIZE);
                continue;
            }

            if let Some((base, discontinuity)) = pcr_base(&packet) {
                self.record(base, discontinuity);
            }

            if self.starts_next_recording(&packet) {
                self.ended = true;
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
    }

    /// Whether `packet` starts a keyframe within [`KEYFRAME_WINDOW`] of the
    /// clip's length, where the next recording begins.
    fn starts_next_recording(&mut self, packet: &[u8]) -> bool {
        let Some(clip_length) = self.clip_length else {
            return false;
        };
        let Some((pts, payload)) = video_pes_start(packet) else {
            return false;
        };

        let Some(first_pts) = self.first_pts else {
            self.first_pts = Some(pts);
            return false;
        };

        if !has_keyframe(payload) {
            return false;
        }

        let position = ticks_to_duration((pts + PCR_MODULUS - first_pts) % PCR_MODULUS);
        if position + KEYFRAME_WINDOW < clip_length {
            return false;
        }

        debug!(
            "Keyframe at {position:?} of a {clip_length:?} clip; the next recording starts here"
        );
        true
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

/// The PTS and the elementary stream bytes of a packet that starts a video
/// PES packet (stream id `0xE0..=0xEF`) carrying a PTS.
fn video_pes_start(packet: &[u8]) -> Option<(u64, &[u8])> {
    let payload_unit_start = packet[1] & 0x40 != 0;
    let adaptation_field_control = (packet[3] >> 4) & 0x03;
    if !payload_unit_start || adaptation_field_control & 0x01 == 0 {
        return None;
    }

    let payload_offset = if adaptation_field_control & 0x02 != 0 {
        5 + usize::from(packet[4])
    } else {
        4
    };
    let payload = packet.get(payload_offset..)?;

    // PES start code, video stream id, and the PTS flag in the second flags
    // byte; the PTS follows the 9-byte fixed header.
    if payload.len() < 14
        || payload[..3] != [0x00, 0x00, 0x01]
        || payload[3] & 0xF0 != 0xE0
        || payload[7] & 0x80 == 0
    {
        return None;
    }

    let pts = &payload[9..14];
    let pts = (u64::from(pts[0] & 0x0E) << 29)
        | (u64::from(pts[1]) << 22)
        | (u64::from(pts[2] & 0xFE) << 14)
        | (u64::from(pts[3]) << 7)
        | (u64::from(pts[4]) >> 1);

    let header_length = 9 + usize::from(payload[8]);
    Some((pts, payload.get(header_length..).unwrap_or(&[])))
}

/// Whether the H.264 bytes hold a sequence parameter set or an IDR slice,
/// either of which starts a keyframe.
fn has_keyframe(elementary: &[u8]) -> bool {
    elementary
        .windows(4)
        .any(|window| window[..3] == [0x00, 0x00, 0x01] && matches!(window[3] & 0x9F, 5 | 7))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIDEO_PID: u16 = 68;

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

    /// A packet starting a video PES with `pts`, whose first NAL unit is a
    /// sequence parameter set (a keyframe) or a non-IDR slice.
    fn video_frame_packet(pts: u64, keyframe: bool) -> [u8; PACKET_SIZE] {
        let mut packet = [0xffu8; PACKET_SIZE];
        packet[0] = SYNC_BYTE;
        packet[1] = 0x40 | (VIDEO_PID >> 8) as u8; // payload unit start
        packet[2] = VIDEO_PID as u8;
        packet[3] = 0x10; // payload only
        packet[4..8].copy_from_slice(&[0x00, 0x00, 0x01, 0xE0]);
        packet[8..10].copy_from_slice(&[0x00, 0x00]); // unbounded PES length
        packet[10] = 0x80;
        packet[11] = 0x80; // PTS present
        packet[12] = 5; // header data length
        packet[13] = 0x21 | ((pts >> 29) & 0x0E) as u8;
        packet[14] = (pts >> 22) as u8;
        packet[15] = 0x01 | ((pts >> 14) & 0xFE) as u8;
        packet[16] = (pts >> 7) as u8;
        packet[17] = 0x01 | ((pts << 1) & 0xFE) as u8;
        packet[18..23].copy_from_slice(&[
            0x00,
            0x00,
            0x00,
            0x01,
            if keyframe { 0x67 } else { 0x41 },
        ]);
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
    fn test_video_pes_start_round_trip() {
        let pts = (1 << 33) - 12_345;
        let keyframe = video_frame_packet(pts, true);
        let (parsed, elementary) = video_pes_start(&keyframe).unwrap();
        assert_eq!(parsed, pts);
        assert!(has_keyframe(elementary));

        let frame = video_frame_packet(pts, false);
        let (_, elementary) = video_pes_start(&frame).unwrap();
        assert!(!has_keyframe(elementary));

        assert_eq!(video_pes_start(&packet_with_pcr(0, false)), None);
        assert_eq!(video_pes_start(&packet_without_pcr()), None);
    }

    #[test]
    fn test_elapsed_accumulates_across_chunks_and_packet_boundaries() {
        let mut clock = StreamClock::default();
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
    fn test_elapsed_handles_wrap_and_ignores_discontinuities_and_backward_steps() {
        let mut clock = StreamClock::default();

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
    fn test_observe_stops_at_the_keyframe_within_a_second_of_the_clip_length() {
        let mut clock = StreamClock::new(Some(Duration::from_secs(30)));
        let first_pts = (1 << 33) - 90_000 * 10; // wraps during the clip

        // The clip's own keyframes, two seconds apart, up to 28 s.
        for second in (0..=28).step_by(2) {
            let pts = (first_pts + 90_000 * second) % PCR_MODULUS;
            assert_eq!(clock.observe(&video_frame_packet(pts, true)), None);
            assert_eq!(
                clock.observe(&video_frame_packet(pts + 45_000, false)),
                None
            );
        }

        // A non-keyframe inside the window does not end the clip.
        let pts = (first_pts + 90_000 * 29) % PCR_MODULUS;
        assert_eq!(clock.observe(&video_frame_packet(pts, false)), None);

        // The next recording starts on a keyframe at 29.5 s: the packet
        // before it is kept, the keyframe is not.
        let mut chunk = Vec::new();
        chunk.extend_from_slice(&packet_without_pcr());
        chunk.extend_from_slice(&video_frame_packet(pts + 45_000, true));
        chunk.extend_from_slice(&video_frame_packet(pts + 50_000, false));
        assert_eq!(clock.observe(&chunk), Some(PACKET_SIZE));

        // Nothing after the cut is kept.
        assert_eq!(
            clock.observe(&video_frame_packet(pts + 55_000, true)),
            Some(0)
        );
    }

    #[test]
    fn test_observe_ignores_keyframes_before_the_window_and_without_a_clip_length() {
        let mut clock = StreamClock::new(Some(Duration::from_secs(30)));
        assert_eq!(clock.observe(&video_frame_packet(0, true)), None);
        // 28.9 s: just outside the window.
        assert_eq!(
            clock.observe(&video_frame_packet(90_000 * 289 / 10, true)),
            None
        );
        // 29.0 s: the window starts here.
        assert_eq!(
            clock.observe(&video_frame_packet(90_000 * 29, true)),
            Some(0)
        );

        let mut clock = StreamClock::new(None);
        assert_eq!(clock.observe(&video_frame_packet(0, true)), None);
        assert_eq!(
            clock.observe(&video_frame_packet(90_000 * 3600, true)),
            None
        );
    }

    #[test]
    fn test_observe_does_not_end_a_short_clip_on_its_first_keyframe() {
        let mut clock = StreamClock::new(Some(Duration::from_secs(1)));
        assert_eq!(clock.observe(&video_frame_packet(0, true)), None);
        assert_eq!(clock.observe(&video_frame_packet(45_000, false)), None);
        assert_eq!(clock.observe(&video_frame_packet(90_000, true)), Some(0));
    }

    #[test]
    fn test_observe_cuts_before_a_packet_that_started_in_the_previous_chunk() {
        let mut clock = StreamClock::new(Some(Duration::from_secs(2)));
        let mut stream = Vec::new();
        stream.extend_from_slice(&video_frame_packet(0, true));
        stream.extend_from_slice(&video_frame_packet(90_000, false));
        stream.extend_from_slice(&video_frame_packet(90_000 * 2, true)); // the next recording

        // The keyframe packet is split across two chunks: its first 100
        // bytes go out with the first chunk, so the second chunk keeps
        // nothing.
        assert_eq!(clock.observe(&stream[..PACKET_SIZE * 2 + 100]), None);
        assert_eq!(clock.observe(&stream[PACKET_SIZE * 2 + 100..]), Some(0));
    }

    #[test]
    fn test_observe_resynchronises_on_garbage() {
        let mut clock = StreamClock::default();
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
