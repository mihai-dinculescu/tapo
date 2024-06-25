use std::fmt;

use serde::de::DeserializeOwned;

use crate::api::ApiClient;
use crate::api::PlugPowerStripHandler;
use crate::error::Error;
use crate::requests::TapoRequest;
use crate::responses::{
    ChildDeviceListPowerStripResult, DeviceInfoPowerStripResult, PlugPowerStripResult,
    TapoResponseExt,
};

/// Handler for the [P300](https://www.tapo.com/en/search/?q=P300) devices.
pub struct PowerStripHandler {
    client: ApiClient,
}

impl PowerStripHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Refreshes the authentication session.
    pub async fn refresh_session(&mut self) -> Result<&mut Self, Error> {
        self.client.refresh_session().await?;
        Ok(self)
    }

    /// Returns *device info* as [`DeviceInfoPowerStripResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present,
    /// try [`PowerStripHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoPowerStripResult, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Returns *child device list* as [`PlugPowerStripResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API
    /// or to support all the possible devices connected to the hub.
    pub async fn get_child_device_list(&self) -> Result<Vec<PlugPowerStripResult>, Error> {
        self.client
            .get_child_device_list::<ChildDeviceListPowerStripResult>()
            .await
            .map(|r| r.sub_plugs)
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
impl PowerStripHandler {
    /// Returns a [`PlugPowerStripHandler`] for the given [`Plug`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a PowerStrip plug identifier.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, Plug};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let power_strip = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p300("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = power_strip.plug(Plug::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn plug(&self, identifier: Plug) -> Result<PlugPowerStripHandler, Error> {
        let children = self.get_child_device_list().await?;

        let device_id = match identifier {
            Plug::ByDeviceId(device_id) => children
                .iter()
                .find(|child| child.device_id == device_id)
                .ok_or_else(|| Error::DeviceNotFound)?
                .device_id
                .clone(),
            Plug::ByNickname(nickname) => children
                .iter()
                .find(|child| child.nickname == nickname)
                .ok_or_else(|| Error::DeviceNotFound)?
                .device_id
                .clone(),
            Plug::ByPosition(position) => children
                .iter()
                .find(|child| child.position == position)
                .ok_or_else(|| Error::DeviceNotFound)?
                .device_id
                .clone(),
        };

        Ok(PlugPowerStripHandler::new(self, device_id))
    }
}

/// Power strip plug.
pub enum Plug {
    ///  By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
    /// By Position.
    ByPosition(u8),
}
