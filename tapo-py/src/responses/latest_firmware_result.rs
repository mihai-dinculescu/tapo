use chrono::NaiveDate;
use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::LatestFirmwareResult;

/// The newest firmware that the device can update to.
///
/// Flattens the Rust enum so that Python checks `update_available` instead of
/// the variant. `release_date` and `release_notes` are `None` when no update
/// is available.
#[derive(Debug, Clone, Serialize)]
#[pyclass(name = "LatestFirmwareResult", from_py_object, get_all)]
pub struct PyLatestFirmwareResult {
    update_available: bool,
    fw_ver: String,
    release_date: Option<NaiveDate>,
    release_notes: Option<String>,
}

impl From<LatestFirmwareResult> for PyLatestFirmwareResult {
    fn from(result: LatestFirmwareResult) -> Self {
        match result {
            LatestFirmwareResult::UpToDate { fw_ver } => Self {
                update_available: false,
                fw_ver,
                release_date: None,
                release_notes: None,
            },
            LatestFirmwareResult::UpdateAvailable {
                fw_ver,
                release_date,
                release_notes,
            } => Self {
                update_available: true,
                fw_ver,
                release_date: Some(release_date),
                release_notes: Some(release_notes),
            },
        }
    }
}

tapo::impl_to_dict!(PyLatestFirmwareResult);
