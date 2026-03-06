use crate::error::Error;
use crate::requests::{Color, ColorLightSetDeviceInfoParams};
use crate::responses::{DeviceInfoRgbLightStripResult, DeviceUsageEnergyMonitoringResult};

tapo_handler! {
    /// Handler for the [L900](https://www.tapo.com/en/search/?q=L900) devices.
    RgbLightStripHandler(DeviceInfoRgbLightStripResult),
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl RgbLightStripHandler {
    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new().on().send(self).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new().off().send(self).await
    }

    /// Returns a [`ColorLightSetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # use tapo::requests::Color;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    /// #     .l900("192.168.1.100")
    /// #     .await?;
    /// device
    ///     .set()
    ///     .brightness(50)
    ///     .color(Color::HotPink)
    ///     .send(&device)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set(&self) -> ColorLightSetDeviceInfoParams {
        ColorLightSetDeviceInfoParams::new()
    }

    /// Sets the *brightness* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .brightness(brightness)
            .send(self)
            .await
    }

    /// Sets the *color* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `color` - one of [crate::requests::Color] as defined in the Google Home app
    pub async fn set_color(&self, color: Color) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .color(color)
            .send(self)
            .await
    }

    /// Sets the *hue*, *saturation* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `hue` - between 0 and 360
    /// * `saturation` - between 1 and 100
    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .hue_saturation(hue, saturation)
            .send(self)
            .await
    }

    /// Sets the *color temperature* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - between 2500 and 6500
    pub async fn set_color_temperature(&self, color_temperature: u16) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .color_temperature(color_temperature)
            .send(self)
            .await
    }
}
