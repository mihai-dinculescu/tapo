use serde::{Deserialize, Serialize};

/// The default state of a device to be used when internet connectivity is lost after a power cut.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[allow(missing_docs)]
pub enum DefaultState<T> {
    Custom(T),
    LastStates(T),
}
