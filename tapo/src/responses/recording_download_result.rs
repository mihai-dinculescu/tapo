use serde::{Deserialize, Serialize};

/// The outcome of downloading a recording stored on a camera hub with
/// [`CameraHubHandler::download_recording`](crate::CameraHubHandler::download_recording).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingDownloadResult {
    /// The number of media bytes written, after decryption.
    pub byte_count: u64,
    /// The playback time the media covers, in seconds, read from the
    /// MPEG-TS clock. `None` when the hub sent a stream without a usable
    /// clock, in which case the length of the download is unknown.
    pub duration_s: Option<f64>,
    /// Why the download stopped.
    pub outcome: RecordingDownloadOutcome,
}

/// Why a recording download stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingDownloadOutcome {
    /// The media covered the requested time range. The hub plays on through
    /// the footage that follows, so this is the usual outcome.
    ClipEndReached,
    /// The hub reported the end of the recording before the requested time
    /// range was covered, so the download may be shorter than asked for.
    Finished,
    /// The hub closed the session or the connection early.
    ClosedByHub,
    /// The time limit elapsed first, so the download is incomplete.
    DurationElapsed,
}
