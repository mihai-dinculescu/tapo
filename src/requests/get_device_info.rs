use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetDeviceInfoParams;

impl GetDeviceInfoParams {
    pub fn new() -> Self {
        Self
    }
}
