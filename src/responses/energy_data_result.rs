use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::responses::TapoResponseExt;
use crate::tapo_date_format;

/// Energy data for the requested [`crate::requests::EnergyDataInterval`].
#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyDataResult {
    /// Local time, with the UTC offset assumed from the machine this call is made on
    #[serde(with = "tapo_date_format")]
    pub local_time: OffsetDateTime,
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
