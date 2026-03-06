use crate::error::Error;
use crate::requests::{AlarmDuration, AlarmRingtone, AlarmVolume, PlayAlarmParams};
#[cfg(feature = "debug")]
use crate::responses::ChildDeviceComponentList;
use crate::responses::{ChildDeviceHubResult, ChildDeviceListHubResult, DeviceInfoHubResult};

use super::{KE100Handler, S200Handler, T31XHandler, T100Handler, T110Handler, T300Handler};

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

tapo_handler! {
    /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.
    HubHandler(DeviceInfoHubResult),
    device_management,
}

/// Hub handler methods.
impl HubHandler {
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
    #[cfg(feature = "debug")]
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

    /// Returns *child device component list* as [`Vec<ChildDeviceComponentList>`].
    /// This information is useful in debugging or when investigating new functionality to add.
    #[cfg(feature = "debug")]
    pub async fn get_child_device_component_list(
        &self,
    ) -> Result<Vec<ChildDeviceComponentList>, Error> {
        self.client
            .read()
            .await
            .get_child_device_component_list()
            .await
    }

    /// Returns a list of ringtones (alarm types) supported by the hub.
    /// Used for debugging only.
    #[cfg(feature = "debug")]
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

    /// Returns a [`S200Handler`] for the given [`HubDevice`].
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
    /// let device = hub.s200(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn s200(&self, identifier: HubDevice) -> Result<S200Handler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::S200);
        Ok(S200Handler::new(self.client.clone(), device_id))
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
    /// let device = hub.t31x(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn t31x(&self, identifier: HubDevice) -> Result<T31XHandler, Error> {
        let device_id = get_device_id!(self, identifier, ChildDeviceHubResult::T31X);
        Ok(T31XHandler::new(self.client.clone(), device_id))
    }
}

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
