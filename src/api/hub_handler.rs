use std::fmt;

use serde::de::DeserializeOwned;

use crate::api::ApiClient;
use crate::error::Error;
use crate::requests::TapoRequest;
use crate::responses::{
    ChildDeviceListResult, ChildDeviceResult, HubDeviceInfoResult, TapoResponseExt,
};

/// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) hubs.
pub struct HubHandler {
    client: ApiClient,
}

impl HubHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Attempts to refresh the authentication session.
    pub async fn login(mut self) -> Result<Self, Error> {
        let session = self.client.get_session_ref()?;
        self.client.login(session.url.clone()).await?;

        Ok(self)
    }

    /// Gets *device info* as [`HubDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`HubHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<HubDeviceInfoResult, Error> {
        self.client.get_device_info().await
    }

    /// Gets *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Gets *child device list* as [`ChildDeviceListResult`].
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
