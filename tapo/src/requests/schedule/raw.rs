use serde::{Deserialize, Serialize};

use crate::responses::{DesiredStateRaw, ScheduleRuleResult};

use super::{DaysOfWeek, ScheduleRule, ScheduleTime};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::PowerState;

    fn wire_json(rule: &ScheduleRule) -> serde_json::Value {
        serde_json::to_value(ScheduleRuleRaw::from(rule)).expect("serialize")
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
            map.insert(key.clone(), value.clone());
        }
        base
    }

    #[test]
    fn clock_once_wire_shape() {
        let rule = ScheduleRule::clock_once(6, 30, PowerState::On).expect("valid");
        let j = wire_json(&rule);
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
        let rule =
            ScheduleRule::clock_weekly(23, 30, DaysOfWeek::MON | DaysOfWeek::WED, PowerState::Off)
                .expect("valid");
        let j = wire_json(&rule);
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
        let original = ScheduleRule::clock_weekly(8, 5, DaysOfWeek::EVERY_DAY, PowerState::On)
            .expect("valid")
            .with_id("S7");
        let result = ScheduleRuleResult::try_from(ScheduleRuleRaw::from(&original)).expect("parse");

        assert_eq!(result.id, "S7");
        assert_eq!(result.time, ScheduleTime::Clock { hour: 8, minute: 5 });
        assert_eq!(result.days, Some(DaysOfWeek::EVERY_DAY));

        assert_eq!(result.to_editable().expect("valid"), original);
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
        assert!(
            matches!(err, crate::error::Error::Validation { ref field, .. } if field == "days")
        );
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
}
