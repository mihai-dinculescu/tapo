use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{Component, DecodableResultExt, TapoResponseExt};

/// Child device component list result.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChildDeviceComponentListResult {
    pub child_component_list: Vec<ChildDeviceComponentList>,
}

impl DecodableResultExt for ChildDeviceComponentListResult {
    fn decode(self) -> Result<Self, Error> {
        Ok(self)
    }
}

impl TapoResponseExt for ChildDeviceComponentListResult {}

/// A single child device's component (feature/capability) list.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct ChildDeviceComponentList {
    /// The device ID of the child device.
    pub device_id: String,
    /// The list of components supported by this child device.
    pub component_list: Vec<Component>,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl ChildDeviceComponentList {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}
