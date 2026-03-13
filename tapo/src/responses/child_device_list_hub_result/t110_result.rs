use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, Status, TapoResponseExt, decode_value};

/// Device info of Tapo T110 contact sensor.
///
/// Specific properties: `open`, `report_interval`,
/// `last_onboarding_timestamp`,`status_follow_edge`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct T110Result {
    // Common properties to all Hub child devices.
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    pub device_id: String,
    pub fw_ver: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub jamming_rssi: i16,
    pub jamming_signal_level: u8,
    pub mac: String,
    pub model: String,
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub region: String,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status: Status,
    pub r#type: String,
    // Specific properties to this device.
    #[serde(rename = "lastOnboardingTimestamp")]
    pub last_onboarding_timestamp: u64,
    pub open: bool,
    /// The time in seconds between each report.
    pub report_interval: u32,
    pub status_follow_edge: bool,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(T110Result);

impl TapoResponseExt for T110Result {}

impl DecodableResultExt for T110Result {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// T110 Log.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub enum T110Log {
    Close {
        id: u64,
        timestamp: u64,
    },
    Open {
        id: u64,
        timestamp: u64,
    },
    /// Fired when the sensor has been open for more than 1 minute.
    KeepOpen {
        id: u64,
        timestamp: u64,
    },
}

#[cfg(feature = "python")]
crate::impl_to_dict!(T110Log);
