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
    on_off,
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl LightHandler {
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
