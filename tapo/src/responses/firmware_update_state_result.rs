use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the progress of a firmware update, as the device reports it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct FirmwareUpdateStateResult {
    /// Whether the device reports the update as an automatic one. `None` on devices
    /// that do not report it.
    pub auto_upgrade: Option<bool>,
    /// Download progress in percent (0-100).
    pub download_progress: u8,
    /// Time the device takes to reboot after flashing, in seconds.
    pub reboot_time: u64,
    /// State of the update. python-kasa reads `0` as idle or finished, `2` as
    /// downloading, `3` as flashing, and a negative value as a failed update. Other
    /// values occur: a T110 in python-kasa's fixtures reports `4`.
    pub status: i64,
    /// Time the device takes to flash the firmware, in seconds.
    pub upgrade_time: u64,
}
impl TapoResponseExt for FirmwareUpdateStateResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(FirmwareUpdateStateResult);
