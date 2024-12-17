use serde::Deserialize;

use super::TapoResponseExt;

/// Contains a list of supported alarm types (ringtones) of the device.
/// Useful for debugging only.
#[derive(Debug, Deserialize)]
pub struct SupportedAlarmTypeListResult {
    /// Available alarm types supported by the play_alarm request
    pub alarm_type_list: Vec<String>,
}

impl TapoResponseExt for SupportedAlarmTypeListResult {}
