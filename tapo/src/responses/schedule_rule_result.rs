use serde::Deserialize;

use crate::requests::ScheduleRule;
use crate::responses::TapoResponseExt;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScheduleRuleListResult {
    #[serde(default)]
    pub rule_list: Vec<ScheduleRule>,
    #[serde(default)]
    pub sum: u32,
}

impl TapoResponseExt for ScheduleRuleListResult {}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScheduleRuleAddResult {
    pub id: String,
}

impl TapoResponseExt for ScheduleRuleAddResult {}
