use serde::{Deserialize, Serialize};

use crate::responses::{PowerState, TapoResponseExt};

/// A pending "Timer" countdown on a Tapo plug. Plugs accept at most
/// one armed timer at a time, set via `set_timer` and cleared via
/// `clear_timer` / `set_timer`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct Timer {
    /// Device-assigned id.
    pub id: String,
    /// Total countdown duration in seconds.
    pub delay_s: u32,
    /// Seconds left until the timer fires.
    pub remaining_s: u32,
    /// The state the plug transitions to when the timer fires.
    pub desired_state: PowerState,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(Timer);

/// The `desired_states` payload exchanged with the device, e.g. `{ "on": true }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TimerDesiredStateRaw {
    pub on: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TimerListResultRaw {
    #[serde(default)]
    pub rule_list: Vec<TimerRaw>,
}

impl TapoResponseExt for TimerListResultRaw {}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TimerRaw {
    pub id: String,
    pub delay: u32,
    #[serde(default)]
    pub remain: u32,
    #[serde(default)]
    pub desired_states: Option<TimerDesiredStateRaw>,
    #[serde(default)]
    pub action: Option<String>,
}

impl TimerRaw {
    pub(crate) fn into_timer(self) -> Option<Timer> {
        let on = self.desired_states.as_ref().map(|states| states.on).or(
            match self.action.as_deref() {
                Some("on") => Some(true),
                Some("off") => Some(false),
                _ => None,
            },
        )?;
        Some(Timer {
            id: self.id,
            delay_s: self.delay,
            remaining_s: self.remain,
            desired_state: if on { PowerState::On } else { PowerState::Off },
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AddTimerResult {
    pub id: String,
}

impl TapoResponseExt for AddTimerResult {}
