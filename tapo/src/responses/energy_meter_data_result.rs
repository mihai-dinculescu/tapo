use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Contains the voltage, current and power measured by the device, and the energy it has used since it last restarted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct EnergyMeterDataResult {
    /// Current in milliamperes (mA).
    pub current_ma: u64,
    /// Energy used since the device last restarted, in milliwatt hours (mWh).
    pub energy_mwh: u64,
    /// Energy used since the device last restarted, in Watt Hours (Wh).
    pub energy_wh: u64,
    /// Active power in milliwatts (mW).
    pub power_mw: u64,
    /// Voltage in millivolts (mV).
    pub voltage_mv: u64,
}
impl TapoResponseExt for EnergyMeterDataResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(EnergyMeterDataResult);
