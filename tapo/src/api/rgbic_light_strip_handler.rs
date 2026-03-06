use crate::error::Error;
use crate::requests::{Color, ColorLightSetDeviceInfoParams, LightingEffect, SegmentEffect};
use crate::responses::{DeviceInfoRgbicLightStripResult, DeviceUsageEnergyMonitoringResult};

tapo_handler! {
    /// Handler for the [L920](https://www.tapo.com/en/search/?q=L920) and
    /// [L930](https://www.tapo.com/en/search/?q=L930) devices.
    RgbicLightStripHandler(DeviceInfoRgbicLightStripResult),
    on_off,
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl RgbicLightStripHandler {
    /// Returns a [`ColorLightSetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    /// For *lighting effects*, use [`RgbicLightStripHandler::set_lighting_effect`] instead.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # use tapo::requests::Color;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    /// #     .l930("192.168.1.100")
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
    /// Pre-existing *lighting effect* will be removed.
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
    /// Pre-existing *lighting effect* will be removed.
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
    /// Pre-existing *lighting effect* will be removed.
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
    /// Pre-existing *lighting effect* will be removed.
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

    /// Sets a *lighting effect* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `lighting_effect` - [crate::requests::LightingEffectPreset] or a custom [crate::requests::LightingEffect].
    pub async fn set_lighting_effect(
        &self,
        lighting_effect: impl Into<LightingEffect>,
    ) -> Result<(), Error> {
        self.client
            .read()
            .await
            .set_lighting_effect(lighting_effect.into())
            .await
    }

    /// Sets a *segment effect* and turns *on* the device.
    ///
    /// This is used for the newer app-defined RGBIC strip effects that cannot be set by
    /// [`LightingEffect`] (for example, "circulating" or "breathe" segment effects).
    ///
    /// # Arguments
    ///
    /// * `segment_effect` - a [`crate::requests::SegmentEffectPreset`] or a custom [`SegmentEffect`]
    pub async fn set_segment_effect(
        &self,
        segment_effect: impl Into<SegmentEffect>,
    ) -> Result<(), Error> {
        let segment_effect = segment_effect.into();
        segment_effect.validate()?;

        self.client
            .read()
            .await
            .set_segment_effect(segment_effect)
            .await
    }
}
