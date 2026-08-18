//! Request types for the plug "Schedule" feature.

use std::fmt;
use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DesiredStateRaw, PowerState, ScheduleRuleResult};

/// The days of the week a repeating [`ScheduleRule`] fires on.
///
/// Combine the individual days with `|`, or use one of the preset groups:
///
/// ```
/// use tapo::requests::DaysOfWeek;
///
/// let midweek = DaysOfWeek::MON | DaysOfWeek::WED;
/// assert!(midweek.contains(DaysOfWeek::MON));
/// assert!(!midweek.contains(DaysOfWeek::FRI));
///
/// assert_eq!(DaysOfWeek::WEEKEND, DaysOfWeek::SUN | DaysOfWeek::SAT);
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, eq, hash, frozen)
)]
#[serde(transparent)]
pub struct DaysOfWeek(u8);

impl DaysOfWeek {
    /// No days. A repeating rule with no days would never fire, so the
    /// `*_weekly` builders reject it.
    pub const NONE: Self = Self(0);
    /// Sunday.
    pub const SUN: Self = Self(1 << 0);
    /// Monday.
    pub const MON: Self = Self(1 << 1);
    /// Tuesday.
    pub const TUE: Self = Self(1 << 2);
    /// Wednesday.
    pub const WED: Self = Self(1 << 3);
    /// Thursday.
    pub const THU: Self = Self(1 << 4);
    /// Friday.
    pub const FRI: Self = Self(1 << 5);
    /// Saturday.
    pub const SAT: Self = Self(1 << 6);
    /// Monday through Friday.
    pub const WEEKDAYS: Self =
        Self(Self::MON.0 | Self::TUE.0 | Self::WED.0 | Self::THU.0 | Self::FRI.0);
    /// Saturday and Sunday.
    pub const WEEKEND: Self = Self(Self::SUN.0 | Self::SAT.0);
    /// Every day of the week.
    pub const EVERY_DAY: Self = Self(Self::WEEKDAYS.0 | Self::WEEKEND.0);

    /// Builds a set from a device bitmask, ignoring any bits above Saturday.
    /// Only bits 0..=6 are meaningful to the device, so this is the inverse
    /// of [`DaysOfWeek::bits`].
    ///
    /// # Arguments
    ///
    /// * `bits` - a device bitmask; bit 0 is Sunday through bit 6, Saturday.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(bits & Self::EVERY_DAY.0)
    }
}

/// Methods with a `&self` receiver need no pyo3 attribute, so they are shared
/// with Python directly rather than through wrappers.
#[cfg_attr(feature = "python", pyo3::pymethods)]
impl DaysOfWeek {
    /// Returns the device bitmask for this set: bit 0 is Sunday through
    /// bit 6, Saturday.
    pub const fn bits(&self) -> u8 {
        self.0
    }

    /// Returns `true` if every day in `other` is also in this set.
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns `true` if this set contains no days.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl DaysOfWeek {
    /// The individual days, Sunday first, paired with their names.
    const fn all_days() -> [(Self, &'static str); 7] {
        [
            (Self::SUN, "SUN"),
            (Self::MON, "MON"),
            (Self::TUE, "TUE"),
            (Self::WED, "WED"),
            (Self::THU, "THU"),
            (Self::FRI, "FRI"),
            (Self::SAT, "SAT"),
        ]
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DaysOfWeek {
    // `#[classattr]` and `#[staticmethod]` are inner attributes of
    // `#[pymethods]`, so they cannot be written behind `cfg_attr` and these
    // cannot join the shared impl above. Distinct Rust names avoid colliding
    // with the consts and constructor they re-export.
    #[classattr]
    #[pyo3(name = "NONE")]
    const PY_NONE: DaysOfWeek = DaysOfWeek::NONE;
    #[classattr]
    #[pyo3(name = "SUN")]
    const PY_SUN: DaysOfWeek = DaysOfWeek::SUN;
    #[classattr]
    #[pyo3(name = "MON")]
    const PY_MON: DaysOfWeek = DaysOfWeek::MON;
    #[classattr]
    #[pyo3(name = "TUE")]
    const PY_TUE: DaysOfWeek = DaysOfWeek::TUE;
    #[classattr]
    #[pyo3(name = "WED")]
    const PY_WED: DaysOfWeek = DaysOfWeek::WED;
    #[classattr]
    #[pyo3(name = "THU")]
    const PY_THU: DaysOfWeek = DaysOfWeek::THU;
    #[classattr]
    #[pyo3(name = "FRI")]
    const PY_FRI: DaysOfWeek = DaysOfWeek::FRI;
    #[classattr]
    #[pyo3(name = "SAT")]
    const PY_SAT: DaysOfWeek = DaysOfWeek::SAT;
    #[classattr]
    #[pyo3(name = "WEEKDAYS")]
    const PY_WEEKDAYS: DaysOfWeek = DaysOfWeek::WEEKDAYS;
    #[classattr]
    #[pyo3(name = "WEEKEND")]
    const PY_WEEKEND: DaysOfWeek = DaysOfWeek::WEEKEND;
    #[classattr]
    #[pyo3(name = "EVERY_DAY")]
    const PY_EVERY_DAY: DaysOfWeek = DaysOfWeek::EVERY_DAY;

    #[staticmethod]
    #[pyo3(name = "from_bits_truncate")]
    fn py_from_bits_truncate(bits: u8) -> Self {
        Self::from_bits_truncate(bits)
    }

    fn __or__(&self, other: &Self) -> Self {
        *self | *other
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

impl fmt::Debug for DaysOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DaysOfWeek(")?;

        if self.is_empty() {
            f.write_str("NONE")?;
        } else {
            let mut first = true;
            for (day, name) in Self::all_days() {
                if self.contains(day) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    f.write_str(name)?;
                    first = false;
                }
            }
        }

        f.write_str(")")
    }
}

impl<'de> Deserialize<'de> for DaysOfWeek {
    /// Truncates rather than failing, so a firmware that sets a bit above
    /// Saturday cannot produce a set that would be sent back to the device.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_bits_truncate(u8::deserialize(deserializer)?))
    }
}

impl BitOr for DaysOfWeek {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for DaysOfWeek {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// When a schedule rule fires within a day.
///
/// Each variant carries only the fields that apply to it, so a clock rule
/// cannot hold a sunrise offset and vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, eq, hash, frozen)
)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleTime {
    /// At a wall-clock time, in the device's own timezone.
    Clock {
        /// Hour of the day, `0..=23`.
        hour: u8,
        /// Minute of the hour, `0..=59`.
        minute: u8,
    },
    /// At an offset from civil sunrise, computed by the device.
    Sunrise {
        /// Minutes from sunrise; negative fires before it.
        offset_minutes: i16,
    },
    /// At an offset from civil sunset, computed by the device.
    Sunset {
        /// Minutes from sunset; negative fires before it.
        offset_minutes: i16,
    },
}

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
    id: Option<String>,
    enabled: bool,
    time: ScheduleTime,
    /// `None` fires once; `Some(days)` repeats weekly on those days.
    days: Option<DaysOfWeek>,
    desired_state: PowerState,
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

/// Wire shape of a schedule rule. Mirrors `ThingRuleSchedule` from the
/// official Tapo Android app, and stays private so the device format is not
/// part of the public API.
///
/// `year` / `month` / `day` are required by the API but their values are
/// ignored by the device (verified on a P110). `s_type` determines whether
/// `s_min` is a clock minute-of-day or `time_offset` is a sunrise / sunset
/// offset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ScheduleRuleRaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    enable: bool,
    year: i32,
    month: u8,
    day: u8,
    time_offset: i32,
    week_day: u8,
    s_min: i32,
    e_min: i32,
    s_type: String,
    e_type: String,
    e_action: String,
    mode: String,
    #[serde(default)]
    desired_states: Option<DesiredStateRaw>,
    /// Deprecated mirror of the firing state, still emitted by some
    /// firmwares. Used as a fallback when `desired_states.on` is absent.
    /// See `ThingRuleSchedule.startAction` in the Tapo app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    s_action: Option<String>,
}

/// Constant placeholder for the wire `year` / `month` / `day` (device ignores these).
const PLACEHOLDER_DATE: (i32, u8, u8) = (1970, 1, 1);

impl From<&ScheduleRule> for ScheduleRuleRaw {
    fn from(rule: &ScheduleRule) -> Self {
        let (s_type, time_offset, s_min) = match rule.time {
            ScheduleTime::Clock { hour, minute } => {
                ("normal", 0_i32, i32::from(hour) * 60 + i32::from(minute))
            }
            ScheduleTime::Sunrise { offset_minutes } => ("sunrise", i32::from(offset_minutes), 0),
            ScheduleTime::Sunset { offset_minutes } => ("sunset", i32::from(offset_minutes), 0),
        };
        let (mode, week_day) = match rule.days {
            None => ("once", 0),
            Some(days) => ("repeat", days.bits()),
        };
        let (year, month, day) = PLACEHOLDER_DATE;

        ScheduleRuleRaw {
            id: rule.id.clone(),
            enable: rule.enabled,
            year,
            month,
            day,
            time_offset,
            week_day,
            s_min,
            e_min: 0,
            s_type: s_type.into(),
            e_type: "normal".into(),
            e_action: "none".into(),
            mode: mode.into(),
            desired_states: Some(DesiredStateRaw::new(rule.desired_state)),
            s_action: None,
        }
    }
}

impl TryFrom<ScheduleRuleRaw> for ScheduleRuleResult {
    type Error = String;

    fn try_from(raw: ScheduleRuleRaw) -> Result<Self, Self::Error> {
        let time = match raw.s_type.as_str() {
            "normal" => {
                let minute_of_day = u16::try_from(raw.s_min)
                    .map_err(|_| format!("s_min {} out of range", raw.s_min))?;
                if minute_of_day >= 24 * 60 {
                    return Err(format!("s_min {minute_of_day} is not a minute of the day"));
                }

                ScheduleTime::Clock {
                    hour: (minute_of_day / 60) as u8,
                    minute: (minute_of_day % 60) as u8,
                }
            }
            "sunrise" | "sunset" => {
                let offset_minutes = i16::try_from(raw.time_offset)
                    .map_err(|_| format!("time_offset {} out of range", raw.time_offset))?;

                if raw.s_type == "sunrise" {
                    ScheduleTime::Sunrise { offset_minutes }
                } else {
                    ScheduleTime::Sunset { offset_minutes }
                }
            }
            other => return Err(format!("unknown schedule s_type {other:?}")),
        };

        let days = match raw.mode.as_str() {
            "once" => None,
            "repeat" => Some(DaysOfWeek::from_bits_truncate(raw.week_day)),
            other => return Err(format!("unknown schedule mode {other:?}")),
        };

        let desired_state = raw
            .desired_states
            .unwrap_or_default()
            .resolve(raw.s_action.as_deref())
            .ok_or_else(|| {
                "neither desired_states.on nor s_action contained a recognised firing state"
                    .to_string()
            })?;

        Ok(ScheduleRuleResult {
            id: raw.id.ok_or_else(|| "the rule has no id".to_string())?,
            enabled: raw.enable,
            time,
            days,
            desired_state,
        })
    }
}

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

    fn wire_json(rule: &ScheduleRule) -> serde_json::Value {
        serde_json::to_value(ScheduleRuleRaw::from(rule)).expect("serialize")
    }

    fn clock_once(hour: u8, minute: u8, desired_state: PowerState) -> ScheduleRule {
        ScheduleRule::clock_once(hour, minute, desired_state).expect("valid clock_once")
    }

    fn clock_weekly(
        hour: u8,
        minute: u8,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> ScheduleRule {
        ScheduleRule::clock_weekly(hour, minute, days, desired_state).expect("valid clock_weekly")
    }

    fn parse(raw: serde_json::Value) -> Result<ScheduleRuleResult, String> {
        let raw: ScheduleRuleRaw = serde_json::from_value(raw).map_err(|e| e.to_string())?;
        ScheduleRuleResult::try_from(raw)
    }

    fn raw_json(overrides: serde_json::Value) -> serde_json::Value {
        let mut base = serde_json::json!({
            "id": "S1", "enable": true, "year": 1970, "month": 1, "day": 1,
            "time_offset": 0, "week_day": 0, "s_min": 0, "e_min": 0,
            "s_type": "normal", "e_type": "normal", "e_action": "none",
            "mode": "once", "desired_states": { "on": true }
        });
        let map = base.as_object_mut().expect("object");
        for (key, value) in overrides.as_object().expect("object") {
            if value.is_null() && key != "desired_states" {
                map.remove(key);
            } else {
                map.insert(key.clone(), value.clone());
            }
        }
        base
    }

    #[test]
    fn days_of_week_combine_and_contain() {
        let midweek = DaysOfWeek::MON | DaysOfWeek::WED;
        assert_eq!(midweek.bits(), 0b0000_1010);
        assert!(midweek.contains(DaysOfWeek::MON));
        assert!(!midweek.contains(DaysOfWeek::TUE));
        assert!(!midweek.is_empty());
        assert!(DaysOfWeek::NONE.is_empty());

        let mut acc = DaysOfWeek::SUN;
        acc |= DaysOfWeek::SAT;
        assert_eq!(acc, DaysOfWeek::WEEKEND);

        assert_eq!(DaysOfWeek::WEEKDAYS.bits(), 0b0011_1110);
        assert_eq!(DaysOfWeek::EVERY_DAY.bits(), 0b0111_1111);
    }

    #[test]
    fn days_of_week_from_device_ignores_high_bits() {
        assert_eq!(
            DaysOfWeek::from_bits_truncate(0b1000_1010),
            DaysOfWeek::MON | DaysOfWeek::WED
        );
    }

    #[test]
    fn days_of_week_deserialize_cannot_smuggle_high_bits() {
        let days: DaysOfWeek = serde_json::from_str("255").expect("deserialize");
        assert_eq!(days, DaysOfWeek::EVERY_DAY);
    }

    #[test]
    fn days_of_week_equal_values_hash_equally() {
        use std::collections::HashSet;

        // Equal sets built by different routes must collapse to one entry,
        // which is what the pyclass `hash` option relies on.
        let mut set = HashSet::new();
        set.insert(DaysOfWeek::MON | DaysOfWeek::WED);
        set.insert(DaysOfWeek::WED | DaysOfWeek::MON);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&DaysOfWeek::from_bits_truncate(0b0000_1010)));
    }

    #[test]
    fn days_of_week_debug_names_the_days() {
        assert_eq!(
            format!("{:?}", DaysOfWeek::MON | DaysOfWeek::WED),
            "DaysOfWeek(MON | WED)"
        );
        assert_eq!(format!("{:?}", DaysOfWeek::NONE), "DaysOfWeek(NONE)");
    }

    #[test]
    fn schedule_time_equal_values_hash_equally() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ScheduleTime::Clock {
            hour: 6,
            minute: 30,
        });
        set.insert(ScheduleTime::Clock {
            hour: 6,
            minute: 30,
        });
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn clock_once_wire_shape() {
        let j = wire_json(&clock_once(6, 30, PowerState::On));
        assert_eq!(j["s_type"], "normal");
        assert_eq!(j["mode"], "once");
        assert_eq!(j["s_min"], 6 * 60 + 30);
        assert_eq!(j["time_offset"], 0);
        assert_eq!(j["week_day"], 0);
        assert_eq!(j["enable"], true);
        assert_eq!(j["desired_states"], serde_json::json!({ "on": true }));
        assert_eq!(j["year"], 1970);
        assert_eq!(j["month"], 1);
        assert_eq!(j["day"], 1);
        assert!(j.get("id").is_none());
    }

    #[test]
    fn clock_weekly_wire_shape() {
        let j = wire_json(&clock_weekly(
            23,
            30,
            DaysOfWeek::MON | DaysOfWeek::WED,
            PowerState::Off,
        ));
        assert_eq!(j["mode"], "repeat");
        assert_eq!(j["s_min"], 23 * 60 + 30);
        assert_eq!(j["week_day"], 0b0000_1010);
        assert_eq!(j["desired_states"], serde_json::json!({ "on": false }));
    }

    #[test]
    fn sun_rules_wire_shape() {
        let j = wire_json(
            &ScheduleRule::sunrise_weekly(-30, DaysOfWeek::WEEKDAYS, PowerState::Off)
                .expect("valid"),
        );
        assert_eq!(j["s_type"], "sunrise");
        assert_eq!(j["time_offset"], -30);
        assert_eq!(j["s_min"], 0);
        assert_eq!(j["week_day"], 0b0011_1110);

        let j = wire_json(&ScheduleRule::sunset_once(60, PowerState::On).expect("valid"));
        assert_eq!(j["s_type"], "sunset");
        assert_eq!(j["mode"], "once");
        assert_eq!(j["time_offset"], 60);
    }

    #[test]
    fn public_serialization_is_not_the_wire_shape() {
        // The device format stays private: a result serializes as its own
        // documented fields, not `s_type` / `s_min` / the 1970 placeholders.
        let result = parse(raw_json(serde_json::json!({ "s_min": 390 }))).expect("parse");
        let j = serde_json::to_value(&result).expect("serialize");
        assert_eq!(
            j,
            serde_json::json!({
                "id": "S1",
                "enabled": true,
                "time": { "clock": { "hour": 6, "minute": 30 } },
                "days": null,
                "desired_state": "on"
            })
        );
    }

    #[test]
    fn result_round_trips_through_to_editable() {
        let original = clock_weekly(8, 5, DaysOfWeek::EVERY_DAY, PowerState::On);
        let raw = ScheduleRuleRaw::from(&original.with_id("S7"));
        let result = ScheduleRuleResult::try_from(raw).expect("parse");

        assert_eq!(result.id, "S7");
        assert_eq!(result.time, ScheduleTime::Clock { hour: 8, minute: 5 });
        assert_eq!(result.days, Some(DaysOfWeek::EVERY_DAY));

        let editable = result.to_editable().expect("valid");
        assert_eq!(editable, original.with_id("S7"));
    }

    #[test]
    fn to_editable_rejects_what_the_builders_would() {
        // A repeating rule the device stored with no days set is readable but
        // not writable, which is why the conversion is fallible.
        let result = parse(raw_json(
            serde_json::json!({ "mode": "repeat", "week_day": 0 }),
        ))
        .expect("parse");
        assert_eq!(result.days, Some(DaysOfWeek::NONE));

        let err = result.to_editable().expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "days"));
    }

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
    fn with_enabled_clones_and_overrides() {
        let rule = clock_weekly(8, 0, DaysOfWeek::MON, PowerState::On).with_id("S42");
        let disabled = rule.with_enabled(false);
        assert_eq!(wire_json(&rule)["enable"], true);
        assert_eq!(wire_json(&disabled)["enable"], false);
        assert_eq!(disabled.id(), Some("S42"));
    }

    #[test]
    fn parse_rejects_unrepresentable_rules() {
        for (overrides, expected) in [
            (serde_json::json!({ "s_min": 5000 }), "minute of the day"),
            (serde_json::json!({ "s_type": "moonrise" }), "moonrise"),
            (serde_json::json!({ "mode": "fortnightly" }), "fortnightly"),
            (serde_json::json!({ "id": null }), "no id"),
        ] {
            let err = parse(raw_json(overrides)).expect_err("should reject");
            assert!(err.contains(expected), "{err} should mention {expected}");
        }
    }

    #[test]
    fn parse_falls_back_to_s_action_and_tolerates_null_desired_states() {
        // A firmware that sends an explicit null, or omits the key, still
        // parses through the legacy `s_action` field.
        for desired_states in [serde_json::Value::Null, serde_json::json!({})] {
            let rule = parse(raw_json(serde_json::json!({
                "desired_states": desired_states,
                "s_action": "off",
                "s_min": 90
            })))
            .expect("parse");
            assert_eq!(rule.desired_state, PowerState::Off);
            assert_eq!(
                rule.time,
                ScheduleTime::Clock {
                    hour: 1,
                    minute: 30
                }
            );
        }

        // `desired_states` wins when both are present.
        let rule = parse(raw_json(serde_json::json!({ "s_action": "off" }))).expect("parse");
        assert_eq!(rule.desired_state, PowerState::On);

        let err = parse(raw_json(serde_json::json!({ "desired_states": null })))
            .expect_err("should reject");
        assert!(err.contains("recognised firing state"));
    }

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
