use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the time in use, the power consumption, and the energy savings of the device.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceUsageResult {
    /// Time usage in minutes
    pub time_usage: UsageByPeriodResult,
    /// Power usage in watt-hour (Wh)
    pub power_usage: UsageByPeriodResult,
    /// Saved power in watt-hour (Wh)
    pub saved_power: UsageByPeriodResult,
}
impl TapoResponseExt for DeviceUsageResult {}

/// Usage by period result for today, the past 7 days, and the past 30 days.
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageByPeriodResult {
    /// Today
    pub today: u64,
    /// Past 7 days
    pub past7: u64,
    /// Past 30 days
    pub past30: u64,
}
