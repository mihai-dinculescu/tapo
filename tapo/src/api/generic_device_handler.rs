use crate::api::{ApiClient, ApiClientExt};
use crate::error::Error;
use crate::requests::GenericSetDeviceInfoParams;
use crate::responses::DeviceInfoGenericResult;

/// Handler for generic devices. It provides the functionality common to all Tapo [devices](https://www.tapo.com/en/).
pub struct GenericDeviceHandler {
    client: ApiClient,
}

impl GenericDeviceHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Refreshes the authentication session.
    pub async fn refresh_session(&mut self) -> Result<&mut Self, Error> {
        self.client.refresh_session().await?;
        Ok(self)
    }

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

    /// Returns *device info* as [`DeviceInfoGenericResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`GenericDeviceHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoGenericResult, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }
}
