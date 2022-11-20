use crate::api::ApiClient;
use crate::devices::L510;
use crate::requests::L510SetDeviceInfoParams;
use crate::responses::L510DeviceInfoResult;

/// The functionality of [`crate::ApiClient<L510>`] that applies to [`crate::L510`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<L510> {
    /// Returns a [`crate::requests::L510SetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// `send` must be called at the end to apply the changes.
    ///
    /// # Example
    /// ```rust,no_run
    /// use tapo::{ApiClient, L510};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = ApiClient::<L510>::new(
    ///         "192.168.1.100".to_string(),
    ///         "tapo-username@example.com".to_string(),
    ///         "tapo-password".to_string(),
    ///         true,
    ///     ).await?;
    ///
    ///     device
    ///     .set()
    ///     .on()
    ///     .brightness(50)?
    ///     .send()
    ///     .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set(&self) -> L510SetDeviceInfoParams {
        L510SetDeviceInfoParams::new(self)
    }

    /// Sets the *brightness*.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> anyhow::Result<()> {
        L510SetDeviceInfoParams::new(self)
            .brightness(brightness)?
            .send()
            .await
    }

    /// Gets *device info* as [`crate::responses::L510DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<L510DeviceInfoResult> {
        self.get_device_info_internal::<L510DeviceInfoResult>()
            .await
    }
}
