use serde::{Deserialize, Serialize};

use crate::error::{Error, TapoResponseError};
use crate::responses::{DesiredStateRaw, PowerState, TapoResponseExt};

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
    pub desired_states: Option<DesiredStateRaw>,
    #[serde(default)]
    pub action: Option<String>,
}

impl TimerRaw {
    /// Fails rather than returning `None` when the firing state cannot be
    /// read: the plug holds at most one timer, so a rule that is present but
    /// unreadable means a timer is armed and about to fire. Reporting "no
    /// timer" there would be worse than reporting an error.
    pub(crate) fn into_timer(self) -> Result<Timer, Error> {
        let desired_state = self
            .desired_states
            .unwrap_or_default()
            .resolve(self.action.as_deref())
            .ok_or_else(|| {
                Error::Tapo(TapoResponseError::ResponseError {
                    description: format!(
                        "Timer {} reported neither desired_states.on nor a recognised action",
                        self.id
                    ),
                })
            })?;

        Ok(Timer {
            id: self.id,
            delay_s: self.delay,
            remaining_s: self.remain,
            desired_state,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AddTimerResult {
    pub id: String,
}

impl TapoResponseExt for AddTimerResult {}
