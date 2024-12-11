use crate::{Error, HubHandler};
use serde::Serialize;

/// The volume of the alarm.
/// For the H100, this is a fixed list of volume levels.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AlarmVolume {
    /// Mute the audio output from the alarm.
    /// This causes the alarm in the app to go off but nothing audible from the hub.
    Mute,
    /// Lowest volume.
    Low,
    /// Normal volume. This is the default.
    #[default]
    Normal,
    /// Highest volume.
    High,
}

/// Parameters for playing the alarm on a H100 hub.
#[derive(Debug, Default, Serialize)]
pub struct PlayAlarmParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_duration: Option<u32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    alarm_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_volume: Option<AlarmVolume>,
}
impl PlayAlarmParams {
    /// Override the volume to play the alarm at.
    pub fn with_alarm_volume(mut self, volume: AlarmVolume) -> Self {
        self.alarm_volume = Some(volume);
        self
    }

    /// Override the alarm ringtone to play.
    /// You can get the list of supported values from (HubHandler)[crate::HubHandler::get_supported_alarm_type_list].
    /// e.g. "Alarm 1".
    pub fn with_alarm_type(mut self, alarm_type: impl Into<String>) -> Self {
        self.alarm_type = alarm_type.into();
        self
    }

    /// Override the number of seconds to play the alarm.
    pub fn with_alarm_duration(mut self, seconds: u32) -> Self {
        self.alarm_duration = Some(seconds);
        self
    }

    /// Send the request to the given hub.
    pub async fn send(self, hub: &HubHandler) -> Result<(), Error> {
        hub.get_client().read().await.play_alarm(self).await
    }
}
