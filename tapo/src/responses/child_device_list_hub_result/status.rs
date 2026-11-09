use serde::{Deserialize, Serialize};

/// Connection status of a hub child device, as reported by the hub.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, hash, frozen)
)]
pub enum Status {
    /// The hub considers the child device connected.
    ///
    /// The hub sets this, not the device, and is slow to change it. A battery-powered
    /// child that stops reporting, for example with its battery removed, can stay
    /// `Online` for many hours, and the hub keeps returning its last reported state in
    /// the meantime. `Online` does not mean the device is reachable right now.
    Online,
    /// The hub has marked the child device as disconnected.
    Offline,
}
