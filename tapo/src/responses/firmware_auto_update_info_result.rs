use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the device's automatic firmware update settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct FirmwareAutoUpdateInfoResult {
    /// Whether the device installs firmware updates automatically.
    #[serde(rename(deserialize = "enable"))]
    pub enabled: bool,
    /// Start of the update window, in minutes after midnight in the device's timezone
    /// (`120` is 2:00 AM).
    #[serde(rename(deserialize = "time"))]
    pub start_min: u16,
    /// Length of the update window, in minutes. The Tapo app always uses `120`.
    #[serde(rename(deserialize = "random_range"))]
    pub window_min: u16,
}
impl TapoResponseExt for FirmwareAutoUpdateInfoResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(FirmwareAutoUpdateInfoResult);
