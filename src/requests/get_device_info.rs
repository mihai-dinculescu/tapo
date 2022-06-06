use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct GetDeviceInfoParams;

impl GetDeviceInfoParams {
    pub fn new() -> Self {
        Self
    }
}
