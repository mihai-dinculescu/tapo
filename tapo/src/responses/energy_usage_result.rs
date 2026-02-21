use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::utils::der_tapo_datetime_format;

/// Contains local time, current power and the energy usage and runtime for today and for the current month.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct EnergyUsageResult {
    /// Current power in milliwatts (mW).
    pub current_power: Option<u64>,
    /// Electricity charge/cost data reported by the device using the tariff configured in the Tapo app.
    /// The third element is the total charge for the current month.
    /// The meaning of the first two elements is not confirmed; please open an issue or pull request if you know.
    pub electricity_charge: Option<[u64; 3]>,
    /// Local time of the device.
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    /// Current month energy usage in Watt Hours (Wh).
    pub month_energy: u64,
    /// Current month runtime in minutes.
    pub month_runtime: u64,
    /// Today energy usage in Watt Hours (Wh).
    pub today_energy: u64,
    /// Today runtime in minutes.
    pub today_runtime: u64,
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
