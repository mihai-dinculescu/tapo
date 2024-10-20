use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the current power reading of the device.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct CurrentPowerResult {
    /// Current power in Watts (W).
    pub current_power: u64,
}
impl TapoResponseExt for CurrentPowerResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl CurrentPowerResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}
