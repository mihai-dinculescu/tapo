use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetDeviceUsageParams;

impl GetDeviceUsageParams {
    pub fn new() -> Self {
        Self
    }
}
