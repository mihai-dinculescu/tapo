use crate::error::Error;
use crate::requests::{AlarmDuration, AlarmRingtone, AlarmVolume, PlayAlarmParams};
use crate::responses::DeviceInfoHubResult;

tapo_handler! {
    /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.
    HubHandler(DeviceInfoHubResult),
    device_management,
}

/// Hub handler methods.
impl HubHandler {
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
