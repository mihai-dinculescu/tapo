use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt, TemperatureUnit};


/// KE100 TRV.
///
/// Specific properties: `detected`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct KE100Result {
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
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub region: String,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status: Status,
    pub r#type: String,
    #[serde(rename = "temp_unit")]
    pub temperature_unit: TemperatureUnit,
    #[serde(rename = "current_temp")]
    pub current_temperature: f32,
    #[serde(rename = "target_temp")]
    pub target_temperature: f32,
    #[serde(rename = "min_control_temp")]
    pub min_control_temperature: u8,
    #[serde(rename = "max_control_temp")]
    pub max_control_temperature: u8,
    pub frost_protection_on: bool,
    pub location: String,
    #[serde(rename = "temp_offset")]
    pub temperature_offset: i8,
    pub child_protection: bool,
}

impl TapoResponseExt for KE100Result {}

impl DecodableResultExt for Box<KE100Result> {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}
