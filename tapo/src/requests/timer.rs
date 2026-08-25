use serde::Serialize;

use crate::responses::{DesiredStateRaw, PowerState};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AddTimerParams {
    pub enable: bool,
    pub delay: u32,
    pub desired_states: DesiredStateRaw,
}

impl AddTimerParams {
    pub(crate) fn new(delay_seconds: u32, desired_state: PowerState) -> Self {
        Self {
            enable: true,
            delay: delay_seconds,
            desired_states: DesiredStateRaw::new(desired_state),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RemoveTimersParams {
    pub remove_all: bool,
}

impl RemoveTimersParams {
    pub(crate) fn remove_all() -> Self {
        Self { remove_all: true }
    }
}
