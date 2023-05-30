use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// S200B button switch.
///
/// Specific properties: none.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct S200BResult {
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
    #[serde(rename = "lastOnboardingTimestamp")]
    pub last_onboarding_timestamp: u64,
    pub mac: String,
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub region: String,
    /// The time in seconds between each report.
    pub report_interval: u32,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status_follow_edge: bool,
    pub status: Status,
    pub r#type: String,
}

impl TapoResponseExt for S200BResult {}

impl DecodableResultExt for Box<S200BResult> {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// S200B Rotation log params.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct S200BRotationParams {
    #[serde(rename = "rotate_deg")]
    pub degrees: i16,
}

/// S200B Log.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
#[allow(missing_docs)]
pub enum S200BLog {
    Rotation {
        id: u64,
        timestamp: u64,
        params: S200BRotationParams,
    },
    SingleClick {
        id: u64,
        timestamp: u64,
    },
    DoubleClick {
        id: u64,
        timestamp: u64,
    },
}
