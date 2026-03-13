use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, TapoParams, TapoRequest};
use crate::responses::{DeviceUsageResult, S210Result};

tapo_child_handler! {
    /// Handler for the [S210](https://www.tapo.com/en/search/?q=S210) devices.
    S210Handler(S210Result),
    on_off,
}

impl S210Handler {
    /// Returns *device usage* as [`DeviceUsageResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        let request = TapoRequest::GetDeviceUsage(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }
}
