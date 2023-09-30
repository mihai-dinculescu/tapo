use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::tapo_date_format::der_tapo_datetime_format;

/// Contains local time, current power and the energy usage and runtime for today and for the current month.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct EnergyUsageResult {
    /// Local time of the device.
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    /// Current power in milliwatts (mW).
    pub current_power: u64,
    /// Today runtime in minutes.
    pub today_runtime: u64,
    /// Today energy usage in watts (W).
    pub today_energy: u64,
    /// Current month runtime in minutes.
    pub month_runtime: u64,
    /// Current month energy usage in watts (W).
    pub month_energy: u64,
    /// Hourly energy usage for the past 24 hours in watts (W).
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::PlugEnergyMonitoringHandler::get_energy_data` instead."
    )]
    #[cfg(not(feature = "python"))]
    pub past24h: Option<Vec<u64>>,
    /// Hourly energy usage by day for the past 7 days in watts (W).
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::PlugEnergyMonitoringHandler::get_energy_data` instead."
    )]
    #[cfg(not(feature = "python"))]
    pub past7d: Option<Vec<Vec<u64>>>,
    /// Daily energy usage for the past 30 days in watts (W).
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::PlugEnergyMonitoringHandler::get_energy_data` instead."
    )]
    #[cfg(not(feature = "python"))]
    pub past30d: Option<Vec<u64>>,
    /// Monthly energy usage for the past year in watts (W).
    #[deprecated(
        since = "0.4.0",
        note = "P110 firmware v1.1.6 no longer returns this field. Use `tapo::PlugEnergyMonitoringHandler::get_energy_data` instead."
    )]
    #[cfg(not(feature = "python"))]
    pub past1y: Option<Vec<u64>>,
}
impl TapoResponseExt for EnergyUsageResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl EnergyUsageResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict<'a>(&self, py: pyo3::Python<'a>) -> pyo3::PyResult<&'a pyo3::types::PyDict> {
        let serialized = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        if let Some(object) = serialized.as_object() {
            let dict = crate::python::serde_object_to_py_dict(py, object)?;

            Ok(dict)
        } else {
            Ok(pyo3::types::PyDict::new(py))
        }
    }
}
