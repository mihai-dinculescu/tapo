use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, DefaultPlugState, TapoResponseExt};

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
/// Specific properties: `detected`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PlugPowerStripResult {
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
    pub device_on: bool,
    /// The time in seconds this device has been ON since the last state change (ON/OFF).
    pub on_time: u64,
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
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultPlugState,
}

impl TapoResponseExt for PlugPowerStripResult {}

impl DecodableResultExt for PlugPowerStripResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}
