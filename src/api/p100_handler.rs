use std::marker::PhantomData;

use crate::api::{ApiClient, ApiClientExt, Authenticated, Unauthenticated};
use crate::error::Error;
use crate::requests::GenericSetDeviceInfoParams;
use crate::responses::{DeviceUsageResult, PlugDeviceInfoResult};

/// Handler for the [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices.
pub struct P100Handler<S = Unauthenticated> {
    client: ApiClient,
    status: PhantomData<S>,
}

impl<S> P100Handler<S> {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client,
            status: PhantomData,
        }
    }

    /// Attempts to login. Each subsequent call will refresh the session.
    pub async fn login(mut self) -> Result<P100Handler<Authenticated>, Error> {
        self.client.login().await?;

        Ok(P100Handler {
            client: self.client,
            status: PhantomData,
        })
    }
}

impl P100Handler<Authenticated> {
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

    /// Gets *device info* as [`crate::responses::PlugDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::P100Handler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<PlugDeviceInfoResult, Error> {
        self.client.get_device_info::<PlugDeviceInfoResult>().await
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
