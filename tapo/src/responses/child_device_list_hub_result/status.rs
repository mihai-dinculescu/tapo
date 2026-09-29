use serde::{Deserialize, Serialize};

/// Device status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, hash, frozen)
)]
#[allow(missing_docs)]
pub enum Status {
    Online,
    Offline,
}
