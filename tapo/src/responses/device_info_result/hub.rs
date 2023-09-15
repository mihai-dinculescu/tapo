use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, TapoResponseExt};

/// Device info of Tapo H100. Superset of [`crate::responses::DeviceInfoGenericResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DeviceInfoHubResult {
    //
    // Inherited from DeviceInfoGenericResult
    //
    pub device_id: String,
    pub r#type: String,
    pub model: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub fw_id: String,
    pub fw_ver: String,
    pub oem_id: String,
    pub mac: String,
    pub ip: String,
    pub ssid: String,
    pub signal_level: u8,
    pub rssi: i16,
    pub specs: String,
    pub lang: String,
    pub overheated: bool,
    pub nickname: String,
    pub avatar: String,
    pub has_set_location_info: bool,
    pub region: Option<String>,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub time_diff: Option<i64>,
    //
    // Unique to this device
    //
    pub in_alarm: bool,
    pub in_alarm_source: String,
}

impl TapoResponseExt for DeviceInfoHubResult {}

impl DecodableResultExt for DeviceInfoHubResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}
