use std::fmt;

use serde::de::DeserializeOwned;

use crate::api::ApiClient;
use crate::api::{KE100Handler, S200BHandler, T100Handler, T110Handler, T300Handler, T31XHandler};
use crate::error::Error;
use crate::requests::TapoRequest;
use crate::responses::{
    ChildDeviceHubResult, ChildDeviceListHubResult, DeviceInfoHubResult, TapoResponseExt,
};

macro_rules! get_device_id {
    ($self:expr, $identifier:expr, $value:path) => {{
        let children = $self.get_child_device_list().await?;

        match $identifier {
            HubDevice::ByDeviceId(device_id) => children
                .iter()
                .filter_map(|d| match d {
                    $value(child) if child.device_id == device_id => Some(child.device_id.clone()),
                    _ => None,
                })
                .next()
                .ok_or_else(|| Error::DeviceNotFound)?,
            HubDevice::ByNickname(nickname) => children
                .iter()
                .filter_map(|d| match d {
                    $value(child) if child.nickname == nickname => Some(child.device_id.clone()),
                    _ => None,
                })
                .next()
                .ok_or_else(|| Error::DeviceNotFound)?,
        }
    }};
}

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

    /// Returns *child device list* as [`ChildDeviceHubResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API
    /// or to support all the possible devices connected to the hub.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`HubHandler::get_child_device_list_json`].
    pub async fn get_child_device_list(&self) -> Result<Vec<ChildDeviceHubResult>, Error> {
        self.client
            .get_child_device_list::<ChildDeviceListHubResult>()
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
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        self.client.control_child(device_id, request_data).await
    }
}

/// Child device handler builders.
impl HubHandler {
    /// Returns a [`KE100Handler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.ke100(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ke100(&self, identifier: HubDevice) -> Result<KE100Handler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::KE100);
        Ok(KE100Handler::new(self, device_id))
    }

    /// Returns a [`S200BHandler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.s200b(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn s200b(&self, identifier: HubDevice) -> Result<S200BHandler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::S200B);
        Ok(S200BHandler::new(self, device_id))
    }

    /// Returns a [`T100Handler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t100(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn t100(&self, identifier: HubDevice) -> Result<T100Handler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::T100);
        Ok(T100Handler::new(self, device_id))
    }

    /// Returns a [`T110Handler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t110(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn t110(&self, identifier: HubDevice) -> Result<T110Handler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::T110);
        Ok(T110Handler::new(self, device_id))
    }

    /// Returns a [`T300Handler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t300(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn t300(&self, identifier: HubDevice) -> Result<T300Handler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::T300);
        Ok(T300Handler::new(self, device_id))
    }

    /// Returns a [`T31XHandler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t310(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn t310(&self, identifier: HubDevice) -> Result<T31XHandler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::T310);
        Ok(T31XHandler::new(self, device_id))
    }

    /// Returns a [`T31XHandler`] for the given [`HubDevice`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t315(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn t315(&self, identifier: HubDevice) -> Result<T31XHandler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::T315);
        Ok(T31XHandler::new(self, device_id))
    }
}

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
