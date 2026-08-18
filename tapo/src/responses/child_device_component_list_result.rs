use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{Component, DecodableResultExt, TapoResponseExt};

/// Child device component list result.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChildDeviceComponentListResult {
    /// H200 firmware 1.6.5 omits the field entirely when the list is empty.
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_child_component_list_parses_as_empty() {
        // H200 firmware 1.6.1 response when no sensors are attached.
        let json = r#"{"child_component_list":[],"start_index":0,"sum":0}"#;

        let parsed: ChildDeviceComponentListResult = serde_json::from_str(json).unwrap();

        assert!(parsed.child_component_list.is_empty());
    }

    #[test]
    fn test_missing_child_component_list_parses_as_empty() {
        // H200 firmware 1.6.5 response when no sensors are attached.
        let json = r#"{"start_index":0,"sum":0}"#;

        let parsed: ChildDeviceComponentListResult = serde_json::from_str(json).unwrap();

        assert!(parsed.child_component_list.is_empty());
    }
}
