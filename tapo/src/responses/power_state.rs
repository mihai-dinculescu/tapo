use serde::{Deserialize, Serialize};

/// The state a plug transitions to when a timer or schedule rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, eq_int)
)]
pub enum PowerState {
    /// The plug turns on when the timer / rule fires.
    On,
    /// The plug turns off when the timer / rule fires.
    Off,
}
