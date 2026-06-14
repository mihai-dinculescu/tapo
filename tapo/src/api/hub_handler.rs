use crate::error::Error;
use crate::requests::{AlarmDuration, AlarmRingtone, AlarmVolume, PlayAlarmParams};
use crate::responses::{ChildDeviceHubResult, ChildDeviceListHubResult, DeviceInfoHubResult};

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
    ) -> Result<Vec<crate::responses::ChildDeviceComponentList>, Error> {
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

hub_child_handlers!(HubHandler, "h100");

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
