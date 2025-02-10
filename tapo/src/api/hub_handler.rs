use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::ApiClient;
use crate::api::{KE100Handler, S200BHandler, T100Handler, T110Handler, T300Handler, T31XHandler};
use crate::error::Error;
use crate::requests::{AlarmDuration, AlarmRingtone, AlarmVolume, PlayAlarmParams};
use crate::responses::{ChildDeviceHubResult, ChildDeviceListHubResult, DeviceInfoHubResult};

macro_rules! get_device_id {
    ($self:expr, $identifier:expr, $($value:path),+) => {{
        let children = $self.get_child_device_list().await?;

        match $identifier {
            HubDevice::ByDeviceId(device_id) => children
                .iter()
                .filter_map(|d| match d {
                    $($value(child) if child.device_id == device_id => Some(child.device_id.clone()),)+
                    _ => None,
                })
                .next()
                .ok_or_else(|| Error::DeviceNotFound)?,
            HubDevice::ByNickname(nickname) => children
                .iter()
                .filter_map(|d| match d {
                    $($value(child) if child.nickname == nickname => Some(child.device_id.clone()),)+
                    _ => None,
                })
                .next()
                .ok_or_else(|| Error::DeviceNotFound)?,
        }
    }};
}

/// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) hubs.
pub struct HubHandler {
    client: Arc<RwLock<ApiClient>>,
}

/// Hub handler methods.
impl HubHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client: Arc::new(RwLock::new(client)),
        }
    }

    /// Refreshes the authentication session.
    pub async fn refresh_session(&mut self) -> Result<&mut Self, Error> {
        self.client.write().await.refresh_session().await?;
        Ok(self)
    }

    /// Returns *device info* as [`DeviceInfoHubResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`HubHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoHubResult, Error> {
        self.client.read().await.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.read().await.get_device_info().await
    }

    /// Returns *child device list* as [`ChildDeviceHubResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API
    /// or to support all the possible devices connected to the hub.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`HubHandler::get_child_device_list_json`].
    pub async fn get_child_device_list(&self) -> Result<Vec<ChildDeviceHubResult>, Error> {
        let mut results = Vec::new();
        let mut start_index = 0;
        let mut fetch = true;

        while fetch {
            let devices = self
                .client
                .read()
                .await
                .get_child_device_list::<ChildDeviceListHubResult>(start_index)
                .await
                .map(|r| r.devices)?;

            fetch = devices.len() == 10;
            start_index += 10;
            results.extend(devices);
        }

        Ok(results)
    }

    /// Returns *child device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    ///
    /// # Arguments
    ///
    /// * `start_index` - the index to start fetching the child device list.
    ///   It should be `0` for the first page, `10` for the second, and so on.
    pub async fn get_child_device_list_json(
        &self,
        start_index: u64,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .read()
            .await
            .get_child_device_list(start_index)
            .await
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

    /// Returns a list of ringtones (alarm types) supported by the hub.
    /// Used for debugging only.
    pub async fn get_supported_ringtone_list(&self) -> Result<Vec<String>, Error> {
        self.client
            .read()
            .await
            .get_supported_alarm_type_list()
            .await
            .map(|response| response.alarm_type_list)
    }

    /// Start playing the hub alarm.
    pub async fn play_alarm(
        &self,
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: AlarmDuration,
    ) -> Result<(), Error> {
        self.client
            .read()
            .await
            .play_alarm(PlayAlarmParams::new(ringtone, volume, duration)?)
            .await
    }

    /// Stop playing the hub alarm, if it's currently playing.
    pub async fn stop_alarm(&self) -> Result<(), Error> {
        self.client.read().await.stop_alarm().await
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
        Ok(KE100Handler::new(self.client.clone(), device_id))
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
        Ok(S200BHandler::new(self.client.clone(), device_id))
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
        Ok(T100Handler::new(self.client.clone(), device_id))
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
        Ok(T110Handler::new(self.client.clone(), device_id))
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
        Ok(T300Handler::new(self.client.clone(), device_id))
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
        let device_id = get_device_id!(
            self,
            identifier,
            ChildDeviceHubResult::T310,
            ChildDeviceHubResult::T315
        );
        Ok(T31XHandler::new(self.client.clone(), device_id))
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
        self.t310(identifier).await
    }
}

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
