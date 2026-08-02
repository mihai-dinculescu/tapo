use serde::{Deserialize, Serialize};

/// The power state of a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, eq_int)
)]
pub enum PowerState {
    /// The device is on.
    On,
    /// The device is off.
    Off,
}
