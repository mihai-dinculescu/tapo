use std::marker::PhantomData;

use crate::api::{ApiClient, Authenticated, Unauthenticated};
use crate::error::Error;
use crate::requests::GenericSetDeviceInfoParams;
use crate::responses::{DeviceUsageResult, GenericDeviceInfoResult};

/// Handler for generic devices. It provides the functionality common to all Tapo [devices](https://www.tapo.com/en/).
pub struct GenericDeviceHandler<S = Unauthenticated> {
    client: ApiClient,
    status: PhantomData<S>,
}

impl<S> GenericDeviceHandler<S> {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client,
            status: PhantomData,
        }
    }

    /// Attempts to login. Each subsequent call will refresh the session.
    pub async fn login(mut self) -> Result<GenericDeviceHandler<Authenticated>, Error> {
        self.client.login().await?;

        Ok(GenericDeviceHandler {
            client: self.client,
            status: PhantomData,
        })
    }
}

impl GenericDeviceHandler<Authenticated> {
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

    /// Gets *device info* as [`crate::responses::GenericDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::GenericDeviceHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<GenericDeviceInfoResult, Error> {
        self.client
            .get_device_info::<GenericDeviceInfoResult>()
            .await
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
}
