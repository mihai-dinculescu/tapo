use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::responses::TapoResponseExt;
use crate::tapo_date_format;

/// Contains local time, current power and the energy usage and runtime for today and for the current month.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyUsageResult {
    /// Local time, with the UTC offset assumed from the machine this call is made on
    #[serde(with = "tapo_date_format")]
    pub local_time: OffsetDateTime,
    /// Current power in milliwatts (mW)
    pub current_power: u64,
    /// Today runtime in minutes
    pub today_runtime: u64,
    /// Today energy usage in watts (W)
    pub today_energy: u64,
    /// Current month runtime in minutes
    pub month_runtime: u64,
    /// Current month energy usage in watts (W)
    pub month_energy: u64,
    /// Hourly energy usage for the past 24 hours in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::EnergyMonitoringPlugHandler::get_energy_data` instead."
    )]
    pub past24h: Option<Vec<u64>>,
    /// Hourly energy usage by day for the past 7 days in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::EnergyMonitoringPlugHandler::get_energy_data` instead."
    )]
    pub past7d: Option<Vec<Vec<u64>>>,
    /// Daily energy usage for the past 30 days in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::EnergyMonitoringPlugHandler::get_energy_data` instead."
    )]
    pub past30d: Option<Vec<u64>>,
    /// Monthly energy usage for the past year in watts (W)
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::EnergyMonitoringPlugHandler::get_energy_data` instead."
    )]
    pub past1y: Option<Vec<u64>>,
}
impl TapoResponseExt for EnergyUsageResult {}
