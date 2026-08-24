use crate::error::Error;
use crate::responses::{PowerState, ScheduleRuleResult};

use super::{DaysOfWeek, ScheduleTime};

/// The largest sunrise / sunset offset the builders accept, in minutes.
///
/// Measured on a P110: `±360` is stored, `±361` and beyond are refused with
/// `-1008 PARAMS`. This matches the ±6 hours the Tapo app offers.
const MAX_OFFSET_MINUTES: i16 = 360;

/// A plug schedule rule to send to the device (the "Schedule" feature in the
/// Tapo app).
///
/// Values are valid by construction: the fields are private and the builders
/// ([`ScheduleRule::clock_weekly`], [`ScheduleRule::sunrise_once`], …) are the
/// only way to make one, each returning an `Error::Validation` for
/// out-of-range input. Rules read back from the device are the separate
/// [`ScheduleRuleResult`], which is lenient rather than validated; convert one
/// for editing with [`ScheduleRuleResult::to_editable`].
///
/// The device evaluates the time against its own configured timezone; you
/// don't supply a calendar date. The on-the-wire `year` / `month` / `day`
/// fields the device requires are filled with a constant placeholder
/// (`1970-01-01`) because the device ignores their values — this was
/// confirmed experimentally on a P110: a `clock_once` sent with
/// `year=1970, month=1, day=1` still fires at the requested HH:MM.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, frozen))]
pub struct ScheduleRule {
    pub(super) id: Option<String>,
    pub(super) enabled: bool,
    pub(super) time: ScheduleTime,
    /// `None` fires once; `Some(days)` repeats weekly on those days.
    pub(super) days: Option<DaysOfWeek>,
    pub(super) desired_state: PowerState,
}

impl ScheduleRule {
    /// Fires every week, on `days`, at the given wall-clock time.
    ///
    /// # Arguments
    ///
    /// * `hour` - hour of the day, `0..=23`.
    /// * `minute` - minute of the hour, `0..=59`.
    /// * `days` - the days to fire on; must not be empty.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn clock_weekly(
        hour: u8,
        minute: u8,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        Self::new(
            ScheduleTime::Clock { hour, minute },
            Some(days),
            desired_state,
        )
    }

    /// Fires once, the next time the device's wall clock reaches `hour:minute`.
    ///
    /// # Arguments
    ///
    /// * `hour` - hour of the day, `0..=23`.
    /// * `minute` - minute of the hour, `0..=59`.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn clock_once(hour: u8, minute: u8, desired_state: PowerState) -> Result<Self, Error> {
        Self::new(ScheduleTime::Clock { hour, minute }, None, desired_state)
    }

    /// Fires every week, on `days`, at `offset_minutes` from sunrise.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunrise, `-360..=360`; negative
    ///   fires before it.
    /// * `days` - the days to fire on; must not be empty.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn sunrise_weekly(
        offset_minutes: i16,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        Self::new(
            ScheduleTime::Sunrise { offset_minutes },
            Some(days),
            desired_state,
        )
    }

    /// Fires once, at the next sunrise plus `offset_minutes`.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunrise, `-360..=360`; negative
    ///   fires before it.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn sunrise_once(offset_minutes: i16, desired_state: PowerState) -> Result<Self, Error> {
        Self::new(
            ScheduleTime::Sunrise { offset_minutes },
            None,
            desired_state,
        )
    }

    /// Fires every week, on `days`, at `offset_minutes` from sunset.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunset, `-360..=360`; negative
    ///   fires before it.
    /// * `days` - the days to fire on; must not be empty.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn sunset_weekly(
        offset_minutes: i16,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        Self::new(
            ScheduleTime::Sunset { offset_minutes },
            Some(days),
            desired_state,
        )
    }

    /// Fires once, at the next sunset plus `offset_minutes`.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunset, `-360..=360`; negative
    ///   fires before it.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn sunset_once(offset_minutes: i16, desired_state: PowerState) -> Result<Self, Error> {
        Self::new(ScheduleTime::Sunset { offset_minutes }, None, desired_state)
    }

    fn new(
        time: ScheduleTime,
        days: Option<DaysOfWeek>,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        match time {
            ScheduleTime::Clock { hour, minute } => {
                if hour >= 24 {
                    return Err(Error::Validation {
                        field: "hour".to_string(),
                        message: format!("Must be 0..=23, got {hour}"),
                    });
                }
                if minute >= 60 {
                    return Err(Error::Validation {
                        field: "minute".to_string(),
                        message: format!("Must be 0..=59, got {minute}"),
                    });
                }
            }
            ScheduleTime::Sunrise { offset_minutes } | ScheduleTime::Sunset { offset_minutes } => {
                if offset_minutes.unsigned_abs() > MAX_OFFSET_MINUTES.unsigned_abs() {
                    return Err(Error::Validation {
                        field: "offset_minutes".to_string(),
                        message: format!(
                            "Must be within -{MAX_OFFSET_MINUTES}..={MAX_OFFSET_MINUTES} minutes (±6h), got {offset_minutes}",
                        ),
                    });
                }
            }
        }

        if let Some(days) = days
            && days.is_empty()
        {
            return Err(Error::Validation {
                field: "days".to_string(),
                message: "A repeating rule must fire on at least one day".to_string(),
            });
        }

        Ok(Self {
            id: None,
            enabled: true,
            time,
            days,
            desired_state,
        })
    }

    /// Returns a copy of this rule with `id` set to the given value.
    /// Use before `edit_schedule_rule` (on
    /// [`PlugHandler`](crate::PlugHandler) or
    /// [`PlugEnergyMonitoringHandler`](crate::PlugEnergyMonitoringHandler))
    /// when reconstructing an edit from scratch;
    /// [`ScheduleRuleResult::to_editable`] carries the id across for you.
    ///
    /// # Arguments
    ///
    /// * `id` - the device-assigned id of the rule to update.
    pub fn with_id(&self, id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..self.clone()
        }
    }

    pub(crate) fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Pairs this rule with the id the device assigned it, for returning from
    /// `add_schedule_rule` without a second round trip.
    pub(crate) fn into_result(self, id: String) -> ScheduleRuleResult {
        ScheduleRuleResult {
            id,
            enabled: self.enabled,
            time: self.time,
            days: self.days,
            desired_state: self.desired_state,
        }
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl ScheduleRule {
    // `#[staticmethod]` is an inner attribute of `#[pymethods]`, so it cannot
    // be written behind `cfg_attr` and these cannot join the shared impl
    // above. `From<Error> for PyErr` means they need no error mapping.
    #[staticmethod]
    #[pyo3(name = "clock_weekly")]
    fn py_clock_weekly(
        hour: u8,
        minute: u8,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        Self::clock_weekly(hour, minute, days, desired_state)
    }

    #[staticmethod]
    #[pyo3(name = "clock_once")]
    fn py_clock_once(hour: u8, minute: u8, desired_state: PowerState) -> Result<Self, Error> {
        Self::clock_once(hour, minute, desired_state)
    }

    #[staticmethod]
    #[pyo3(name = "sunrise_weekly")]
    fn py_sunrise_weekly(
        offset_minutes: i16,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        Self::sunrise_weekly(offset_minutes, days, desired_state)
    }

    #[staticmethod]
    #[pyo3(name = "sunrise_once")]
    fn py_sunrise_once(offset_minutes: i16, desired_state: PowerState) -> Result<Self, Error> {
        Self::sunrise_once(offset_minutes, desired_state)
    }

    #[staticmethod]
    #[pyo3(name = "sunset_weekly")]
    fn py_sunset_weekly(
        offset_minutes: i16,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> Result<Self, Error> {
        Self::sunset_weekly(offset_minutes, days, desired_state)
    }

    #[staticmethod]
    #[pyo3(name = "sunset_once")]
    fn py_sunset_once(offset_minutes: i16, desired_state: PowerState) -> Result<Self, Error> {
        Self::sunset_once(offset_minutes, desired_state)
    }

    #[pyo3(name = "with_id")]
    fn py_with_id(&self, id: String) -> Self {
        self.with_id(id)
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

/// `&self` methods need no pyo3 attribute, so they are shared with Python
/// directly rather than through wrappers.
#[cfg_attr(feature = "python", pyo3::pymethods)]
impl ScheduleRule {
    /// Returns a copy of this rule with `enabled` set to the given value.
    ///
    /// # Arguments
    ///
    /// * `enabled` - whether the rule should fire; a disabled rule stays on
    ///   the device without firing.
    pub fn with_enabled(&self, enabled: bool) -> Self {
        Self {
            enabled,
            ..self.clone()
        }
    }
}

impl TryFrom<&ScheduleRuleResult> for ScheduleRule {
    type Error = Error;

    /// Re-runs the builder validation, because a [`ScheduleRuleResult`] is
    /// parsed leniently and may hold values this type must refuse.
    fn try_from(result: &ScheduleRuleResult) -> Result<Self, Error> {
        Ok(Self::new(result.time, result.days, result.desired_state)?
            .with_enabled(result.enabled)
            .with_id(result.id.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sun_rules_reject_offsets_beyond_the_device_limit() {
        // The device stores ±360 and refuses ±361, so the builders draw the
        // line in the same place.
        ScheduleRule::sunrise_once(360, PowerState::On).expect("360 is accepted");
        ScheduleRule::sunset_once(-360, PowerState::On).expect("-360 is accepted");

        for offset in [361, -361, 1440] {
            let err = ScheduleRule::sunrise_once(offset, PowerState::On)
                .expect_err("should reject offset");
            assert!(
                matches!(err, Error::Validation { ref field, .. } if field == "offset_minutes")
            );
        }
    }

    #[test]
    fn builders_reject_out_of_range_clock_and_empty_days() {
        let err = ScheduleRule::clock_weekly(24, 0, DaysOfWeek::MON, PowerState::On)
            .expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "hour"));

        let err = ScheduleRule::clock_weekly(0, 60, DaysOfWeek::MON, PowerState::On)
            .expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "minute"));

        for build in [
            ScheduleRule::clock_weekly(8, 0, DaysOfWeek::NONE, PowerState::On),
            ScheduleRule::sunset_weekly(0, DaysOfWeek::NONE, PowerState::On),
        ] {
            let err = build.expect_err("should reject");
            assert!(matches!(err, Error::Validation { ref field, .. } if field == "days"));
        }
    }

    #[test]
    fn with_enabled_and_with_id_clone_and_override() {
        let rule = ScheduleRule::clock_weekly(8, 0, DaysOfWeek::MON, PowerState::On)
            .expect("valid")
            .with_id("S42");
        let disabled = rule.with_enabled(false);

        assert!(rule.enabled);
        assert!(!disabled.enabled);
        assert_eq!(disabled.id(), Some("S42"));
    }
}
