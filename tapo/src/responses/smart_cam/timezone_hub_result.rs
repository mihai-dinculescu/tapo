use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Timezone result (`getTimezone`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TimezoneHubResultRaw {
    system: TimezoneSystemRaw,
}

impl TimezoneHubResultRaw {
    pub fn timezone(self) -> TimezoneHubResult {
        self.system.basic
    }
}

impl TapoResponseExt for TimezoneHubResultRaw {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TimezoneSystemRaw {
    basic: TimezoneHubResult,
}

/// The timezone configured on a camera hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneHubResult {
    /// The hub's standard UTC offset as a label, e.g. `UTC+01:00`.
    /// It does not follow daylight saving, so it can be an hour off the
    /// offset in effect; convert times with `zone_id` instead.
    pub timezone: String,
    /// The hub's timezone, from the IANA name it reports, e.g. `Europe/Paris`.
    pub zone_id: Tz,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone_parses() {
        let json =
            r#"{"system": {"basic": {"timezone": "UTC+01:00", "zone_id": "Europe/Brussels"}}}"#;

        let parsed: TimezoneHubResultRaw = serde_json::from_str(json).unwrap();
        let timezone = parsed.timezone();

        assert_eq!(timezone.timezone, "UTC+01:00");
        assert_eq!(timezone.zone_id, Tz::Europe__Brussels);
    }

    #[test]
    fn test_unknown_zone_id_is_an_error() {
        let json =
            r#"{"system": {"basic": {"timezone": "UTC+01:00", "zone_id": "Europe/Atlantis"}}}"#;

        assert!(serde_json::from_str::<TimezoneHubResultRaw>(json).is_err());
    }
}
