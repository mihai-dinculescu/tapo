use serde::Serialize;

/// `module → section → data` envelope for `searchDetectionList`:
/// `{"playback": {"search_detection_list": {...}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamSearchDetectionListParams {
    playback: PlaybackParams,
}

#[derive(Debug, Serialize)]
struct PlaybackParams {
    search_detection_list: DetectionFilterParams,
}

#[derive(Debug, Serialize)]
struct DetectionFilterParams {
    channel: u8,
    /// Unix timestamp (seconds).
    start_time: u64,
    /// Unix timestamp (seconds).
    end_time: u64,
    start_index: u64,
    end_index: u64,
}

impl SmartCamSearchDetectionListParams {
    pub fn new(start_time: u64, end_time: u64, start_index: u64, end_index: u64) -> Self {
        Self {
            playback: PlaybackParams {
                search_detection_list: DetectionFilterParams {
                    channel: 0,
                    start_time,
                    end_time,
                    start_index,
                    end_index,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_serialize_to_the_camera_filter() {
        let params = SmartCamSearchDetectionListParams::new(1792783708, 1792870108, 0, 99);

        assert_eq!(
            serde_json::to_string(&params).unwrap(),
            r#"{"playback":{"search_detection_list":{"channel":0,"start_time":1792783708,"end_time":1792870108,"start_index":0,"end_index":99}}}"#
        );
    }
}
