use crate::api::ApiClient;
use crate::requests::L530SetDeviceInfoParams;
use crate::responses::L530DeviceInfoResult;
use crate::{Color, L530};

/// The functionality of [`crate::ApiClient<L530>`] that applies to [`crate::L530`]. Superset of [`crate::ApiClient<D>`].
impl ApiClient<L530> {
    /// Sets the light bulb's *color*.
    ///
    /// # Arguments
    ///
    /// * `color` - [crate::Color]
    pub async fn set_color(&self, color: Color) -> anyhow::Result<()> {
        let json = serde_json::to_value(&L530SetDeviceInfoParams::color(color)?)?;
        self.set_device_info_internal(json).await
    }

    /// Sets the light bulb's *brightness*.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> anyhow::Result<()> {
        let json = serde_json::to_value(&L530SetDeviceInfoParams::brightness(brightness)?)?;
        self.set_device_info_internal(json).await
    }

    /// Sets the light bulb's *hue* and *saturation*.
    ///
    /// # Arguments
    ///
    /// * `hue` - *u16* between 1 and 360
    /// * `saturation` - *u8*; between 1 and 100
    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> anyhow::Result<()> {
        let json =
            serde_json::to_value(&L530SetDeviceInfoParams::hue_saturation(hue, saturation)?)?;
        self.set_device_info_internal(json).await
    }

    /// Sets the light bulb's *color temperature*.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - *u16*; between 2500 and 6500
    pub async fn set_color_temperature(&self, color_temperature: u16) -> anyhow::Result<()> {
        let json = serde_json::to_value(&L530SetDeviceInfoParams::color_temperature(
            color_temperature,
        )?)?;
        self.set_device_info_internal(json).await
    }

    /// Gets *device info* as [`crate::L530DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ApiClient::get_device_info_json`].
    pub async fn get_device_info(&self) -> anyhow::Result<L530DeviceInfoResult> {
        self.get_device_info_internal::<L530DeviceInfoResult>()
            .await
    }
}
