use crate::api::{ApiClient, ApiClientExt};
use crate::error::Error;
use crate::requests::GenericSetDeviceInfoParams;
use crate::responses::DeviceInfoGenericResult;

use super::{
    ColorLightHandler, HubHandler, LightHandler, PlugEnergyMonitoringHandler, PlugHandler,
    PowerStripEnergyMonitoringHandler, PowerStripHandler, RgbLightStripHandler,
    RgbicLightStripHandler,
};

/// Handler for generic devices. It provides the functionality common to all Tapo [devices](https://www.tapo.com/en/).
///
/// If you'd like to propose support for a device that isn't currently supported,
/// please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues) to start the conversation.
#[derive(Debug)]
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

impl From<GenericDeviceHandler> for LightHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        LightHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for ColorLightHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        ColorLightHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for RgbLightStripHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        RgbLightStripHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for RgbicLightStripHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        RgbicLightStripHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for PlugHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        PlugHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for PlugEnergyMonitoringHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        PlugEnergyMonitoringHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for PowerStripHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        PowerStripHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for PowerStripEnergyMonitoringHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        PowerStripEnergyMonitoringHandler::new(value.client)
    }
}

impl From<GenericDeviceHandler> for HubHandler {
    fn from(value: GenericDeviceHandler) -> Self {
        HubHandler::new(value.client)
    }
}
