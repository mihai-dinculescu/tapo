use crate::Error;
use serde::{Serialize, Serializer};

/// The volume of the alarm.
/// For the H100, this is a fixed list of volume levels.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
pub enum AlarmVolume {
    /// Use the default volume for the hub.
    #[default]
    Default,
    /// Mute the audio output from the alarm.
    /// This causes the alarm to be shown as triggered in the Tapo App
    /// without an audible sound, and makes the `in_alarm` property
    /// in [`crate::responses::DeviceInfoHubResult`] return as `true`.
    Mute,
    /// Lowest volume.
    Low,
    /// Normal volume. This is the default.
    Normal,
    /// Highest volume.
    High,
}

impl AlarmVolume {
    fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}

/// The ringtone of a H100 alarm.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
pub enum AlarmRingtone {
    /// Use the default ringtone for the hub.
    #[default]
    Default,
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

impl AlarmRingtone {
    fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}

/// Controls how long the alarm plays for.
#[derive(Debug, Clone, Copy)]
pub enum AlarmDuration {
    /// Play the alarm continuously until stopped.
    Continuous,
    /// Play the alarm once.
    /// This is useful for previewing the audio.
    ///
    /// # Limitations
    /// The `in_alarm` field of [`crate::responses::DeviceInfoHubResult`] will not remain `true` for the
    /// duration of the audio track. Each audio track has a different runtime.
    ///
    /// Has no observable affect when used in conjunction with [`AlarmVolume::Mute`].
    Once,
    /// Play the alarm a number of seconds.
    Seconds(u32),
}
impl AlarmDuration {
    fn is_continuous(&self) -> bool {
        matches!(self, Self::Continuous)
    }
}
impl Serialize for AlarmDuration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let as_option = match self {
            Self::Continuous => None,
            Self::Once => Some(0),
            Self::Seconds(seconds) => Some(*seconds),
        };
        Serialize::serialize(&as_option, serializer)
    }
}

/// Parameters for playing the alarm on a H100 hub.
#[derive(Debug, Serialize)]
pub(crate) struct PlayAlarmParams {
    #[serde(skip_serializing_if = "AlarmRingtone::is_default")]
    alarm_type: AlarmRingtone,
    #[serde(skip_serializing_if = "AlarmVolume::is_default")]
    alarm_volume: AlarmVolume,
    #[serde(skip_serializing_if = "AlarmDuration::is_continuous")]
    alarm_duration: AlarmDuration,
}
impl PlayAlarmParams {
    pub(crate) fn new(
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: AlarmDuration,
    ) -> Result<Self, Error> {
        let params = Self {
            alarm_type: ringtone,
            alarm_volume: volume,
            alarm_duration: duration,
        };
        params.validate()?;
        Ok(params)
    }

    fn validate(&self) -> Result<(), Error> {
        match self.alarm_duration {
            AlarmDuration::Seconds(0) => Err(Error::Validation {
                field: "duration".to_string(),
                message: "The seconds value must be greater than zero".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_inputs() {
        for valid_ringtone in [AlarmRingtone::Default, AlarmRingtone::Alarm1] {
            for valid_volume in [AlarmVolume::Default, AlarmVolume::Normal] {
                for valid_duration in [
                    AlarmDuration::Continuous,
                    AlarmDuration::Once,
                    AlarmDuration::Seconds(1),
                ] {
                    let result = PlayAlarmParams::new(valid_ringtone, valid_volume, valid_duration);
                    assert!(result.is_ok());
                }
            }
        }
    }

    #[test]
    fn test_invalid_inputs() {
        let result = PlayAlarmParams::new(
            AlarmRingtone::Default,
            AlarmVolume::Default,
            AlarmDuration::Seconds(0),
        );
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "duration" && message == "The seconds value must be greater than zero"
        ));
    }

    fn params_to_json(
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: AlarmDuration,
    ) -> String {
        let params = PlayAlarmParams::new(ringtone, volume, duration).unwrap();
        serde_json::to_string(&params).expect("Serialization failed")
    }

    #[test]
    fn test_serialize_params_where_ringtone_is_some() {
        assert_eq!(
            r#"{"alarm_type":"Alarm 1"}"#,
            params_to_json(
                AlarmRingtone::Alarm1,
                AlarmVolume::Default,
                AlarmDuration::Continuous
            )
        );
    }

    #[test]
    fn test_serialize_params_where_volume_is_some() {
        assert_eq!(
            r#"{"alarm_volume":"mute"}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::Mute,
                AlarmDuration::Continuous
            )
        );
        assert_eq!(
            r#"{"alarm_volume":"low"}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::Low,
                AlarmDuration::Continuous
            )
        );
        assert_eq!(
            r#"{"alarm_volume":"normal"}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::Normal,
                AlarmDuration::Continuous
            )
        );
        assert_eq!(
            r#"{"alarm_volume":"high"}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::High,
                AlarmDuration::Continuous
            )
        );
    }

    #[test]
    fn test_serialize_params_where_duration_is_continuous() {
        assert_eq!(
            r#"{}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::Default,
                AlarmDuration::Continuous
            )
        );
    }

    #[test]
    fn test_serialize_params_where_duration_is_once() {
        assert_eq!(
            r#"{"alarm_duration":0}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::Default,
                AlarmDuration::Once
            )
        );
    }

    #[test]
    fn test_serialize_params_where_duration_is_1second() {
        assert_eq!(
            r#"{"alarm_duration":1}"#,
            params_to_json(
                AlarmRingtone::Default,
                AlarmVolume::Default,
                AlarmDuration::Seconds(1)
            )
        );
    }

    #[test]
    fn test_serialize_all_params_are_some_and_duration_is_1second() {
        assert_eq!(
            r#"{"alarm_type":"Doorbell Ring 1","alarm_volume":"normal","alarm_duration":1}"#,
            params_to_json(
                AlarmRingtone::DoorbellRing1,
                AlarmVolume::Normal,
                AlarmDuration::Seconds(1)
            )
        );
    }
}
