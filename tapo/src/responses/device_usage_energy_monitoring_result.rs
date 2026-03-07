use serde::{Deserialize, Serialize};

use super::{TapoResponseExt, UsageByPeriodResult};

/// Contains the time usage, the power consumption, and the energy savings of the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct DeviceUsageEnergyMonitoringResult {
    /// Time usage in minutes.
    pub time_usage: UsageByPeriodResult,
    /// Power usage in Watt Hours (Wh).
    pub power_usage: UsageByPeriodResult,
    /// Saved power in Watt Hours (Wh).
    pub saved_power: UsageByPeriodResult,
}
impl TapoResponseExt for DeviceUsageEnergyMonitoringResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceUsageEnergyMonitoringResult);
