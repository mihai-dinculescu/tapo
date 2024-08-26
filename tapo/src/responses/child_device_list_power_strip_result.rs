use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::device_info_result::OverheatStatus;
use crate::responses::{decode_value, DecodableResultExt, TapoResponseExt};

/// Power Strip child device list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChildDeviceListPowerStripResult {
    /// Power Strip child devices
    #[serde(rename = "child_device_list")]
    pub sub_plugs: Vec<PlugPowerStripResult>,
}

impl DecodableResultExt for ChildDeviceListPowerStripResult {
    fn decode(self) -> Result<Self, Error> {
        Ok(ChildDeviceListPowerStripResult {
            sub_plugs: self
                .sub_plugs
                .into_iter()
                .map(|d| d.decode())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TapoResponseExt for ChildDeviceListPowerStripResult {}

/// P300 power strip child plug.
///
/// Specific properties: `auto_off_remain_time`, `auto_off_status`,
/// `bind_count`, `overheat_status`, `position`, `slot_number`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PlugPowerStripResult {
    pub auto_off_remain_time: u64,
    pub auto_off_status: AutoOffStatus,
    pub avatar: String,
    pub bind_count: u8,
    pub category: String,
    pub device_id: String,
    pub device_on: bool,
    pub fw_id: String,
    pub fw_ver: String,
    pub has_set_location_info: bool,
    pub hw_id: String,
    pub hw_ver: String,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub mac: String,
    pub model: String,
    pub nickname: String,
    pub oem_id: String,
    /// The time in seconds this device has been ON since the last state change (ON/OFF).
    pub on_time: u64,
    pub original_device_id: String,
    pub overheat_status: OverheatStatus,
    pub position: u8,
    pub region: Option<String>,
    pub slot_number: u8,
    pub status_follow_edge: bool,
    pub r#type: String,
}

impl TapoResponseExt for PlugPowerStripResult {}

impl DecodableResultExt for PlugPowerStripResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// Auto Off Status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum AutoOffStatus {
    On,
    Off,
}
