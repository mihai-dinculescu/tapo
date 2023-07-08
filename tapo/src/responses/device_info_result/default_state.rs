use serde::{Deserialize, Serialize};

/// The type of the default state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub enum DefaultStateType {
    Custom,
    LastStates,
}
