use std::fmt;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::api::{ApiClient, Authenticated, Unauthenticated};
use crate::error::Error;
use crate::requests::TapoRequest;
use crate::responses::{
    ChildDeviceListResult, ChildDeviceResult, HubDeviceInfoResult, TapoResponseExt,
};

/// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) hubs.
pub struct HubHandler<S = Unauthenticated> {
    client: ApiClient,
    status: PhantomData<S>,
}

impl<S> HubHandler<S> {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client,
            status: PhantomData,
        }
    }

    /// Attempts to login. Each subsequent call will refresh the session.
    pub async fn login(mut self) -> Result<HubHandler<Authenticated>, Error> {
        self.client.login().await?;

        Ok(HubHandler {
            client: self.client,
            status: PhantomData,
        })
    }
}

impl HubHandler<Authenticated> {
    /// Gets *device info* as [`crate::responses::HubDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::HubHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<HubDeviceInfoResult, Error> {
        self.client.get_device_info().await
    }

    /// Gets *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Gets *child device list* as [`crate::responses::ChildDeviceListResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API
    /// or to support all the possible devices connected to the hub.
    pub async fn get_child_device_list(&self) -> Result<Vec<ChildDeviceResult>, Error> {
        self.client
            .get_child_device_list::<ChildDeviceListResult>()
            .await
            .map(|r| r.devices)
    }

    /// Gets *child device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_child_device_list_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_child_device_list().await
    }

    /// Gets *child device component list* as [`serde_json::Value`].
    /// This information is useful in debugging or when investigating new functionality to add.
    pub async fn get_child_device_component_list_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_child_device_component_list().await
    }

    /// Internal method that's called by functions of the child devices.
    pub(crate) async fn control_child<R>(
        &self,
        device_id: String,
        request_data: TapoRequest,
    ) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        self.client.control_child(device_id, request_data).await
    }
}
