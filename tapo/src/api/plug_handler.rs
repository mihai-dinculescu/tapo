use crate::error::Error;
use crate::requests::GenericSetDeviceInfoParams;
use crate::responses::{DeviceInfoPlugResult, DeviceUsageResult};

use super::ApiClientExt;

tapo_handler! {
    /// Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
    /// [P105](https://www.tapo.com/en/search/?q=P105) devices.
    PlugHandler(DeviceInfoPlugResult),
    device_usage = DeviceUsageResult,
    device_management,
}

impl PlugHandler {
    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        self.client.read().await.set_device_info(json).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        self.client.read().await.set_device_info(json).await
    }
}
