use crate::api::ApiClient;
use crate::devices::L510;
use crate::requests::L510SetDeviceInfoParams;
use crate::responses::{DeviceUsageResult, L510DeviceInfoResult};

/// The functionality of [`crate::ApiClient<L510>`] that applies to [`crate::L510`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<L510> {
    /// Sets the light bulb's *brightness*.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> anyhow::Result<()> {
        let json = serde_json::to_value(&L510SetDeviceInfoParams::brightness(brightness)?)?;
        self.set_device_info_internal(json).await
    }

    /// Gets *device info* as [`crate::L510DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<L510DeviceInfoResult> {
        self.get_device_info_internal::<L510DeviceInfoResult>()
            .await
    }

    /// Gets *device usage*. It contains the time in use, the power consumption, and the energy savings of the device.
    pub async fn get_device_usage(&self) -> anyhow::Result<DeviceUsageResult> {
        self.get_device_usage_internal().await
    }
}
