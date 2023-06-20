use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the time in use, the power consumption, and the energy savings of the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct DeviceUsageResult {
    /// Time usage in minutes.
    pub time_usage: UsageByPeriodResult,
    /// Power usage in watt-hour (Wh).
    pub power_usage: UsageByPeriodResult,
    /// Saved power in watt-hour (Wh).
    pub saved_power: UsageByPeriodResult,
}
impl TapoResponseExt for DeviceUsageResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DeviceUsageResult {
    /// Get all the properties of this result as a dictionary.
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

/// Usage by period result for today, the past 7 days, and the past 30 days.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct UsageByPeriodResult {
    /// Today.
    pub today: u64,
    /// Past 7 days.
    pub past7: u64,
    /// Past 30 days.
    pub past30: u64,
}
