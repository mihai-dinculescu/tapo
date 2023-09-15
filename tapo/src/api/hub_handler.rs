use std::fmt;

use serde::de::DeserializeOwned;

use crate::api::ApiClient;
use crate::api::{S200BHandler, T100Handler, T110Handler, T31XHandler};
use crate::error::Error;
use crate::requests::TapoRequest;
use crate::responses::{
    ChildDeviceListResult, ChildDeviceResult, DeviceInfoHubResult, TapoResponseExt,
};

/// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) hubs.
pub struct HubHandler {
    client: ApiClient,
}

/// Hub handler methods.
impl HubHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Refreshes the authentication session.
    pub async fn refresh_session(&mut self) -> Result<&mut Self, Error> {
        self.client.refresh_session().await?;
        Ok(self)
    }

    /// Returns *device info* as [`DeviceInfoHubResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`HubHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoHubResult, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Returns *child device list* as [`ChildDeviceListResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API
    /// or to support all the possible devices connected to the hub.
    pub async fn get_child_device_list(&self) -> Result<Vec<ChildDeviceResult>, Error> {
        self.client
            .get_child_device_list::<ChildDeviceListResult>()
            .await
            .map(|r| r.devices)
    }

    /// Returns *child device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_child_device_list_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_child_device_list().await
    }

    /// Returns *child device component list* as [`serde_json::Value`].
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

/// Child device handler builders.
impl HubHandler {
    /// Returns a [`S200BHandler`] for the given `device_id`.
    ///
    /// # Arguments
    ///
    /// * `device_id` - the Device ID of the child device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device = hub.s200b("0000000000000000000000000000000000000000");
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn s200b(&self, device_id: impl Into<String>) -> S200BHandler {
        S200BHandler::new(self, device_id.into())
    }

    /// Returns a [`T31XHandler`] for the given `device_id`.
    ///
    /// # Arguments
    ///
    /// * `device_id` - the Device ID of the child device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device = hub.t310("0000000000000000000000000000000000000000");
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn t310(&self, device_id: impl Into<String>) -> T31XHandler {
        T31XHandler::new(self, device_id.into())
    }

    /// Returns a [`T31XHandler`] for the given `device_id`.
    ///
    /// # Arguments
    ///
    /// * `device_id` - the Device ID of the child device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device = hub.t315("0000000000000000000000000000000000000000");
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn t315(&self, device_id: impl Into<String>) -> T31XHandler {
        T31XHandler::new(self, device_id.into())
    }

    /// Returns a [`T100Handler`] for the given `device_id`.
    ///
    /// # Arguments
    ///
    /// * `device_id` - the Device ID of the child device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device = hub.t100("0000000000000000000000000000000000000000");
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn t100(&self, device_id: impl Into<String>) -> T100Handler {
        T100Handler::new(self, device_id.into())
    }

    /// Returns a [`T110Handler`] for the given `device_id`.
    ///
    /// # Arguments
    ///
    /// * `device_id` - the Device ID of the child device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device = hub.t110("0000000000000000000000000000000000000000");
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn t110(&self, device_id: impl Into<String>) -> T110Handler {
        T110Handler::new(self, device_id.into())
    }
}
