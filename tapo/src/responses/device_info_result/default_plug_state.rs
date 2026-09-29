use serde::{Deserialize, Serialize};

/// Plug Default State.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, hash, frozen)
)]
#[allow(missing_docs)]
pub enum DefaultPlugState {
    Custom { state: PlugState },
    LastStates {},
}

/// Plug State.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, hash, frozen)
)]
#[allow(missing_docs)]
pub struct PlugState {
    pub on: bool,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DefaultPlugState);

#[cfg(feature = "python")]
crate::impl_to_dict!(PlugState);
