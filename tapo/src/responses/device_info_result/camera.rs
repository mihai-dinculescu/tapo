use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, TapoResponseExt};
use crate::utils::{bool_from_int_or_bool, option_bool_from_int_or_bool};

/// Device info of Tapo cameras (C100, C110, C210, C220, C225, C325WB, C520WS, C720, TC40, TC65, TC70, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoCameraResult {
    pub avatar: String,
    #[serde(alias = "dev_id")]
    pub device_id: String,
    pub device_info: String,
    pub device_name: String,
    #[serde(alias = "sw_version")]
    pub fw_ver: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub has_set_location_info: bool,
    pub hw_id: String,
    #[serde(alias = "hw_version")]
    pub hw_ver: String,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub mac: String,
    #[serde(alias = "device_model")]
    pub model: String,
    #[serde(alias = "device_alias")]
    pub nickname: String,
    /// Whether RTSP streaming is available without restrictions.
    /// Only present on some models (e.g. C220, C225, C720).
    #[serde(default, deserialize_with = "option_bool_from_int_or_bool")]
    pub no_rtsp_constrain: Option<bool>,
    pub region: Option<String>,
    #[serde(alias = "device_type")]
    pub r#type: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoCameraResult);

impl TapoResponseExt for DeviceInfoCameraResult {}

impl DecodableResultExt for DeviceInfoCameraResult {
    fn decode(self) -> Result<Self, Error> {
        // SmartCam devices don't base64-encode fields.
        Ok(self)
    }
}
