use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the time usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct DeviceUsageResult {
    /// Time usage in minutes.
    pub time_usage: UsageByPeriodResult,
}
impl TapoResponseExt for DeviceUsageResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DeviceUsageResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
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
