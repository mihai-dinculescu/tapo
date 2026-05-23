use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AddTimerParams {
    pub enable: bool,
    pub delay: u32,
    pub desired_states: serde_json::Value,
}

impl AddTimerParams {
    pub(crate) fn new(delay_seconds: u32, turn_on: bool) -> Self {
        Self {
            enable: true,
            delay: delay_seconds,
            desired_states: serde_json::json!({ "on": turn_on }),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RemoveTimersParams {
    pub remove_all: bool,
    pub rule_list: Vec<TimerIdParam>,
}

impl RemoveTimersParams {
    pub(crate) fn remove_all() -> Self {
        Self {
            remove_all: true,
            rule_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct TimerIdParam {
    pub id: String,
}
