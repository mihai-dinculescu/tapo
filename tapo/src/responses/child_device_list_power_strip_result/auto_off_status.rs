use serde::{Deserialize, Serialize};

/// Auto Off Status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, eq_int)
)]
#[allow(missing_docs)]
pub enum AutoOffStatus {
    On,
    Off,
}
