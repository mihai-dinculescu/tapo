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
crate::impl_to_dict!(ChildDeviceComponentList);
