use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::responses::TapoResponseExt;
use crate::tapo_date_format;

/// Contains local time, current power and the energy usage and runtime for today and for the current month.
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentPowerResult {
    /// Current power in milliwatts (mW)
    pub current_power: u64,
}
impl TapoResponseExt for CurrentPowerResult {}
