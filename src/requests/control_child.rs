use serde::Serialize;

use crate::requests::tapo_request::TapoRequest;

#[derive(Debug, Serialize)]
pub(crate) struct ControlChildParams {
    device_id: String,
    #[serde(rename = "requestData")]
    request_data: TapoRequest,
}

impl ControlChildParams {
    pub fn new(device_id: String, request_data: TapoRequest) -> Self {
        Self {
            device_id,
            request_data,
        }
    }
}
