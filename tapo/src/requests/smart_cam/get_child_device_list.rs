use serde::Serialize;

use crate::requests::GetChildDeviceListParams;

#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetChildDeviceListParams {
    #[serde(rename = "childControl")]
    child_control: GetChildDeviceListParams,
}

impl SmartCamGetChildDeviceListParams {
    pub fn new(start_index: u64) -> Self {
        Self {
            child_control: GetChildDeviceListParams::new(start_index),
        }
    }
}
