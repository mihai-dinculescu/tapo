use serde::Serialize;

/// `module → section → data` envelope for `searchVideoWithUTC`:
/// `{"playback": {"search_video_with_utc": {...}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamSearchVideoWithUtcParams {
    playback: PlaybackParams,
}

#[derive(Debug, Serialize)]
struct PlaybackParams {
    search_video_with_utc: UtcFilterParams,
}

#[derive(Debug, Serialize)]
struct UtcFilterParams {
    channel: u8,
    /// Unix timestamp (seconds).
    start_time: u64,
    /// Unix timestamp (seconds).
    end_time: u64,
    start_index: u64,
    end_index: u64,
    child_device_id: String,
    child_device_mac: String,
}

impl SmartCamSearchVideoWithUtcParams {
    pub fn new(
        start_time: u64,
        end_time: u64,
        start_index: u64,
        end_index: u64,
        child_device_id: String,
        child_device_mac: String,
    ) -> Self {
        Self {
            playback: PlaybackParams {
                search_video_with_utc: UtcFilterParams {
                    channel: 0,
                    start_time,
                    end_time,
                    start_index,
                    end_index,
                    child_device_id,
                    child_device_mac,
                },
            },
        }
    }
}
