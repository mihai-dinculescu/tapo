use serde::{Deserialize, Serialize};

/// The outcome of downloading a recording stored on a camera hub with
/// [`CameraHubHandler::download_recording`](crate::CameraHubHandler::download_recording).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingDownloadResult {
    /// The number of media bytes written, after decryption.
    pub byte_count: u64,
    /// The playback time the media covers, in seconds, measured between the
    /// first and the last MPEG-TS clock reference. The H200 sends one about
    /// every 2 s, so media after the last one is not counted and this can
    /// fall short of what was written by up to that much. `None` when the
    /// hub sent a stream without a usable clock, in which case the length of
    /// the download is unknown.
    pub duration_s: Option<f64>,
    /// Why the download stopped.
    pub outcome: RecordingDownloadOutcome,
}

/// Why a recording download stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingDownloadOutcome {
    /// The hub reported the end of the footage, which is the usual outcome:
    /// it ends a download itself at the end of the recording.
    Finished,
    /// The hub closed the connection early.
    ClosedByHub,
    /// The time limit elapsed first, so the download is incomplete.
    DurationElapsed,
}
