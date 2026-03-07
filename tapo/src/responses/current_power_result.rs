use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the current power reading of the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct CurrentPowerResult {
    /// Current power in Watts (W).
    pub current_power: u64,
}
impl TapoResponseExt for CurrentPowerResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(CurrentPowerResult);
