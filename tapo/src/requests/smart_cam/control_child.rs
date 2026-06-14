use serde::Serialize;

use crate::requests::TapoRequest;

#[derive(Debug, Serialize)]
pub(crate) struct SmartCamControlChildParams {
    #[serde(rename = "childControl")]
    child_control: SmartCamChildControl,
}

impl SmartCamControlChildParams {
    pub fn new(device_id: String, request_data: TapoRequest) -> Self {
        Self {
            child_control: SmartCamChildControl {
                device_id,
                request_data,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct SmartCamChildControl {
    device_id: String,
    request_data: TapoRequest,
}
