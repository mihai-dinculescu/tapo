use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, Status, TapoResponseExt, decode_value};

/// Device info of Tapo S200B and S200D button switches.
///
/// Specific properties: `report_interval`, `last_onboarding_timestamp`, `status_follow_edge`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct S200Result {
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
    /// The time in seconds between each report.
    pub report_interval: u32,
    pub status_follow_edge: bool,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(S200Result);

impl TapoResponseExt for S200Result {}

impl DecodableResultExt for S200Result {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// S200B and S200D Rotation log params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct S200RotationParams {
    #[serde(rename = "rotate_deg")]
    pub rotation_degrees: i16,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(S200RotationParams);

/// S200B and S200D Log.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub enum S200Log {
    Rotation {
        id: u64,
        timestamp: u64,
        params: S200RotationParams,
    },
    SingleClick {
        id: u64,
        timestamp: u64,
    },
    DoubleClick {
        id: u64,
        timestamp: u64,
    },
    LowBattery {
        id: u64,
        timestamp: u64,
    },
}

#[cfg(feature = "python")]
crate::impl_to_dict!(S200Log);
