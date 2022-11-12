use crate::api::ApiClient;
use crate::devices::P100;
use crate::responses::PlugDeviceInfoResult;

/// The functionality of [`crate::ApiClient<P100>`] that applies to [`crate::P100`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<P100> {
    /// Gets *device info* as [`crate::responses::PlugDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<PlugDeviceInfoResult> {
        self.get_device_info_internal::<PlugDeviceInfoResult>()
            .await
    }
}
