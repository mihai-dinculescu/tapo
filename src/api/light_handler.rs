use std::marker::PhantomData;

use crate::api::{ApiClient, ApiClientExt, Authenticated, Unauthenticated};
use crate::error::Error;
use crate::requests::{GenericSetDeviceInfoParams, LightSetDeviceInfoParams};
use crate::responses::{DeviceUsageResult, L510DeviceInfoResult};

/// Handler for the [L510](https://www.tapo.com/en/search/?q=L510) and [L610](https://www.tapo.com/en/search/?q=L610) devices.
pub struct LightHandler<S = Unauthenticated> {
    client: ApiClient,
    status: PhantomData<S>,
}

impl<S> LightHandler<S> {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client,
            status: PhantomData,
        }
    }

    /// Attempts to login. Each subsequent call will refresh the session.
    pub async fn login(mut self) -> Result<LightHandler<Authenticated>, Error> {
        self.client.login().await?;

        Ok(LightHandler {
            client: self.client,
            status: PhantomData,
        })
    }
}

impl LightHandler<Authenticated> {
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

    /// Gets *device info* as [`crate::responses::L510DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::LightHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<L510DeviceInfoResult, Error> {
        self.client.get_device_info().await
    }

    /// Gets *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Gets *device usage* as [`crate::responses::DeviceUsageResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        self.client.get_device_usage().await
    }

    /// Returns a [`crate::requests::LightSetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// `send` must be called at the end to apply the changes.
    ///
    /// # Example
    /// ```rust,no_run
    /// use tapo::ApiClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = ApiClient::new(
    ///         "192.168.1.100",
    ///         "tapo-username@example.com",
    ///         "tapo-password",
    ///     )?
    ///     .l510()
    ///     .login()
    ///     .await?;
    ///
    ///     device
    ///     .set()
    ///     .on()
    ///     .brightness(50)
    ///     .send()
    ///     .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set(&self) -> LightSetDeviceInfoParams {
        LightSetDeviceInfoParams::new(&self.client)
    }

    /// Sets the *brightness* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        LightSetDeviceInfoParams::new(&self.client)
            .brightness(brightness)
            .send()
            .await
    }
}
