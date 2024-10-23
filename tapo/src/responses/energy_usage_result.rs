use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::utils::der_tapo_datetime_format;

/// Contains local time, current power and the energy usage and runtime for today and for the current month.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct EnergyUsageResult {
    /// Local time of the device.
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    /// Current power in Milliwatts (mW).
    pub current_power: u64,
    /// Today runtime in minutes.
    pub today_runtime: u64,
    /// Today energy usage in Watt Hours (Wh).
    pub today_energy: u64,
    /// Current month runtime in minutes.
    pub month_runtime: u64,
    /// Current month energy usage in Watt Hours (Wh).
    pub month_energy: u64,
}
impl TapoResponseExt for EnergyUsageResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl EnergyUsageResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}
