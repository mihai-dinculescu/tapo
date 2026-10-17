use chrono_tz::Tz;
use serde::{Deserialize, Deserializer};

use crate::responses::{TapoResponseExt, TimezoneResult};

/// Timezone result (`getTimezone`).
#[derive(Debug, Deserialize)]
pub(crate) struct TimezoneHubResultRaw {
    system: TimezoneSystemRaw,
}

impl TapoResponseExt for TimezoneHubResultRaw {}

impl From<TimezoneHubResultRaw> for TimezoneResult {
    fn from(raw: TimezoneHubResultRaw) -> Self {
        Self {
            timezone: raw.system.basic.zone_id,
            standard_offset: raw.system.basic.timezone,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TimezoneSystemRaw {
    basic: TimezoneBasicRaw,
}

#[derive(Debug, Deserialize)]
struct TimezoneBasicRaw {
    /// The UTC offset as a label, e.g. `UTC+01:00`, parsed into minutes.
    #[serde(deserialize_with = "deserialize_utc_offset_label")]
    timezone: i32,
    zone_id: Tz,
}

/// Parses a `UTC+HH:MM` or `UTC-HH:MM` label into minutes.
fn deserialize_utc_offset_label<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let label = String::deserialize(deserializer)?;

    parse_utc_offset_label(&label)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid UTC offset label: {label}")))
}

fn parse_utc_offset_label(label: &str) -> Option<i32> {
    let offset = label.strip_prefix("UTC")?;
    let (sign, offset) = match offset.split_at_checked(1)? {
        ("+", rest) => (1, rest),
        ("-", rest) => (-1, rest),
        _ => return None,
    };
    let (hours, minutes) = offset.split_once(':')?;
    let hours: u8 = hours.parse().ok()?;
    let minutes: u8 = minutes.parse().ok()?;

    if hours > 14 || minutes >= 60 {
        return None;
    }

    Some(sign * (i32::from(hours) * 60 + i32::from(minutes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone_parses() {
        let json =
            r#"{"system": {"basic": {"timezone": "UTC+01:00", "zone_id": "Europe/Brussels"}}}"#;

        let timezone: TimezoneResult = serde_json::from_str::<TimezoneHubResultRaw>(json)
            .unwrap()
            .into();

        assert_eq!(timezone.timezone, Tz::Europe__Brussels);
        assert_eq!(timezone.standard_offset, 60);
    }

    #[test]
    fn test_utc_offset_labels() {
        assert_eq!(parse_utc_offset_label("UTC-00:00"), Some(0));
        assert_eq!(parse_utc_offset_label("UTC+05:30"), Some(330));
        assert_eq!(parse_utc_offset_label("UTC-03:30"), Some(-210));
        assert_eq!(parse_utc_offset_label("UTC01:00"), None);
        assert_eq!(parse_utc_offset_label("UTC+14:00"), Some(840));
        assert_eq!(parse_utc_offset_label("UTC+01:60"), None);
        assert_eq!(parse_utc_offset_label("UTC+15:00"), None);
        assert_eq!(parse_utc_offset_label("UTC+-1:00"), None);
        assert_eq!(parse_utc_offset_label("GMT+01:00"), None);
    }

    #[test]
    fn test_unknown_zone_id_is_an_error() {
        let json =
            r#"{"system": {"basic": {"timezone": "UTC+01:00", "zone_id": "Europe/Atlantis"}}}"#;

        assert!(serde_json::from_str::<TimezoneHubResultRaw>(json).is_err());
    }
}
