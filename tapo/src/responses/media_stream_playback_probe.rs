use serde::{Deserialize, Serialize};

use crate::responses::MediaStreamSession;

/// What a camera hub sent while a recording stored on it was played back over
/// the media stream service (TCP port 8800). Media parts are counted, not
/// kept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStreamPlaybackProbe {
    /// The authenticated media stream session the playback ran on.
    pub session: MediaStreamSession,
    /// The playback session id issued by the hub in its response to the
    /// playback request, when it reports one.
    pub playback_session_id: Option<String>,
    /// The playback speed reported by the hub, when it reports one.
    pub speed: Option<String>,
    /// The number of media (non-JSON) parts received.
    pub media_part_count: u64,
    /// The total size of the media parts received, in bytes.
    pub media_byte_count: u64,
    /// The distinct `Content-Type` values of the media parts, in order of
    /// first appearance. The Tapo app expects `video/mp2t` (MPEG-TS).
    pub media_content_types: Vec<String>,
    /// The `X-Data-Sequence` of the last media part received.
    pub last_data_sequence: Option<u64>,
    /// Whether the hub flagged the parts as encrypted (`X-If-Encrypt: 1`).
    /// Encrypted parts are not supported yet.
    pub encrypted: bool,
    /// The `event_type` of every notification the hub sent, in order.
    pub event_types: Vec<String>,
    /// Why the playback stopped.
    pub outcome: MediaStreamPlaybackOutcome,
}

/// Why a media stream playback stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaStreamPlaybackOutcome {
    /// The hub reported the end of the recording.
    Finished,
    /// The hub closed the session or the connection.
    ClosedByHub,
    /// The requested duration elapsed and the client stopped the playback.
    DurationElapsed,
}
