use serde::{Deserialize, Serialize};

/// The default state of a device to be used when internet connectivity is lost after a power cut.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[allow(missing_docs)]
pub enum DefaultState<T> {
    Custom(T),
    LastStates(T),
}

/// The type of the default state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub enum DefaultStateType {
    Custom,
    LastStates,
}
