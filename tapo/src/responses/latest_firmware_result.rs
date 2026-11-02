use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the newest firmware that the device can update to, as the device reports it.
///
/// When no update is available, `type` is `0` and the other fields are empty or describe
/// the firmware the device is running.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct LatestFirmwareResult {
    /// A size the device reports with the update. It is not the size of the
    /// firmware image: every non-zero size in python-kasa's fixtures is `786432`.
    /// `0` when no update is available, and `None` on devices that do not send it.
    pub fw_size: Option<u64>,
    /// Version of the newest firmware. When no update is available, the version the
    /// device is running.
    pub fw_ver: String,
    /// Hardware ID that the update is for. Empty when no update is available.
    pub hw_id: String,
    /// `true` when an update is available. In python-kasa's device fixtures, it is
    /// `true` exactly when `type` is non-zero.
    pub need_to_upgrade: bool,
    /// OEM ID that the update is for. Empty when no update is available.
    pub oem_id: String,
    /// Release date of the newest firmware, as text. Empty when no update is available.
    pub release_date: String,
    /// Release notes of the newest firmware. Empty when no update is available.
    pub release_note: String,
    /// `0` when no update is available, and non-zero when one is. python-kasa's
    /// device fixtures hold `1`, `2` and `3`.
    pub r#type: u8,
}
impl TapoResponseExt for LatestFirmwareResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(LatestFirmwareResult);
