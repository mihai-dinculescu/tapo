use serde::{Deserialize, Serialize};

/// The type of the default state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
pub enum DefaultStateType {
    Custom,
    LastStates,
}

/// The type of the default power state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
pub enum DefaultPowerType {
    AlwaysOn,
    LastStates,
}

/// Default brightness state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct DefaultBrightnessState {
    pub r#type: DefaultStateType,
    pub value: u8,
}
