use serde::Deserialize;

use crate::requests::ScheduleRule;
use crate::responses::TapoResponseExt;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScheduleRuleListResultRaw {
    #[serde(default)]
    pub rule_list: Vec<ScheduleRule>,
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
