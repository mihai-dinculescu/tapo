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
    /// The timezone label reported by the hub.
    pub timezone: String,
    /// The IANA timezone name, e.g. `Europe/Paris`.
    pub zone_id: String,
}
