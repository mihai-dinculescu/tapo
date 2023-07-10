use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::tapo_date_format::der_tapo_datetime_format;

/// Energy data for the requested [`crate::requests::EnergyDataInterval`].
#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyDataResult {
    /// Local time of the device
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    /// Energy data for the given `interval` in watts (W)
    pub data: Vec<u64>,
    /// Interval start timestamp in milliseconds
    pub start_timestamp: u64,
    /// Interval end timestamp in milliseconds
    pub end_timestamp: u64,
    /// Interval in minutes
    pub interval: u64,
}
impl TapoResponseExt for EnergyDataResult {}
