use serde::{Deserialize, Serialize};

use crate::responses::MediaStreamSession;

/// What a camera hub sent while a recording stored on it was played back over
/// the media stream service (TCP port 8800).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStreamPlaybackResult {
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
    /// Whether the first media part, after decryption when applicable, looks
    /// like MPEG-TS (a `0x47` sync byte every 188 bytes). `None` until a
    /// media part has arrived.
    pub media_is_mpeg_ts: Option<bool>,
    /// The playback time covered by the media so far, from the MPEG-TS
    /// Program Clock Reference, when one was found.
    pub media_duration_s: Option<f64>,
    /// The `X-Data-Sequence` of the last media part received.
    pub last_data_sequence: Option<u64>,
    /// Whether the hub flagged the parts as encrypted (`X-If-Encrypt: 1`).
    /// An H200 does so even on a LAN session it declares
    /// `X-Encrypt-Type: PLAIN`.
    pub encrypted: bool,
    /// Whether the media parts were decrypted with keys derived from the
    /// session's `Key-Exchange` header before being written and inspected.
    pub decrypted: bool,
    /// Whether the `X-Data-Hmac` of the first encrypted part matched one of
    /// the candidate secrets. `None` when the parts carried no HMAC or were
    /// not encrypted.
    pub hmac_verified: Option<bool>,
    /// The number of encrypted parts whose `X-Data-Hmac` did not match once a
    /// secret had been chosen. Such parts are dropped, not written.
    pub hmac_mismatch_count: u64,
    /// The headers of the first media part received, verbatim (names
    /// lower-cased). When the parts are encrypted, this exposes the crypto
    /// headers the Tapo app decrypts with (`x-nonce` as the per-part IV,
    /// `x-data-hmac`, and any `x-chain` / `x-password` key material), which
    /// is what a decryption implementation needs. Empty until a media part
    /// has arrived.
    pub first_media_part_headers: Vec<(String, String)>,
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
    /// The client stopped the playback once the media covered the clip's
    /// length. The hub plays on through the following footage otherwise.
    ClipEndReached,
    /// The hub closed the session or the connection.
    ClosedByHub,
    /// The time limit elapsed and the client stopped the playback.
    DurationElapsed,
}
