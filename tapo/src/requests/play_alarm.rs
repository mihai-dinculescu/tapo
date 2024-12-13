use serde::Serialize;

/// The volume of the alarm.
/// For the H100, this is a fixed list of volume levels.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AlarmVolume {
    /// Mute the audio output from the alarm.
    /// This causes the alarm in the app to go off but nothing audible from the hub.
    /// You can check if the alarm is going off from the `in_alarm` field of
    /// [`crate::DeviceInfoHubResult`].
    Mute,
    /// Lowest volume.
    Low,
    /// Normal volume. This is the default.
    #[default]
    Normal,
    /// Highest volume.
    High,
}

/// The ringtone of a H100 alarm.
#[derive(Debug, Serialize)]
pub enum AlarmRingtone {
    /// Alarm 1
    #[serde(rename = "Alarm 1")]
    Alarm1,
    /// Alarm 2
    #[serde(rename = "Alarm 2")]
    Alarm2,
    /// Alarm 3
    #[serde(rename = "Alarm 3")]
    Alarm3,
    /// Alarm 4
    #[serde(rename = "Alarm 4")]
    Alarm4,
    /// Alarm 5
    #[serde(rename = "Alarm 5")]
    Alarm5,
    /// Connection 1
    #[serde(rename = "Connection 1")]
    Connection1,
    /// Connection 2
    #[serde(rename = "Connection 2")]
    Connection2,
    /// Doorbell Ring 1
    #[serde(rename = "Doorbell Ring 1")]
    DoorbellRing1,
    /// Doorbell Ring 2
    #[serde(rename = "Doorbell Ring 2")]
    DoorbellRing2,
    /// Doorbell Ring 3
    #[serde(rename = "Doorbell Ring 3")]
    DoorbellRing3,
    /// Doorbell Ring 4
    #[serde(rename = "Doorbell Ring 4")]
    DoorbellRing4,
    /// Doorbell Ring 5
    #[serde(rename = "Doorbell Ring 5")]
    DoorbellRing5,
    /// Doorbell Ring 6
    #[serde(rename = "Doorbell Ring 6")]
    DoorbellRing6,
    /// Doorbell Ring 7
    #[serde(rename = "Doorbell Ring 7")]
    DoorbellRing7,
    /// Doorbell Ring 8
    #[serde(rename = "Doorbell Ring 8")]
    DoorbellRing8,
    /// Doorbell Ring 9
    #[serde(rename = "Doorbell Ring 9")]
    DoorbellRing9,
    /// Doorbell Ring 10
    #[serde(rename = "Doorbell Ring 10")]
    DoorbellRing10,
    /// Dripping Tap
    #[serde(rename = "Dripping Tap")]
    DrippingTap,
    /// Phone Ring
    #[serde(rename = "Phone Ring")]
    PhoneRing,
}

/// Parameters for playing the alarm on a H100 hub.
#[derive(Debug, Default, Serialize)]
pub(crate) struct PlayAlarmParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_type: Option<AlarmRingtone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_volume: Option<AlarmVolume>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_duration: Option<u32>,
}
impl PlayAlarmParams {
    pub(crate) fn new(
        ringtone: Option<AlarmRingtone>,
        volume: Option<AlarmVolume>,
        duration: Option<u32>,
    ) -> Self {
        Self {
            alarm_type: ringtone,
            alarm_volume: volume,
            alarm_duration: duration,
        }
    }
}
