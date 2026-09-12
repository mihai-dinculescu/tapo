use chrono::NaiveDate;
use serde::Serialize;

/// `module → section → data` envelope for `searchDateWithVideo`:
/// `{"playback": {"search_year_utility": {...}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamSearchDateWithVideoParams {
    playback: PlaybackParams,
}

#[derive(Debug, Serialize)]
struct PlaybackParams {
    search_year_utility: DateFilterParams,
}

#[derive(Debug, Serialize)]
struct DateFilterParams {
    channel: Vec<u8>,
    /// `YYYYMMDD`
    start_date: String,
    /// `YYYYMMDD`
    end_date: String,
    child_device_id: String,
    child_device_mac: String,
}

impl SmartCamSearchDateWithVideoParams {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        child_device_id: String,
        child_device_mac: String,
    ) -> Self {
        Self {
            playback: PlaybackParams {
                search_year_utility: DateFilterParams {
                    channel: vec![0],
                    start_date: start_date.format("%Y%m%d").to_string(),
                    end_date: end_date.format("%Y%m%d").to_string(),
                    child_device_id,
                    child_device_mac,
                },
            },
        }
    }
}
