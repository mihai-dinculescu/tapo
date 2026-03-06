use tokio::sync::RwLockReadGuard;

use crate::error::Error;
use crate::requests::LightSetDeviceInfoParams;
use crate::responses::{DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult};

use super::{ApiClient, ApiClientExt};

tapo_handler! {
    /// Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
    /// [L520](https://www.tapo.com/en/search/?q=L520) and
    /// [L610](https://www.tapo.com/en/search/?q=L610) devices.
    LightHandler(DeviceInfoLightResult),
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl LightHandler {
    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let client = RwLockReadGuard::map(
            self.client.read().await,
            |client: &ApiClient| -> &dyn ApiClientExt { client },
        );

        LightSetDeviceInfoParams::new(client).on().send().await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let client = RwLockReadGuard::map(
            self.client.read().await,
            |client: &ApiClient| -> &dyn ApiClientExt { client },
        );

        LightSetDeviceInfoParams::new(client).off().send().await
    }

    /// Sets the *brightness* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        let client = RwLockReadGuard::map(
            self.client.read().await,
            |client: &ApiClient| -> &dyn ApiClientExt { client },
        );

        LightSetDeviceInfoParams::new(client)
            .brightness(brightness)
            .send()
            .await
    }
}
