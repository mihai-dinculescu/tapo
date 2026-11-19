use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::CloudConnectionStateResult;

// PyO3 cannot expose a Rust enum that mixes unit variants with data-carrying
// ones, so this plain enum mirrors `CloudConnectionStateResult`; the `Unknown`
// variant's code is exposed on `PyCloudConnectionStateResult` instead.
/// The status of the device's connection to the Tapo cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[pyclass(name = "CloudConnectionStatus", from_py_object, eq, hash, frozen)]
#[allow(missing_docs)]
pub enum PyCloudConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Disconnecting,
    Unknown,
}

/// The state of the device's connection to the Tapo cloud.
///
/// Flattens the Rust enum so that Python checks `status` instead of the
/// variant.
#[derive(Debug, Clone, Serialize)]
#[pyclass(name = "CloudConnectionStateResult", from_py_object, get_all)]
pub struct PyCloudConnectionStateResult {
    status: PyCloudConnectionStatus,
    code: Option<i32>,
}

impl From<CloudConnectionStateResult> for PyCloudConnectionStateResult {
    fn from(result: CloudConnectionStateResult) -> Self {
        let (status, code) = match result {
            CloudConnectionStateResult::Connected => (PyCloudConnectionStatus::Connected, None),
            CloudConnectionStateResult::Disconnected => {
                (PyCloudConnectionStatus::Disconnected, None)
            }
            CloudConnectionStateResult::Connecting => (PyCloudConnectionStatus::Connecting, None),
            CloudConnectionStateResult::Disconnecting => {
                (PyCloudConnectionStatus::Disconnecting, None)
            }
            CloudConnectionStateResult::Unknown { code } => {
                (PyCloudConnectionStatus::Unknown, Some(code))
            }
        };

        Self { status, code }
    }
}

tapo::impl_to_dict!(PyCloudConnectionStateResult);
