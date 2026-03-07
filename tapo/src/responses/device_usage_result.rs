use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::utils::ok_or_default;

/// Contains the time usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct DeviceUsageResult {
    /// Time usage in minutes.
    pub time_usage: UsageByPeriodResult,
}
impl TapoResponseExt for DeviceUsageResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceUsageResult);

/// Usage by period result for today, the past 7 days, and the past 30 days.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct UsageByPeriodResult {
    /// Today.
    #[serde(deserialize_with = "ok_or_default")]
    pub today: Option<u64>,
    /// Past 7 days.
    #[serde(deserialize_with = "ok_or_default")]
    pub past7: Option<u64>,
    /// Past 30 days.
    #[serde(deserialize_with = "ok_or_default")]
    pub past30: Option<u64>,
}
