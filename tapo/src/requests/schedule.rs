//! Request types for the plug "Schedule" feature.

use std::fmt;
use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DesiredStateRaw, PowerState};

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// The days of the week a weekly [`ScheduleRule`] fires on.
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
#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "python", pyclass(from_py_object, eq, frozen))]
#[serde(transparent)]
pub struct DaysOfWeek(u8);

impl DaysOfWeek {
    /// No days. A weekly rule with no days would never fire, so the
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

    /// Returns the device bitmask for this set: bit 0 is Sunday
    /// through bit 6, Saturday.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Returns `true` if every day in `other` is also in this set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns `true` if this set contains no days.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

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

    /// The individual days in this set, Sunday first, paired with their names.
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

/// When a [`ScheduleRule`] fires within a day.
///
/// Each variant carries only the fields that apply to it, so a clock rule
/// cannot hold a sunrise offset and vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "python", pyclass(from_py_object, eq))]
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

/// Whether a [`ScheduleRule`] fires once or repeats weekly.
///
/// Python has no sum types, so the bindings flatten this onto
/// `ScheduleRule.days`, which is `None` for a one-shot rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleFrequency {
    /// Fires once, at the next matching time.
    Once,
    /// Fires every week, on the given days.
    Weekly {
        /// The days the rule fires on.
        days: DaysOfWeek,
    },
}

/// A plug schedule rule (the "Schedule" feature in the Tapo app).
///
/// Construct one with the builders ([`ScheduleRule::clock_weekly`],
/// [`ScheduleRule::sunrise_once`], …); each returns `Result<Self, Error>`
/// and reports an `Error::Validation` for out-of-range inputs. The wire
/// representation is filled in on serialization.
///
/// The device evaluates the time against its own configured timezone; you
/// don't supply a calendar date. The on-the-wire `year` / `month` / `day`
/// fields the device requires are filled with a constant placeholder
/// (`1970-01-01`) because the device ignores their values — this was
/// confirmed experimentally on a P110: a `clock_once` sent with
/// `year=1970, month=1, day=1` still fires at the requested HH:MM.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(from_py_object, frozen))]
#[serde(try_from = "ScheduleRuleRaw", into = "ScheduleRuleRaw")]
pub struct ScheduleRule {
    /// Device-assigned id. `None` when the rule was constructed locally;
    /// `Some` when read back from the device.
    pub id: Option<String>,
    /// Whether the rule is currently active. Disabled rules are kept on
    /// the device but do not fire.
    pub enabled: bool,
    /// When the rule fires within a day.
    pub time: ScheduleTime,
    /// Once, or weekly on a set of days.
    pub frequency: ScheduleFrequency,
    /// The state the plug transitions to when the rule fires.
    pub desired_state: PowerState,
}

/// The largest sunrise / sunset offset the builders accept, in minutes.
const MAX_OFFSET_MINUTES: i16 = 1440;

impl ScheduleRule {
    fn new(
        time: ScheduleTime,
        frequency: ScheduleFrequency,
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
                            "Must be within -{MAX_OFFSET_MINUTES}..={MAX_OFFSET_MINUTES} minutes (±24h), got {offset_minutes}",
                        ),
                    });
                }
            }
        }

        if let ScheduleFrequency::Weekly { days } = frequency
            && days.is_empty()
        {
            return Err(Error::Validation {
                field: "days".to_string(),
                message: "A weekly rule must fire on at least one day".to_string(),
            });
        }

        Ok(Self {
            id: None,
            enabled: true,
            time,
            frequency,
            desired_state,
        })
    }

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
            ScheduleFrequency::Weekly { days },
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
        Self::new(
            ScheduleTime::Clock { hour, minute },
            ScheduleFrequency::Once,
            desired_state,
        )
    }

    /// Fires every week, on `days`, at `offset_minutes` from sunrise.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunrise, `-1440..=1440`; negative
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
            ScheduleFrequency::Weekly { days },
            desired_state,
        )
    }

    /// Fires once, at the next sunrise plus `offset_minutes`.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunrise, `-1440..=1440`; negative
    ///   fires before it.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn sunrise_once(offset_minutes: i16, desired_state: PowerState) -> Result<Self, Error> {
        Self::new(
            ScheduleTime::Sunrise { offset_minutes },
            ScheduleFrequency::Once,
            desired_state,
        )
    }

    /// Fires every week, on `days`, at `offset_minutes` from sunset.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunset, `-1440..=1440`; negative
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
            ScheduleFrequency::Weekly { days },
            desired_state,
        )
    }

    /// Fires once, at the next sunset plus `offset_minutes`.
    ///
    /// # Arguments
    ///
    /// * `offset_minutes` - minutes from sunset, `-1440..=1440`; negative
    ///   fires before it.
    /// * `desired_state` - the state the plug transitions to when the rule fires.
    pub fn sunset_once(offset_minutes: i16, desired_state: PowerState) -> Result<Self, Error> {
        Self::new(
            ScheduleTime::Sunset { offset_minutes },
            ScheduleFrequency::Once,
            desired_state,
        )
    }

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

    /// Returns a copy of this rule with `id` set to the given value.
    /// Use before `edit_schedule_rule` (on
    /// [`PlugHandler`](crate::PlugHandler) or
    /// [`PlugEnergyMonitoringHandler`](crate::PlugEnergyMonitoringHandler))
    /// when reconstructing an edit from scratch.
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
}

/// Wire shape of a schedule rule, used for (de)serialization and as the
/// parameters of the add / edit requests. Mirrors `ThingRuleSchedule` from
/// the official Tapo Android app.
///
/// `year` / `month` / `day` are required by the API but their values are
/// ignored by the device (verified on a P110). `s_type` determines whether
/// `s_min` is a clock minute-of-day or `time_offset` is a sunrise / sunset
/// offset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ScheduleRuleRaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
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
    desired_states: DesiredStateRaw,
    /// Deprecated mirror of the firing state, still emitted by some
    /// firmwares. Used as a fallback when `desired_states.on` is absent.
    /// See `ThingRuleSchedule.startAction` in the Tapo app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    s_action: Option<String>,
}

/// Constant placeholder for the wire `year` / `month` / `day` (device ignores these).
const PLACEHOLDER_DATE: (i32, u8, u8) = (1970, 1, 1);

impl From<ScheduleRule> for ScheduleRuleRaw {
    fn from(rule: ScheduleRule) -> Self {
        let (s_type, time_offset, s_min) = match rule.time {
            ScheduleTime::Clock { hour, minute } => {
                ("normal", 0_i32, i32::from(hour) * 60 + i32::from(minute))
            }
            ScheduleTime::Sunrise { offset_minutes } => ("sunrise", i32::from(offset_minutes), 0),
            ScheduleTime::Sunset { offset_minutes } => ("sunset", i32::from(offset_minutes), 0),
        };
        let (mode, week_day) = match rule.frequency {
            ScheduleFrequency::Once => ("once", 0),
            ScheduleFrequency::Weekly { days } => ("repeat", days.bits()),
        };
        let (year, month, day) = PLACEHOLDER_DATE;

        ScheduleRuleRaw {
            id: rule.id,
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
            desired_states: DesiredStateRaw::new(rule.desired_state),
            s_action: None,
        }
    }
}

impl TryFrom<ScheduleRuleRaw> for ScheduleRule {
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

        let frequency = match raw.mode.as_str() {
            "once" => ScheduleFrequency::Once,
            "repeat" => ScheduleFrequency::Weekly {
                days: DaysOfWeek::from_bits_truncate(raw.week_day),
            },
            other => return Err(format!("unknown schedule mode {other:?}")),
        };

        let desired_state = raw
            .desired_states
            .resolve(raw.s_action.as_deref())
            .ok_or_else(|| {
                "neither desired_states.on nor s_action contained a recognised firing state"
                    .to_string()
            })?;

        Ok(ScheduleRule {
            id: raw.id,
            enabled: raw.enable,
            time,
            frequency,
            desired_state,
        })
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl DaysOfWeek {
    #[classattr]
    #[pyo3(name = "NONE")]
    fn py_none() -> Self {
        Self::NONE
    }

    #[classattr]
    #[pyo3(name = "SUN")]
    fn py_sun() -> Self {
        Self::SUN
    }

    #[classattr]
    #[pyo3(name = "MON")]
    fn py_mon() -> Self {
        Self::MON
    }

    #[classattr]
    #[pyo3(name = "TUE")]
    fn py_tue() -> Self {
        Self::TUE
    }

    #[classattr]
    #[pyo3(name = "WED")]
    fn py_wed() -> Self {
        Self::WED
    }

    #[classattr]
    #[pyo3(name = "THU")]
    fn py_thu() -> Self {
        Self::THU
    }

    #[classattr]
    #[pyo3(name = "FRI")]
    fn py_fri() -> Self {
        Self::FRI
    }

    #[classattr]
    #[pyo3(name = "SAT")]
    fn py_sat() -> Self {
        Self::SAT
    }

    #[classattr]
    #[pyo3(name = "WEEKDAYS")]
    fn py_weekdays() -> Self {
        Self::WEEKDAYS
    }

    #[classattr]
    #[pyo3(name = "WEEKEND")]
    fn py_weekend() -> Self {
        Self::WEEKEND
    }

    #[classattr]
    #[pyo3(name = "EVERY_DAY")]
    fn py_every_day() -> Self {
        Self::EVERY_DAY
    }

    #[staticmethod]
    #[pyo3(name = "from_bits_truncate")]
    fn py_from_bits_truncate(bits: u8) -> Self {
        Self::from_bits_truncate(bits)
    }

    #[pyo3(name = "bits")]
    fn py_bits(&self) -> u8 {
        self.bits()
    }

    #[pyo3(name = "contains")]
    fn py_contains(&self, other: &Self) -> bool {
        self.contains(*other)
    }

    fn __or__(&self, other: &Self) -> Self {
        *self | *other
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl ScheduleRule {
    #[getter]
    fn id(&self) -> Option<String> {
        self.id.clone()
    }

    #[getter]
    fn enabled(&self) -> bool {
        self.enabled
    }

    #[getter]
    fn time(&self) -> ScheduleTime {
        self.time
    }

    /// The days a weekly rule fires on, or `None` when it fires once.
    #[getter]
    fn days(&self) -> Option<DaysOfWeek> {
        match self.frequency {
            ScheduleFrequency::Once => None,
            ScheduleFrequency::Weekly { days } => Some(days),
        }
    }

    #[getter]
    fn desired_state(&self) -> PowerState {
        self.desired_state
    }

    #[staticmethod]
    #[pyo3(name = "clock_weekly")]
    fn py_clock_weekly(
        hour: u8,
        minute: u8,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> PyResult<Self> {
        Ok(Self::clock_weekly(hour, minute, days, desired_state)?)
    }

    #[staticmethod]
    #[pyo3(name = "clock_once")]
    fn py_clock_once(hour: u8, minute: u8, desired_state: PowerState) -> PyResult<Self> {
        Ok(Self::clock_once(hour, minute, desired_state)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunrise_weekly")]
    fn py_sunrise_weekly(
        offset_minutes: i16,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> PyResult<Self> {
        Ok(Self::sunrise_weekly(offset_minutes, days, desired_state)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunrise_once")]
    fn py_sunrise_once(offset_minutes: i16, desired_state: PowerState) -> PyResult<Self> {
        Ok(Self::sunrise_once(offset_minutes, desired_state)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunset_weekly")]
    fn py_sunset_weekly(
        offset_minutes: i16,
        days: DaysOfWeek,
        desired_state: PowerState,
    ) -> PyResult<Self> {
        Ok(Self::sunset_weekly(offset_minutes, days, desired_state)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunset_once")]
    fn py_sunset_once(offset_minutes: i16, desired_state: PowerState) -> PyResult<Self> {
        Ok(Self::sunset_once(offset_minutes, desired_state)?)
    }

    #[pyo3(name = "with_enabled")]
    fn py_with_enabled(&self, enabled: bool) -> Self {
        self.with_enabled(enabled)
    }

    #[pyo3(name = "with_id")]
    fn py_with_id(&self, id: String) -> Self {
        self.with_id(id)
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    /// Returns the user-facing fields of this rule as a Python dictionary.
    ///
    /// Mirrors the names exposed by attribute access (`time`, `days`, …)
    /// rather than the wire shape used for transport (`s_type`, `s_min`, …).
    fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(DictRule::from(self))
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
        crate::python::serde_object_to_py_dict(py, &value)
    }
}

/// User-facing dict shape for `ScheduleRule::to_dict`. Matches the public
/// fields of [`ScheduleRule`] (and the `get_all` attributes exposed to
/// Python) instead of the on-the-wire [`ScheduleRuleRaw`].
#[cfg(feature = "python")]
#[derive(Serialize)]
struct DictRule<'a> {
    id: &'a Option<String>,
    enabled: bool,
    time: ScheduleTime,
    days: Option<DaysOfWeek>,
    desired_state: PowerState,
}

#[cfg(feature = "python")]
impl<'a> From<&'a ScheduleRule> for DictRule<'a> {
    fn from(rule: &'a ScheduleRule) -> Self {
        DictRule {
            id: &rule.id,
            enabled: rule.enabled,
            time: rule.time,
            days: match rule.frequency {
                ScheduleFrequency::Once => None,
                ScheduleFrequency::Weekly { days } => Some(days),
            },
            desired_state: rule.desired_state,
        }
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
        serde_json::to_value(rule).expect("serialize")
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

    #[test]
    fn days_of_week_combine_and_contain() {
        let midweek = DaysOfWeek::MON | DaysOfWeek::WED;
        assert_eq!(midweek.bits(), 0b0000_1010);
        assert!(midweek.contains(DaysOfWeek::MON));
        assert!(midweek.contains(DaysOfWeek::WED));
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
        // The bit-7+ guard is the type's invariant, so the one remaining way
        // in — deserializing a bitmask directly — has to honour it too.
        let days: DaysOfWeek = serde_json::from_str("255").expect("deserialize");
        assert_eq!(days, DaysOfWeek::EVERY_DAY);
        assert_eq!(days.bits(), DaysOfWeek::EVERY_DAY.bits());
    }

    #[test]
    fn days_of_week_debug_names_the_days() {
        assert_eq!(
            format!("{:?}", DaysOfWeek::MON | DaysOfWeek::WED),
            "DaysOfWeek(MON | WED)"
        );
        assert_eq!(format!("{:?}", DaysOfWeek::NONE), "DaysOfWeek(NONE)");
        assert_eq!(
            format!("{:?}", DaysOfWeek::EVERY_DAY),
            "DaysOfWeek(SUN | MON | TUE | WED | THU | FRI | SAT)"
        );
    }

    #[test]
    fn clock_once_wire_shape() {
        let r = clock_once(6, 30, PowerState::On);
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "normal");
        assert_eq!(j["mode"], "once");
        assert_eq!(j["s_min"], 6 * 60 + 30);
        assert_eq!(j["time_offset"], 0);
        assert_eq!(j["week_day"], 0);
        assert_eq!(j["enable"], true);
        assert_eq!(j["desired_states"], serde_json::json!({ "on": true }));
        // Date placeholder is constant per the ScheduleRuleRaw contract.
        assert_eq!(j["year"], 1970);
        assert_eq!(j["month"], 1);
        assert_eq!(j["day"], 1);
        // No id field when constructed locally.
        assert!(j.get("id").is_none());
    }

    #[test]
    fn clock_weekly_wire_shape() {
        let r = clock_weekly(23, 30, DaysOfWeek::MON | DaysOfWeek::WED, PowerState::Off);
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "normal");
        assert_eq!(j["mode"], "repeat");
        assert_eq!(j["s_min"], 23 * 60 + 30);
        assert_eq!(j["week_day"], 0b0000_1010);
        assert_eq!(j["desired_states"], serde_json::json!({ "on": false }));
    }

    #[test]
    fn sunrise_weekly_wire_shape() {
        let r = ScheduleRule::sunrise_weekly(-30, DaysOfWeek::WEEKDAYS, PowerState::Off)
            .expect("valid sunrise_weekly");
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "sunrise");
        assert_eq!(j["mode"], "repeat");
        assert_eq!(j["time_offset"], -30);
        assert_eq!(j["s_min"], 0);
        assert_eq!(j["week_day"], 0b0011_1110);
    }

    #[test]
    fn sunset_once_wire_shape() {
        let r = ScheduleRule::sunset_once(60, PowerState::On).expect("valid sunset_once");
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "sunset");
        assert_eq!(j["mode"], "once");
        assert_eq!(j["time_offset"], 60);
        assert_eq!(j["week_day"], 0);
    }

    #[test]
    fn round_trip_via_wire() {
        let original = clock_weekly(8, 5, DaysOfWeek::EVERY_DAY, PowerState::On);
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: ScheduleRule = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, original);
        assert_eq!(parsed.time, ScheduleTime::Clock { hour: 8, minute: 5 });
        assert_eq!(
            parsed.frequency,
            ScheduleFrequency::Weekly {
                days: DaysOfWeek::EVERY_DAY
            }
        );
    }

    #[test]
    fn round_trip_preserves_device_id() {
        let original = clock_weekly(8, 0, DaysOfWeek::MON, PowerState::On).with_id("S42");
        let json = serde_json::to_string(&original).expect("serialize");
        assert!(json.contains("\"id\":\"S42\""));
        let parsed: ScheduleRule = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed.id.as_deref(), Some("S42"));
    }

    #[test]
    fn sun_rules_round_trip_offsets() {
        for original in [
            ScheduleRule::sunrise_once(-1440, PowerState::Off).expect("valid"),
            ScheduleRule::sunset_weekly(1440, DaysOfWeek::WEEKEND, PowerState::On).expect("valid"),
        ] {
            let json = serde_json::to_string(&original).expect("serialize");
            let parsed: ScheduleRule = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, original);
        }
    }

    #[test]
    fn with_enabled_clones_and_overrides() {
        let r = clock_weekly(8, 0, DaysOfWeek::MON, PowerState::On).with_id("S42");
        let disabled = r.with_enabled(false);
        assert!(r.enabled);
        assert!(!disabled.enabled);
        assert_eq!(disabled.id.as_deref(), Some("S42"));
        assert_eq!(wire_json(&disabled)["enable"], false);
    }

    #[test]
    fn deserialize_rejects_bad_s_min() {
        let raw = serde_json::json!({
            "enable": true, "year": 1970, "month": 1, "day": 1,
            "time_offset": 0, "week_day": 0, "s_min": 5000, "e_min": 0,
            "s_type": "normal", "e_type": "normal", "e_action": "none",
            "mode": "once", "desired_states": { "on": true }
        });
        let err = serde_json::from_value::<ScheduleRule>(raw).expect_err("should reject");
        assert!(err.to_string().contains("minute of the day"));
    }

    #[test]
    fn deserialize_rejects_unknown_s_type() {
        let raw = serde_json::json!({
            "enable": true, "year": 1970, "month": 1, "day": 1,
            "time_offset": 0, "week_day": 0, "s_min": 0, "e_min": 0,
            "s_type": "moonrise", "e_type": "normal", "e_action": "none",
            "mode": "once", "desired_states": { "on": true }
        });
        let err = serde_json::from_value::<ScheduleRule>(raw).expect_err("should reject");
        assert!(err.to_string().contains("moonrise"));
    }

    #[test]
    fn clock_weekly_rejects_bad_hour() {
        let err = ScheduleRule::clock_weekly(24, 0, DaysOfWeek::MON, PowerState::On)
            .expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "hour"));
    }

    #[test]
    fn clock_weekly_rejects_bad_minute() {
        let err = ScheduleRule::clock_weekly(0, 60, DaysOfWeek::MON, PowerState::On)
            .expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "minute"));
    }

    #[test]
    fn weekly_rejects_empty_days() {
        let err = ScheduleRule::clock_weekly(8, 0, DaysOfWeek::NONE, PowerState::On)
            .expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "days"));

        let err = ScheduleRule::sunset_weekly(0, DaysOfWeek::NONE, PowerState::On)
            .expect_err("should reject");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "days"));
    }

    #[test]
    fn sunrise_rejects_huge_offset() {
        let err =
            ScheduleRule::sunrise_once(1441, PowerState::On).expect_err("should reject offset");
        assert!(matches!(err, Error::Validation { ref field, .. } if field == "offset_minutes"));
    }

    #[test]
    fn deserialize_falls_back_to_s_action_when_desired_states_absent() {
        let raw = serde_json::json!({
            "enable": true, "year": 1970, "month": 1, "day": 1,
            "time_offset": 0, "week_day": 0, "s_min": 90, "e_min": 0,
            "s_type": "normal", "e_type": "normal", "e_action": "none",
            "mode": "once", "s_action": "off"
        });
        let rule: ScheduleRule = serde_json::from_value(raw).expect("deserialize");
        assert_eq!(rule.desired_state, PowerState::Off);
        assert_eq!(
            rule.time,
            ScheduleTime::Clock {
                hour: 1,
                minute: 30
            }
        );
    }

    #[test]
    fn deserialize_prefers_desired_states_over_s_action() {
        let raw = serde_json::json!({
            "enable": true, "year": 1970, "month": 1, "day": 1,
            "time_offset": 0, "week_day": 0, "s_min": 0, "e_min": 0,
            "s_type": "normal", "e_type": "normal", "e_action": "none",
            "mode": "once", "desired_states": { "on": true }, "s_action": "off"
        });
        let rule: ScheduleRule = serde_json::from_value(raw).expect("deserialize");
        assert_eq!(rule.desired_state, PowerState::On);
    }

    #[test]
    fn deserialize_rejects_missing_firing_state() {
        let raw = serde_json::json!({
            "enable": true, "year": 1970, "month": 1, "day": 1,
            "time_offset": 0, "week_day": 0, "s_min": 0, "e_min": 0,
            "s_type": "normal", "e_type": "normal", "e_action": "none",
            "mode": "once"
        });
        let err = serde_json::from_value::<ScheduleRule>(raw).expect_err("should reject");
        assert!(err.to_string().contains("recognised firing state"));
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
