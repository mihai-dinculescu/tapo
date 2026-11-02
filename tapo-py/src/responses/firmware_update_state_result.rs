use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::FirmwareUpdateStateResult;

// PyO3 cannot expose a Rust enum that mixes unit variants with data-carrying
// ones, so this plain enum mirrors `FirmwareUpdateStateResult`; the variants'
// data is exposed on `PyFirmwareUpdateStateResult` instead.
/// The status of a firmware update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[pyclass(name = "FirmwareUpdateStatus", from_py_object, eq, hash, frozen)]
#[allow(missing_docs)]
pub enum PyFirmwareUpdateStatus {
    Idle,
    Downloading,
    Downloaded,
    AwaitingTransfer,
    Failed,
    Unknown,
}

/// The state of a firmware update on the device.
///
/// Flattens the Rust enum so that Python checks `status` instead of the
/// variant. The fields that belong to other statuses are `None`.
#[derive(Debug, Clone, Serialize)]
#[pyclass(name = "FirmwareUpdateStateResult", from_py_object, get_all)]
pub struct PyFirmwareUpdateStateResult {
    status: PyFirmwareUpdateStatus,
    download_progress: Option<u8>,
    install_time_s: Option<u16>,
    reboot_time_s: Option<u16>,
    code: Option<i32>,
}

impl PyFirmwareUpdateStateResult {
    fn new(status: PyFirmwareUpdateStatus) -> Self {
        Self {
            status,
            download_progress: None,
            install_time_s: None,
            reboot_time_s: None,
            code: None,
        }
    }
}

impl From<FirmwareUpdateStateResult> for PyFirmwareUpdateStateResult {
    fn from(result: FirmwareUpdateStateResult) -> Self {
        match result {
            FirmwareUpdateStateResult::Idle => Self::new(PyFirmwareUpdateStatus::Idle),
            FirmwareUpdateStateResult::Downloading { progress } => Self {
                download_progress: Some(progress),
                ..Self::new(PyFirmwareUpdateStatus::Downloading)
            },
            FirmwareUpdateStateResult::Downloaded {
                install_time_s,
                reboot_time_s,
            } => Self {
                install_time_s: Some(install_time_s),
                reboot_time_s: Some(reboot_time_s),
                ..Self::new(PyFirmwareUpdateStatus::Downloaded)
            },
            FirmwareUpdateStateResult::AwaitingTransfer => {
                Self::new(PyFirmwareUpdateStatus::AwaitingTransfer)
            }
            FirmwareUpdateStateResult::Failed { code } => Self {
                code: Some(code),
                ..Self::new(PyFirmwareUpdateStatus::Failed)
            },
            FirmwareUpdateStateResult::Unknown { code } => Self {
                code: Some(code),
                ..Self::new(PyFirmwareUpdateStatus::Unknown)
            },
        }
    }
}

tapo::impl_to_dict!(PyFirmwareUpdateStateResult);
