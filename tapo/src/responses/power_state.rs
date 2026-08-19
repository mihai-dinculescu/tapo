use serde::{Deserialize, Serialize};

/// The power state of a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, eq_int)
)]
pub enum PowerState {
    /// The device is on.
    On,
    /// The device is off.
    Off,
}

/// The `desired_states` payload exchanged with the device, e.g. `{ "on": true }`.
///
/// Shared by the plug's countdown timer and its schedule rules, which encode
/// the state to transition to identically. `on` is optional because some
/// firmwares emit an empty object and carry the state in a legacy field
/// instead (`action` for timers, `s_action` for schedule rules).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct DesiredStateRaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
}

impl DesiredStateRaw {
    pub(crate) fn new(desired_state: PowerState) -> Self {
        Self {
            on: Some(desired_state == PowerState::On),
        }
    }

    /// Resolves the firing state, falling back to a legacy `on` / `off` string
    /// field when `desired_states.on` is absent.
    pub(crate) fn resolve(&self, legacy_action: Option<&str>) -> Option<PowerState> {
        let on = self.on.or(match legacy_action {
            Some("on") => Some(true),
            Some("off") => Some(false),
            _ => None,
        })?;

        Some(if on { PowerState::On } else { PowerState::Off })
    }
}
