use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains local time, current power and the energy usage and runtime for today and for the current month.
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentPowerResult {
    /// Current power in watts (W)
    pub current_power: u64,
}
impl TapoResponseExt for CurrentPowerResult {}
