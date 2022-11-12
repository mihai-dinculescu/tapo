use crate::api::ApiClient;
use crate::devices::GenericDevice;
use crate::responses::GenericDeviceInfoResult;

/// The functionality of [`crate::ApiClient<GenericDevice>`] that applies to [`crate::GenericDevice`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<GenericDevice> {
    /// Gets *device info* as [`crate::responses::GenericDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<GenericDeviceInfoResult> {
        self.get_device_info_internal::<GenericDeviceInfoResult>()
            .await
    }
}
