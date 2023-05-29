use serde::{Deserialize, Serialize};

use crate::api::HubHandler;
use crate::error::Error;
use crate::requests::{EmptyParams, TapoParams, TapoRequest};
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// Temperature unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

/// T310/T315 temperature & humidity sensor.
///
/// Specific properties: `current_humidity`, `current_temperature`, `temperature_unit`, `current_humidity_exception`, `current_temperature_exception`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct T31XResult {
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    /// This value will be 0 when the current humidity is within the comfort zone.
    /// When the current humidity value falls outside the comfort zone, this value
    /// will be the difference between the current humidity and the lower or upper bound of the comfort zone.
    pub current_humidity_exception: i8,
    pub current_humidity: u8,
    /// This value will be 0 when the current temperature is within the comfort zone.
    /// When the current temperature value falls outside the comfort zone, this value
    /// will be the difference between the current temperature and the lower or upper bound of the comfort zone.
    #[serde(rename = "current_temp_exception")]
    pub current_temperature_exception: f32,
    #[serde(rename = "current_temp")]
    pub current_temperature: f32,
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
    #[serde(rename = "temp_unit")]
    pub temperature_unit: TemperatureUnit,
    pub r#type: String,
}

impl TapoResponseExt for T31XResult {}

impl DecodableResultExt for Box<T31XResult> {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

impl T31XResult {
    /// Gets *device info* as [`crate::responses::T31XResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self, handler: &HubHandler) -> Result<T31XResult, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        handler.control_child(self.device_id.clone(), request).await
    }
}
