use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// A pending "Timer" countdown on a Tapo plug. Plugs accept at most
/// one armed timer at a time, set via `set_timer` and cleared via
/// `clear_timer` / `set_timer`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct Timer {
    /// Device-assigned id.
    pub id: String,
    /// Total countdown duration in seconds.
    pub delay_seconds: u32,
    /// Seconds left until the timer fires.
    pub remaining_seconds: u32,
    /// Whether the timer turns the plug on (`true`) or off (`false`) when it fires.
    pub turn_on: bool,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(Timer);

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TimerListResult {
    #[serde(default)]
    pub rule_list: Vec<RawTimer>,
}

impl TapoResponseExt for TimerListResult {}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RawTimer {
    pub id: String,
    pub delay: u32,
    #[serde(default)]
    pub remain: u32,
    #[serde(default)]
    pub desired_states: Option<serde_json::Value>,
    #[serde(default)]
    pub action: Option<String>,
}

impl RawTimer {
    pub(crate) fn into_timer(self) -> Option<Timer> {
        let turn_on = self
            .desired_states
            .as_ref()
            .and_then(|v| v.get("on").and_then(|x| x.as_bool()))
            .or(match self.action.as_deref() {
                Some("on") => Some(true),
                Some("off") => Some(false),
                _ => None,
            })?;
        Some(Timer {
            id: self.id,
            delay_seconds: self.delay,
            remaining_seconds: self.remain,
            turn_on,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AddTimerResult {
    pub id: String,
}

impl TapoResponseExt for AddTimerResult {}
