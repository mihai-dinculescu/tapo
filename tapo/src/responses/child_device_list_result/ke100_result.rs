use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// Temperature unit for KE100 devices.
/// Currently *Celsius* is the only unit supported by KE100.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum TemperatureUnitKE100 {
    Celsius,
}

/// KE100 thermostatic radiator valve (TRV).
///
/// Specific properties: `temperature_unit`, `current_temperature`, `target_temperature`,
/// `min_control_temperature, `max_control_temperature`, `temperature_offset`,
/// `child_protection_on`, `frost_protection_on`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct KE100Result {
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    #[serde(rename = "child_protection")]
    pub child_protection_on: bool,
    #[serde(rename = "current_temp")]
    pub current_temperature: f32,
    pub device_id: String,
    pub frost_protection_on: bool,
    pub fw_ver: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub jamming_rssi: i16,
    pub jamming_signal_level: u8,
    pub location: String,
    pub mac: String,
    #[serde(rename = "max_control_temp")]
    pub max_control_temperature: u8,
    #[serde(rename = "min_control_temp")]
    pub min_control_temperature: u8,
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub r#type: String,
    pub region: String,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status: Status,
    #[serde(rename = "target_temp")]
    pub target_temperature: f32,
    #[serde(rename = "temp_offset")]
    pub temperature_offset: i8,
    #[serde(rename = "temp_unit")]
    pub temperature_unit: TemperatureUnitKE100,
}

impl TapoResponseExt for KE100Result {}

impl DecodableResultExt for KE100Result {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}
