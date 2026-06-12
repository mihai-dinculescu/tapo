use serde::Serialize;

use crate::requests::GetChildDeviceListParams;

#[derive(Debug, Serialize)]
pub(crate) struct ChildControlListParams {
    #[serde(rename = "childControl")]
    child_control: GetChildDeviceListParams,
}

impl ChildControlListParams {
    pub fn new(start_index: u64) -> Self {
        Self {
            child_control: GetChildDeviceListParams::new(start_index),
        }
    }
}
