use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Timezone result (`get_device_time`).
#[derive(Debug, Deserialize)]
pub(crate) struct TimezoneResultRaw {
    region: Tz,
    time_diff: i32,
}

impl TapoResponseExt for TimezoneResultRaw {}

impl From<TimezoneResultRaw> for TimezoneResult {
    fn from(raw: TimezoneResultRaw) -> Self {
        Self {
            timezone: raw.region,
            standard_offset: raw.time_diff,
        }
    }
}

/// The timezone configured on a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct TimezoneResult {
    /// The device's timezone, from the IANA name it reports, e.g. `Europe/London`.
    pub timezone: Tz,
    /// The device's standard UTC offset in minutes.
    /// It does not follow daylight saving, so it can be an hour off the
    /// offset in effect; convert times with `timezone` instead.
    pub standard_offset: i32,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(TimezoneResult);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone_parses() {
        let json = r#"{"region": "Europe/London", "time_diff": 0, "timestamp": 1792244123}"#;

        let timezone: TimezoneResult = serde_json::from_str::<TimezoneResultRaw>(json)
            .unwrap()
            .into();

        assert_eq!(timezone.timezone, Tz::Europe__London);
        assert_eq!(timezone.standard_offset, 0);
    }

    #[test]
    fn test_unknown_region_is_an_error() {
        let json = r#"{"region": "Europe/Atlantis", "time_diff": 0, "timestamp": 1792244123}"#;

        assert!(serde_json::from_str::<TimezoneResultRaw>(json).is_err());
    }
}
