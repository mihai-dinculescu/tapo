use serde::{Deserialize, Serialize};

use super::{TapoResponseExt, UsageByPeriodResult};

/// Contains the time usage, the power consumption, and the energy savings of the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct DeviceUsageEnergyMonitoringResult {
    /// Time usage in minutes.
    pub time_usage: UsageByPeriodResult,
    /// Power usage in Watt Hours (Wh).
    pub power_usage: UsageByPeriodResult,
    /// Saved power in Watt Hours (Wh).
    pub saved_power: UsageByPeriodResult,
}
impl TapoResponseExt for DeviceUsageEnergyMonitoringResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DeviceUsageEnergyMonitoringResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}
