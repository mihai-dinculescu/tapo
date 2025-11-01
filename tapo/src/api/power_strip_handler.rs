use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::error::Error;
use crate::responses::{
    ChildDeviceListPowerStripResult, DeviceInfoPowerStripResult, PowerStripPlugResult,
};

use super::{
    ApiClient, ApiClientExt, DeviceManagementExt, HandlerExt, Plug, PowerStripPlugHandler,
};

/// Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
/// [P306](https://www.tp-link.com/us/search/?q=P306) devices.
#[derive(Debug)]
pub struct PowerStripHandler {
    client: Arc<RwLock<ApiClient>>,
}

impl PowerStripHandler {
    pub(crate) fn new(client: Arc<RwLock<ApiClient>>) -> Self {
        Self { client }
    }

    /// Refreshes the authentication session.
    pub async fn refresh_session(&mut self) -> Result<&mut Self, Error> {
        self.client.write().await.refresh_session().await?;
        Ok(self)
    }

    /// Returns *device info* as [`DeviceInfoPowerStripResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present,
    /// try [`PowerStripHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoPowerStripResult, Error> {
        self.client.read().await.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.read().await.get_device_info().await
    }

    /// Returns *child device list* as [`Vec<PowerStripPlugResult>`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present,
    /// try [`PowerStripHandler::get_child_device_list_json`].
    pub async fn get_child_device_list(&self) -> Result<Vec<PowerStripPlugResult>, Error> {
        self.client
            .read()
            .await
            .get_child_device_list::<ChildDeviceListPowerStripResult>(0)
            .await
            .map(|r| r.plugs)
    }

    /// Returns *child device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_child_device_list_json(&self) -> Result<serde_json::Value, Error> {
        self.client.read().await.get_child_device_list(0).await
    }

    /// Returns *child device component list* as [`serde_json::Value`].
    /// This information is useful in debugging or when investigating new functionality to add.
    pub async fn get_child_device_component_list_json(&self) -> Result<serde_json::Value, Error> {
        self.client
            .read()
            .await
            .get_child_device_component_list()
            .await
    }
}

/// Child device handler builders.
impl PowerStripHandler {
    /// Returns a [`PowerStripPlugHandler`] for the given [`Plug`].
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
    pub async fn plug(&self, identifier: Plug) -> Result<PowerStripPlugHandler, Error> {
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

        Ok(PowerStripPlugHandler::new(self.client.clone(), device_id))
    }
}

#[async_trait]
impl HandlerExt for PowerStripHandler {
    async fn get_client(&self) -> RwLockReadGuard<'_, dyn ApiClientExt> {
        RwLockReadGuard::map(
            self.client.read().await,
            |client: &ApiClient| -> &dyn ApiClientExt { client },
        )
    }
}

impl DeviceManagementExt for PowerStripHandler {}
