use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GetScheduleRulesParams {
    pub start_index: u32,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RemoveScheduleRulesParams {
    pub remove_all: bool,
    /// Omitted entirely when `remove_all` is set, where the device ignores it.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_list: Vec<ScheduleRuleIdParam>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ScheduleRuleIdParam {
    pub id: String,
}

impl RemoveScheduleRulesParams {
    pub(crate) fn remove_all() -> Self {
        Self {
            remove_all: true,
            rule_list: Vec::new(),
        }
    }

    pub(crate) fn specific(ids: Vec<String>) -> Self {
        Self {
            remove_all: false,
            rule_list: ids
                .into_iter()
                .map(|id| ScheduleRuleIdParam { id })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_all_params_omit_the_rule_list() {
        let json =
            serde_json::to_value(RemoveScheduleRulesParams::remove_all()).expect("serialize");
        assert_eq!(json["remove_all"], true);
        assert!(json.get("rule_list").is_none());

        let json =
            serde_json::to_value(RemoveScheduleRulesParams::specific(vec!["S1".to_string()]))
                .expect("serialize");
        assert_eq!(json["remove_all"], false);
        assert_eq!(json["rule_list"], serde_json::json!([{ "id": "S1" }]));
    }
}
