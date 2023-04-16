use std::marker::PhantomData;

use crate::api::{ApiClient, Authenticated, Unauthenticated};
use crate::error::Error;
use crate::requests::{Color, GenericSetDeviceInfoParams, L530SetDeviceInfoParams};
use crate::responses::{DeviceUsageResult, L530DeviceInfoResult};

/// Handler for the [L530](https://www.tapo.com/en/search/?q=L530) devices.
pub struct L530Handler<S = Unauthenticated> {
    client: ApiClient,
    status: PhantomData<S>,
}

impl<S> L530Handler<S> {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client,
            status: PhantomData,
        }
    }

    /// Attempts to login. Each subsequent call will refresh the session.
    pub async fn login(mut self) -> Result<L530Handler<Authenticated>, Error> {
        self.client.login().await?;

        Ok(L530Handler {
            client: self.client,
            status: PhantomData,
        })
    }
}

impl L530Handler<Authenticated> {
    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        self.client.set_device_info(json).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        self.client.set_device_info(json).await
    }

    /// Gets *device info* as [`crate::responses::L530DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::L530Handler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<L530DeviceInfoResult, Error> {
        self.client.get_device_info::<L530DeviceInfoResult>().await
    }

    /// Gets *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info_json().await
    }

    /// Gets *device usage* as [`crate::responses::DeviceUsageResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        self.client.get_device_usage().await
    }

    /// Returns a [`crate::requests::L530SetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// `send` must be called at the end to apply the changes.
    ///
    /// # Example
    /// ```rust,no_run
    /// use tapo::ApiClient;
    /// use tapo::requests::Color;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = ApiClient::new(
    ///         "192.168.1.100",
    ///         "tapo-username@example.com",
    ///         "tapo-password",
    ///     )?
    ///     .l530()
    ///     .login()
    ///     .await?;
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
        L530SetDeviceInfoParams::new(&self.client)
    }

    /// Sets the *brightness*.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        L530SetDeviceInfoParams::new(&self.client)
            .brightness(brightness)?
            .send()
            .await
    }

    /// Sets the *color*.
    ///
    /// # Arguments
    ///
    /// * `color` - [crate::requests::Color]
    pub async fn set_color(&self, color: Color) -> Result<(), Error> {
        L530SetDeviceInfoParams::new(&self.client)
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
    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> Result<(), Error> {
        L530SetDeviceInfoParams::new(&self.client)
            .hue_saturation(hue, saturation)?
            .send()
            .await
    }

    /// Sets the *color temperature*.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - *u16*; between 2500 and 6500
    pub async fn set_color_temperature(&self, color_temperature: u16) -> Result<(), Error> {
        L530SetDeviceInfoParams::new(&self.client)
            .color_temperature(color_temperature)?
            .send()
            .await
    }
}
