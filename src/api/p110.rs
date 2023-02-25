use crate::api::ApiClient;
use crate::devices::P110;
use crate::requests::EnergyDataInterval;
use crate::responses::{EnergyDataResult, EnergyUsageResult, PlugDeviceInfoResult};

/// The functionality of [`crate::ApiClient<P110>`] that applies to [`crate::P110`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<P110> {
    /// Gets *device info* as [`crate::responses::PlugDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<PlugDeviceInfoResult> {
        self.get_device_info_internal::<PlugDeviceInfoResult>()
            .await
    }

    /// Gets *energy usage*. It returns local time, current power and the energy usage and runtime for the current day and past month.
    pub async fn get_energy_usage(&self) -> anyhow::Result<EnergyUsageResult> {
        self.get_energy_usage_internal().await
    }

    /// Gets *energy data*. It returns local time and energy data for the requested `interval`.
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> anyhow::Result<EnergyDataResult> {
        self.get_energy_data_internal(interval).await
    }
}
