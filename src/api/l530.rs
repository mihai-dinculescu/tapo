use crate::api::ApiClient;
use crate::devices::L530;
use crate::requests::{Color, L530SetDeviceInfoParams};
use crate::responses::L530DeviceInfoResult;

/// The functionality of [`crate::ApiClient<L530>`] that applies to [`crate::L530`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<L530> {
    /// Returns a [`crate::requests::L530SetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// `send` must be called at the end to apply the changes.
    ///
    /// # Example
    /// ```rust,no_run
    /// use tapo::{ApiClient, L530};
    /// use tapo::requests::Color;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = ApiClient::<L530>::new(
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
    ///     .color(Color::HotPink)?
    ///     .send()
    ///     .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set(&self) -> L530SetDeviceInfoParams {
        L530SetDeviceInfoParams::new(self)
    }

    /// Sets the *brightness*.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> anyhow::Result<()> {
        L530SetDeviceInfoParams::new(self)
            .brightness(brightness)?
            .send()
            .await
    }

    /// Sets the *color*.
    ///
    /// # Arguments
    ///
    /// * `color` - [crate::requests::Color]
    pub async fn set_color(&self, color: Color) -> anyhow::Result<()> {
        L530SetDeviceInfoParams::new(self)
            .color(color)?
            .send()
            .await
    }

    /// Sets the *hue* and *saturation*.
    ///
    /// # Arguments
    ///
    /// * `hue` - *u16* between 1 and 360
    /// * `saturation` - *u8*; between 1 and 100
    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> anyhow::Result<()> {
        L530SetDeviceInfoParams::new(self)
            .hue_saturation(hue, saturation)?
            .send()
            .await
    }

    /// Sets the *color temperature*.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - *u16*; between 2500 and 6500
    pub async fn set_color_temperature(&self, color_temperature: u16) -> anyhow::Result<()> {
        L530SetDeviceInfoParams::new(self)
            .color_temperature(color_temperature)?
            .send()
            .await
    }

    /// Gets *device info* as [`crate::responses::L530DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<L530DeviceInfoResult> {
        self.get_device_info_internal::<L530DeviceInfoResult>()
            .await
    }
}
