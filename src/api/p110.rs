use crate::api::ApiClient;
use crate::devices::P110;
use crate::responses::{DeviceUsageResult, EnergyUsageResult, PlugDeviceInfoResult};

/// The functionality of [`crate::ApiClient<P110>`] that applies to [`crate::P110`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<P110> {
    /// Gets *device info* as [`crate::PlugDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<PlugDeviceInfoResult> {
        self.get_device_info_internal::<PlugDeviceInfoResult>()
            .await
    }

    /// Gets *device usage*. It contains the time in use, the power consumption, and the energy savings of the device.
    pub async fn get_device_usage(&self) -> anyhow::Result<DeviceUsageResult> {
        self.get_device_usage_internal().await
    }

    /// Gets *energy usage*. It contains local time, current power and the energy usage and runtime over multiple periods of time.
    pub async fn get_energy_usage(&self) -> anyhow::Result<EnergyUsageResult> {
        self.get_energy_usage_internal().await
    }
}
