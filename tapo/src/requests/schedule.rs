//! Request types for the plug "Schedule" feature.

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// When a [`ScheduleRule`] fires within a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq, eq_int, from_py_object))]
#[serde(rename_all = "lowercase")]
pub enum ScheduleTimeKind {
    /// At a wall-clock time (minutes after midnight, device local time).
    Clock,
    /// At a fixed offset from civil sunrise.
    Sunrise,
    /// At a fixed offset from civil sunset.
    Sunset,
}

/// Whether a [`ScheduleRule`] fires once or repeats weekly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq, eq_int, from_py_object))]
#[serde(rename_all = "lowercase")]
pub enum ScheduleFrequency {
    /// Fires once, at the next matching time.
    Once,
    /// Fires every day matched by the rule's `week_day` bitmask.
    Weekly,
}

/// Day-of-week bit constants for the `week_day` argument of
/// [`ScheduleRule::clock_weekly`] and the other `*_weekly` builders.
/// Bit 0 = Sunday … bit 6 = Saturday.
pub mod week_day {
    /// Sunday.
    pub const SUN: u8 = 1 << 0;
    /// Monday.
    pub const MON: u8 = 1 << 1;
    /// Tuesday.
    pub const TUE: u8 = 1 << 2;
    /// Wednesday.
    pub const WED: u8 = 1 << 3;
    /// Thursday.
    pub const THU: u8 = 1 << 4;
    /// Friday.
    pub const FRI: u8 = 1 << 5;
    /// Saturday.
    pub const SAT: u8 = 1 << 6;
    /// Monday through Friday.
    pub const WEEKDAYS: u8 = MON | TUE | WED | THU | FRI;
    /// Saturday and Sunday.
    pub const WEEKEND: u8 = SUN | SAT;
    /// Every day of the week.
    pub const EVERY_DAY: u8 = SUN | MON | TUE | WED | THU | FRI | SAT;

    /// All bits the device accepts for `week_day` (bits 0..=6).
    pub(crate) const VALID_MASK: u8 = EVERY_DAY;
}

/// A plug schedule rule (the "Schedule" feature in the Tapo app).
///
/// Construct one with the builders ([`ScheduleRule::clock_weekly`],
/// [`ScheduleRule::sunrise_once`], …); each returns
/// `Result<Self, Error>` and reports an `Error::Validation` for
/// out-of-range inputs (e.g. `hour >= 24`).  The wire representation
/// is filled in on serialization.
///
/// The device evaluates HH:MM (or sunrise / sunset offset) against
/// its own configured timezone; you don't supply a calendar date.
/// The on-the-wire `year` / `month` / `day` fields the device
/// requires are filled with a constant placeholder
/// (`1970-01-01`) because the device ignores their values — this was
/// confirmed experimentally on a P110: a `clock_once` sent with
/// `year=1970, month=1, day=1` still fires at the requested HH:MM.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(from_py_object, frozen, get_all))]
#[serde(try_from = "WireRule", into = "WireRule")]
pub struct ScheduleRule {
    /// Device-assigned id.  `None` when the rule was constructed
    /// locally; `Some` when read back from the device.
    pub id: Option<String>,
    /// Whether the rule is currently active.  Disabled rules are kept
    /// on the device but do not fire.
    pub enable: bool,
    /// Time-of-day kind: clock, sunrise-relative, or sunset-relative.
    pub time_kind: ScheduleTimeKind,
    /// For [`ScheduleTimeKind::Clock`]: minutes after midnight (0..1440).
    /// For sunrise / sunset rules: ignored (the device computes the
    /// fire time on its own from `offset_minutes`).
    pub minute_of_day: u16,
    /// For sunrise / sunset rules: signed minutes from the astronomical
    /// event (negative = before, positive = after).  Ignored for
    /// clock rules.
    pub offset_minutes: i16,
    /// Once or weekly.
    pub frequency: ScheduleFrequency,
    /// Bitmask of days the rule fires on, when `frequency == Weekly`.
    /// Bit 0 = Sunday, bit 6 = Saturday.  See [`week_day`].
    pub week_day: u8,
    /// When the rule fires, turn the plug on (`true`) or off (`false`).
    pub turn_on: bool,
}

impl ScheduleRule {
    fn clock(
        hour: u8,
        minute: u8,
        frequency: ScheduleFrequency,
        week_day: u8,
        turn_on: bool,
    ) -> Result<Self, Error> {
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
        validate_week_day(week_day)?;
        Ok(Self {
            id: None,
            enable: true,
            time_kind: ScheduleTimeKind::Clock,
            minute_of_day: u16::from(hour) * 60 + u16::from(minute),
            offset_minutes: 0,
            frequency,
            week_day,
            turn_on,
        })
    }

    fn sun(
        kind: ScheduleTimeKind,
        offset_minutes: i16,
        frequency: ScheduleFrequency,
        week_day: u8,
        turn_on: bool,
    ) -> Result<Self, Error> {
        if offset_minutes.unsigned_abs() > 1440 {
            return Err(Error::Validation {
                field: "offset_minutes".to_string(),
                message: format!(
                    "Must be within -1440..=1440 minutes (±24h), got {offset_minutes}",
                ),
            });
        }
        validate_week_day(week_day)?;
        Ok(Self {
            id: None,
            enable: true,
            time_kind: kind,
            minute_of_day: 0,
            offset_minutes,
            frequency,
            week_day,
            turn_on,
        })
    }

    /// Fires every day matched by `week_day` at the given wall-clock time.
    ///
    /// Returns `Err(Error::Validation)` if `hour >= 24`, `minute >= 60`,
    /// or `week_day` has bits 7+ set.
    pub fn clock_weekly(hour: u8, minute: u8, week_day: u8, turn_on: bool) -> Result<Self, Error> {
        Self::clock(hour, minute, ScheduleFrequency::Weekly, week_day, turn_on)
    }

    /// Fires once, the next time the device's wall clock reaches `hour:minute`.
    ///
    /// Returns `Err(Error::Validation)` if `hour >= 24` or `minute >= 60`.
    pub fn clock_once(hour: u8, minute: u8, turn_on: bool) -> Result<Self, Error> {
        Self::clock(hour, minute, ScheduleFrequency::Once, 0, turn_on)
    }

    /// Fires every day matched by `week_day` at `offset_minutes` from sunrise.
    ///
    /// Returns `Err(Error::Validation)` if `offset_minutes` is outside
    /// ±1440 or `week_day` has bits 7+ set.
    pub fn sunrise_weekly(offset_minutes: i16, week_day: u8, turn_on: bool) -> Result<Self, Error> {
        Self::sun(
            ScheduleTimeKind::Sunrise,
            offset_minutes,
            ScheduleFrequency::Weekly,
            week_day,
            turn_on,
        )
    }

    /// Fires once at the next sunrise plus `offset_minutes`.
    ///
    /// Returns `Err(Error::Validation)` if `offset_minutes` is outside ±1440.
    pub fn sunrise_once(offset_minutes: i16, turn_on: bool) -> Result<Self, Error> {
        Self::sun(
            ScheduleTimeKind::Sunrise,
            offset_minutes,
            ScheduleFrequency::Once,
            0,
            turn_on,
        )
    }

    /// Fires every day matched by `week_day` at `offset_minutes` from sunset.
    ///
    /// Returns `Err(Error::Validation)` if `offset_minutes` is outside
    /// ±1440 or `week_day` has bits 7+ set.
    pub fn sunset_weekly(offset_minutes: i16, week_day: u8, turn_on: bool) -> Result<Self, Error> {
        Self::sun(
            ScheduleTimeKind::Sunset,
            offset_minutes,
            ScheduleFrequency::Weekly,
            week_day,
            turn_on,
        )
    }

    /// Fires once at the next sunset plus `offset_minutes`.
    ///
    /// Returns `Err(Error::Validation)` if `offset_minutes` is outside ±1440.
    pub fn sunset_once(offset_minutes: i16, turn_on: bool) -> Result<Self, Error> {
        Self::sun(
            ScheduleTimeKind::Sunset,
            offset_minutes,
            ScheduleFrequency::Once,
            0,
            turn_on,
        )
    }

    /// Returns a copy of this rule with `enable` set to the given value.
    pub fn with_enable(&self, enable: bool) -> Self {
        Self {
            enable,
            ..self.clone()
        }
    }

    /// Returns a copy of this rule with `id` set to the given value.
    /// Use before `edit_schedule_rule` (on
    /// [`PlugHandler`](crate::PlugHandler) or
    /// [`PlugEnergyMonitoringHandler`](crate::PlugEnergyMonitoringHandler))
    /// when reconstructing an edit from scratch.
    pub fn with_id(&self, id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..self.clone()
        }
    }
}

fn validate_week_day(week_day: u8) -> Result<(), Error> {
    if week_day & !self::week_day::VALID_MASK != 0 {
        return Err(Error::Validation {
            field: "week_day".to_string(),
            message: format!(
                "Must use bits 0..=6 only (bit 0 = Sun, bit 6 = Sat), got {week_day:#010b}",
            ),
        });
    }
    Ok(())
}

/// Wire shape of a schedule rule, used internally for (de)serialization.
/// Mirrors `ThingRuleSchedule` from the official Tapo Android app.
///
/// `year` / `month` / `day` are required by the API but their values
/// are ignored by the device (verified on a P110).  `s_type`
/// determines whether `s_min` is a clock minute-of-day or `time_offset`
/// is a sunrise / sunset offset.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireRule {
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
    desired_states: serde_json::Value,
    /// Deprecated mirror of the firing state, still emitted by some
    /// firmwares.  Used as a fallback when `desired_states.on` is
    /// absent.  See `ThingRuleSchedule.startAction` in the Tapo app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    s_action: Option<String>,
}

/// Constant placeholder for the wire `year` / `month` / `day` (device ignores these).
const PLACEHOLDER_DATE: (i32, u8, u8) = (1970, 1, 1);

impl From<ScheduleRule> for WireRule {
    fn from(r: ScheduleRule) -> Self {
        let (s_type, time_offset, s_min) = match r.time_kind {
            ScheduleTimeKind::Clock => ("normal", 0_i32, i32::from(r.minute_of_day)),
            ScheduleTimeKind::Sunrise => ("sunrise", i32::from(r.offset_minutes), 0),
            ScheduleTimeKind::Sunset => ("sunset", i32::from(r.offset_minutes), 0),
        };
        let (mode, week_day) = match r.frequency {
            ScheduleFrequency::Once => ("once", 0),
            ScheduleFrequency::Weekly => ("repeat", r.week_day),
        };
        let (year, month, day) = PLACEHOLDER_DATE;
        WireRule {
            id: r.id,
            enable: r.enable,
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
            desired_states: serde_json::json!({ "on": r.turn_on }),
            s_action: None,
        }
    }
}

impl TryFrom<WireRule> for ScheduleRule {
    type Error = String;

    fn try_from(w: WireRule) -> Result<Self, Self::Error> {
        let time_kind = match w.s_type.as_str() {
            "normal" => ScheduleTimeKind::Clock,
            "sunrise" => ScheduleTimeKind::Sunrise,
            "sunset" => ScheduleTimeKind::Sunset,
            other => return Err(format!("unknown schedule s_type {other:?}")),
        };
        let frequency = match w.mode.as_str() {
            "once" => ScheduleFrequency::Once,
            "repeat" => ScheduleFrequency::Weekly,
            other => return Err(format!("unknown schedule mode {other:?}")),
        };
        let turn_on = w
            .desired_states
            .get("on")
            .and_then(|v| v.as_bool())
            .or(match w.s_action.as_deref() {
                Some("on") => Some(true),
                Some("off") => Some(false),
                _ => None,
            })
            .ok_or_else(|| {
                "neither desired_states.on nor s_action contained a recognised firing state"
                    .to_string()
            })?;
        let minute_of_day: u16 = w
            .s_min
            .try_into()
            .map_err(|_| format!("s_min {} out of range for u16", w.s_min))?;
        let offset_minutes: i16 = w
            .time_offset
            .try_into()
            .map_err(|_| format!("time_offset {} out of range for i16", w.time_offset))?;
        Ok(ScheduleRule {
            id: w.id,
            enable: w.enable,
            time_kind,
            minute_of_day,
            offset_minutes,
            frequency,
            week_day: w.week_day,
            turn_on,
        })
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl ScheduleRule {
    #[staticmethod]
    #[pyo3(name = "clock_weekly")]
    fn py_clock_weekly(hour: u8, minute: u8, week_day: u8, turn_on: bool) -> PyResult<Self> {
        Ok(Self::clock_weekly(hour, minute, week_day, turn_on)?)
    }

    #[staticmethod]
    #[pyo3(name = "clock_once")]
    fn py_clock_once(hour: u8, minute: u8, turn_on: bool) -> PyResult<Self> {
        Ok(Self::clock_once(hour, minute, turn_on)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunrise_weekly")]
    fn py_sunrise_weekly(offset_minutes: i16, week_day: u8, turn_on: bool) -> PyResult<Self> {
        Ok(Self::sunrise_weekly(offset_minutes, week_day, turn_on)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunrise_once")]
    fn py_sunrise_once(offset_minutes: i16, turn_on: bool) -> PyResult<Self> {
        Ok(Self::sunrise_once(offset_minutes, turn_on)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunset_weekly")]
    fn py_sunset_weekly(offset_minutes: i16, week_day: u8, turn_on: bool) -> PyResult<Self> {
        Ok(Self::sunset_weekly(offset_minutes, week_day, turn_on)?)
    }

    #[staticmethod]
    #[pyo3(name = "sunset_once")]
    fn py_sunset_once(offset_minutes: i16, turn_on: bool) -> PyResult<Self> {
        Ok(Self::sunset_once(offset_minutes, turn_on)?)
    }

    #[pyo3(name = "with_enable")]
    fn py_with_enable(&self, enable: bool) -> Self {
        self.with_enable(enable)
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
    /// Mirrors the names exposed by attribute access (`time_kind`,
    /// `minute_of_day`, …) rather than the wire shape used for
    /// transport (`s_type`, `s_min`, …).
    fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(DictRule::from(self))
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
        crate::python::serde_object_to_py_dict(py, &value)
    }
}

/// User-facing dict shape for `ScheduleRule::to_dict`. Matches the
/// public fields of [`ScheduleRule`] (and the `get_all` attributes
/// exposed to Python) instead of the on-the-wire [`WireRule`].
#[cfg(feature = "python")]
#[derive(Serialize)]
struct DictRule<'a> {
    id: &'a Option<String>,
    enable: bool,
    time_kind: ScheduleTimeKind,
    minute_of_day: u16,
    offset_minutes: i16,
    frequency: ScheduleFrequency,
    week_day: u8,
    turn_on: bool,
}

#[cfg(feature = "python")]
impl<'a> From<&'a ScheduleRule> for DictRule<'a> {
    fn from(r: &'a ScheduleRule) -> Self {
        DictRule {
            id: &r.id,
            enable: r.enable,
            time_kind: r.time_kind,
            minute_of_day: r.minute_of_day,
            offset_minutes: r.offset_minutes,
            frequency: r.frequency,
            week_day: r.week_day,
            turn_on: r.turn_on,
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

    fn clock_once(hour: u8, minute: u8, turn_on: bool) -> ScheduleRule {
        ScheduleRule::clock_once(hour, minute, turn_on).expect("valid clock_once")
    }

    fn clock_weekly(hour: u8, minute: u8, week_day: u8, turn_on: bool) -> ScheduleRule {
        ScheduleRule::clock_weekly(hour, minute, week_day, turn_on).expect("valid clock_weekly")
    }

    #[test]
    fn clock_once_wire_shape() {
        let r = clock_once(6, 30, true);
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "normal");
        assert_eq!(j["mode"], "once");
        assert_eq!(j["s_min"], 6 * 60 + 30);
        assert_eq!(j["time_offset"], 0);
        assert_eq!(j["week_day"], 0);
        assert_eq!(j["enable"], true);
        assert_eq!(j["desired_states"], serde_json::json!({ "on": true }));
        // Date placeholder is constant per the WireRule contract.
        assert_eq!(j["year"], 1970);
        assert_eq!(j["month"], 1);
        assert_eq!(j["day"], 1);
        // No id field when constructed locally.
        assert!(j.get("id").is_none());
    }

    #[test]
    fn clock_weekly_wire_shape() {
        let r = clock_weekly(23, 30, week_day::WEEKDAYS, false);
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "normal");
        assert_eq!(j["mode"], "repeat");
        assert_eq!(j["s_min"], 23 * 60 + 30);
        assert_eq!(j["week_day"], week_day::WEEKDAYS);
        assert_eq!(j["desired_states"], serde_json::json!({ "on": false }));
    }

    #[test]
    fn sunrise_weekly_wire_shape() {
        let r = ScheduleRule::sunrise_weekly(-63, week_day::MON | week_day::WED, true)
            .expect("valid sunrise_weekly");
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "sunrise");
        assert_eq!(j["mode"], "repeat");
        assert_eq!(j["time_offset"], -63);
        assert_eq!(j["s_min"], 0);
        assert_eq!(j["week_day"], week_day::MON | week_day::WED);
    }

    #[test]
    fn sunset_once_wire_shape() {
        let r = ScheduleRule::sunset_once(0, false).expect("valid sunset_once");
        let j = wire_json(&r);
        assert_eq!(j["s_type"], "sunset");
        assert_eq!(j["mode"], "once");
        assert_eq!(j["time_offset"], 0);
        assert_eq!(j["week_day"], 0);
    }

    #[test]
    fn round_trip_via_wire() {
        for original in [
            clock_once(6, 30, true),
            clock_weekly(23, 30, week_day::WEEKDAYS, false),
            clock_weekly(0, 0, week_day::EVERY_DAY, true),
            ScheduleRule::sunrise_weekly(-63, week_day::MON | week_day::WED, true).unwrap(),
            ScheduleRule::sunrise_once(15, true).unwrap(),
            ScheduleRule::sunset_weekly(7, week_day::EVERY_DAY, false).unwrap(),
            ScheduleRule::sunset_once(0, false).unwrap(),
        ] {
            let wire = serde_json::to_value(&original).expect("serialize");
            let back: ScheduleRule = serde_json::from_value(wire).expect("deserialize");
            assert_eq!(back, original);
        }
    }

    #[test]
    fn round_trip_preserves_device_id() {
        let original = clock_weekly(8, 0, week_day::MON, true).with_id("S42");
        let wire = serde_json::to_value(&original).expect("serialize");
        assert_eq!(wire["id"], "S42");
        let back: ScheduleRule = serde_json::from_value(wire).expect("deserialize");
        assert_eq!(back, original);
        assert_eq!(back.id.as_deref(), Some("S42"));
    }

    #[test]
    fn with_enable_clones_and_overrides() {
        let r = clock_weekly(8, 0, week_day::MON, true).with_id("S42");
        let disabled = r.with_enable(false);
        assert!(!disabled.enable);
        assert!(r.enable); // original unchanged
        assert_eq!(disabled.id, r.id);
        assert_eq!(disabled.minute_of_day, r.minute_of_day);
    }

    #[test]
    fn deserialize_rejects_bad_s_min() {
        let mut wire = serde_json::to_value(clock_once(6, 30, true)).unwrap();
        wire["s_min"] = serde_json::json!(-1);
        let err: Result<ScheduleRule, _> = serde_json::from_value(wire);
        assert!(err.is_err());
    }

    #[test]
    fn deserialize_rejects_unknown_s_type() {
        let mut wire = serde_json::to_value(clock_once(6, 30, true)).unwrap();
        wire["s_type"] = serde_json::json!("eclipse");
        let err: Result<ScheduleRule, _> = serde_json::from_value(wire);
        assert!(err.is_err());
    }

    #[test]
    fn clock_weekly_rejects_bad_hour() {
        let err = ScheduleRule::clock_weekly(25, 0, week_day::MON, true).unwrap_err();
        assert!(
            matches!(&err, Error::Validation { field, message }
                if field == "hour" && message.contains("0..=23")),
            "unexpected error: {err:?}",
        );
    }

    #[test]
    fn clock_weekly_rejects_bad_minute() {
        let err = ScheduleRule::clock_weekly(8, 99, week_day::MON, true).unwrap_err();
        assert!(
            matches!(&err, Error::Validation { field, message }
                if field == "minute" && message.contains("0..=59")),
            "unexpected error: {err:?}",
        );
    }

    #[test]
    fn clock_weekly_rejects_high_week_day_bits() {
        let err = ScheduleRule::clock_weekly(8, 0, 0b1000_0000, true).unwrap_err();
        assert!(
            matches!(&err, Error::Validation { field, message }
                if field == "week_day" && message.contains("bits 0..=6")),
            "unexpected error: {err:?}",
        );
    }

    #[test]
    fn sunrise_rejects_huge_offset() {
        let err = ScheduleRule::sunrise_once(1500, true).unwrap_err();
        assert!(
            matches!(&err, Error::Validation { field, message }
                if field == "offset_minutes" && message.contains("-1440..=1440")),
            "unexpected error: {err:?}",
        );
    }

    #[test]
    fn deserialize_falls_back_to_s_action_when_desired_states_absent() {
        // Simulate a legacy / minimal firmware that only emits `s_action`
        // and an empty `desired_states`.
        let mut wire = serde_json::to_value(clock_once(6, 30, true)).unwrap();
        wire["desired_states"] = serde_json::json!({});
        wire["s_action"] = serde_json::json!("off");
        let parsed: ScheduleRule = serde_json::from_value(wire).expect("fallback to s_action");
        assert!(!parsed.turn_on);
    }

    #[test]
    fn deserialize_prefers_desired_states_over_s_action() {
        // Both fields present and disagree — desired_states wins.
        let mut wire = serde_json::to_value(clock_once(6, 30, true)).unwrap();
        wire["s_action"] = serde_json::json!("off");
        let parsed: ScheduleRule = serde_json::from_value(wire).expect("desired_states wins");
        assert!(parsed.turn_on);
    }

    #[cfg(feature = "python")]
    #[test]
    fn to_dict_shape_matches_user_facing_fields() {
        // The dict surface must mirror the public/get_all fields,
        // not the on-the-wire WireRule. See module-level docs.
        let rule = ScheduleRule::sunset_weekly(-15, week_day::WEEKDAYS, true)
            .unwrap()
            .with_id("S42");
        let dict = serde_json::to_value(DictRule::from(&rule)).unwrap();
        let obj = dict.as_object().expect("object");
        let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "enable",
                "frequency",
                "id",
                "minute_of_day",
                "offset_minutes",
                "time_kind",
                "turn_on",
                "week_day",
            ],
        );
        assert_eq!(dict["id"], "S42");
        assert_eq!(dict["enable"], true);
        assert_eq!(dict["time_kind"], "sunset");
        assert_eq!(dict["frequency"], "weekly");
        assert_eq!(dict["offset_minutes"], -15);
        assert_eq!(dict["week_day"], week_day::WEEKDAYS);
        assert_eq!(dict["turn_on"], true);
        // No wire-only fields leak through.
        assert!(!obj.contains_key("s_type"));
        assert!(!obj.contains_key("s_min"));
        assert!(!obj.contains_key("year"));
    }

    #[test]
    fn deserialize_rejects_missing_firing_state() {
        let mut wire = serde_json::to_value(clock_once(6, 30, true)).unwrap();
        wire["desired_states"] = serde_json::json!({});
        // No s_action either.
        let err: Result<ScheduleRule, _> = serde_json::from_value(wire);
        assert!(err.is_err());
    }
}
