use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// T100 motion sensor.
///
/// Specific properties: `detected`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct T100Result {
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    pub detected: bool,
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

impl TapoResponseExt for T100Result {}

impl DecodableResultExt for T100Result {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// T100 Log.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
#[allow(missing_docs)]
pub enum T100Log {
    Motion { id: u64, timestamp: u64 },
}
