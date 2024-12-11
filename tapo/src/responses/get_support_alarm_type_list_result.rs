use serde::Deserialize;

use super::TapoResponseExt;

/// Contains a list of supported alarm types (ringtones) of the device.
#[derive(Debug, Deserialize)]
pub struct GetSupportAlarmTypeListResult {
    /// Available alarm types that can be passed to [PlayAlarmParams](`crate::requests::PlayAlarmParams`)
    pub alarm_type_list: Vec<String>,
}

impl TapoResponseExt for GetSupportAlarmTypeListResult {}
