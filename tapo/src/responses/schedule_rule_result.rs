use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::requests::{DaysOfWeek, ScheduleRule, ScheduleTime};
use crate::responses::{PowerState, TapoResponseExt};

/// A plug schedule rule read back from the device (the "Schedule" feature in
/// the Tapo app).
///
/// This is the lenient counterpart of [`ScheduleRule`]: it reports whatever
/// the device holds, so a rule written by a newer app or firmware does not
/// stop the rest of the listing from being read. Convert one into a validated
/// [`ScheduleRule`] with [`ScheduleRuleResult::to_editable`] to edit it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, frozen)
)]
pub struct ScheduleRuleResult {
    /// Device-assigned id.
    pub id: String,
    /// Whether the rule is currently active. Disabled rules are kept on the
    /// device but do not fire.
    pub enabled: bool,
    /// When the rule fires within a day.
    pub time: ScheduleTime,
    /// The days a repeating rule fires on, or `None` when it fires once.
    pub days: Option<DaysOfWeek>,
    /// The state the plug transitions to when the rule fires.
    pub desired_state: PowerState,
}

#[cfg_attr(feature = "python", pyo3::pymethods)]
impl ScheduleRuleResult {
    /// Returns this rule as a validated [`ScheduleRule`], carrying its id
    /// across, ready to be changed with `with_*` and passed to
    /// `edit_schedule_rule`.
    ///
    /// Fallible because this type is parsed leniently and may hold values the
    /// write type refuses, such as a repeating rule the device stored with no
    /// days set.
    pub fn to_editable(&self) -> Result<ScheduleRule, Error> {
        ScheduleRule::try_from(self)
    }
}

#[cfg(feature = "python")]
crate::impl_to_dict!(ScheduleRuleResult);

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScheduleRuleListResultRaw {
    #[serde(default)]
    pub rule_list: Vec<crate::requests::ScheduleRuleRaw>,
    #[serde(default)]
    pub sum: u32,
    /// How many rules the device can store in total. Kept as an `Option` so a
    /// firmware that omits it is distinguishable from one that reports zero.
    #[serde(default)]
    pub schedule_rule_max_count: Option<u32>,
}

impl TapoResponseExt for ScheduleRuleListResultRaw {}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AddScheduleRuleResult {
    pub id: String,
}

impl TapoResponseExt for AddScheduleRuleResult {}
