use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ComponentListResult {
    pub component_list: Vec<Component>,
}

impl TapoResponseExt for ComponentListResult {}

/// A component (feature/capability) reported by a Tapo device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct Component {
    /// The component identifier (e.g. `"energy_monitoring"`, `"countdown"`).
    pub id: String,
    /// The version code of the component.
    pub ver_code: u8,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(Component);
