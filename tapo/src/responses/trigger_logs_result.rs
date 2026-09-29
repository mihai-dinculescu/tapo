use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::TapoResponseExt;

/// Trigger logs result.
#[derive(Debug, Deserialize, Serialize)]
pub struct TriggerLogsResult<E> {
    /// The `id` of the most recent log item that is returned.
    pub start_id: u64,
    /// The total number of log items that the hub holds for this device.
    pub sum: u64,
    /// Log items in reverse chronological order (newest first).
    pub logs: Vec<TriggerLog<E>>,
}

impl<E> TapoResponseExt for TriggerLogsResult<E> {}

/// One entry in a sensor's trigger log.
///
/// `E` is the sensor's event enum: [`S200Event`](crate::responses::S200Event),
/// [`T100Event`](crate::responses::T100Event),
/// [`T110Event`](crate::responses::T110Event) or
/// [`T300Event`](crate::responses::T300Event).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TriggerLog<E> {
    /// Hub-assigned id of the log item. The hub numbers the logs of all its sensors from one counter, so newer items have larger ids.
    pub id: u64,
    /// When the event happened, in UTC.
    #[serde(
        rename(deserialize = "timestamp"),
        deserialize_with = "chrono::serde::ts_seconds::deserialize"
    )]
    pub triggered_at: DateTime<Utc>,
    /// What happened.
    #[serde(flatten)]
    pub event: E,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::{S200Event, S200RotationParams, T100Event, T110Event, T300Event};

    fn utc(seconds: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(seconds, 0).unwrap()
    }

    #[test]
    fn test_s200_page_parses_rotation_params_and_unit_events() {
        let json = r#"{
            "start_id": 12,
            "sum": 40,
            "logs": [
                {"id": 12, "timestamp": 1758400000, "event": "rotation", "params": {"rotate_deg": -90}},
                {"id": 11, "timestamp": 1758399990, "event": "singleClick"},
                {"id": 10, "timestamp": 1758399980, "event": "doubleClick"},
                {"id": 9, "timestamp": 1758399970, "event": "lowBattery"}
            ]
        }"#;

        let parsed: TriggerLogsResult<S200Event> = serde_json::from_str(json).unwrap();

        assert_eq!(parsed.start_id, 12);
        assert_eq!(parsed.sum, 40);
        assert_eq!(
            parsed.logs,
            vec![
                TriggerLog {
                    id: 12,
                    triggered_at: utc(1758400000),
                    event: S200Event::Rotation {
                        params: S200RotationParams {
                            rotation_degrees: -90
                        }
                    },
                },
                TriggerLog {
                    id: 11,
                    triggered_at: utc(1758399990),
                    event: S200Event::SingleClick,
                },
                TriggerLog {
                    id: 10,
                    triggered_at: utc(1758399980),
                    event: S200Event::DoubleClick,
                },
                TriggerLog {
                    id: 9,
                    triggered_at: utc(1758399970),
                    event: S200Event::LowBattery,
                },
            ]
        );
    }

    #[test]
    fn test_t100_t110_t300_events_parse() {
        let t100: TriggerLog<T100Event> =
            serde_json::from_str(r#"{"id": 1, "timestamp": 1758400000, "event": "motion"}"#)
                .unwrap();
        assert_eq!(t100.event, T100Event::Motion);

        let t110: TriggerLogsResult<T110Event> = serde_json::from_str(
            r#"{"start_id": 3, "sum": 3, "logs": [
                {"id": 3, "timestamp": 1758400002, "event": "keepOpen"},
                {"id": 2, "timestamp": 1758400001, "event": "open"},
                {"id": 1, "timestamp": 1758400000, "event": "close"}
            ]}"#,
        )
        .unwrap();
        assert_eq!(
            t110.logs.iter().map(|log| &log.event).collect::<Vec<_>>(),
            vec![&T110Event::KeepOpen, &T110Event::Open, &T110Event::Close]
        );

        let t300: TriggerLogsResult<T300Event> = serde_json::from_str(
            r#"{"start_id": 2, "sum": 2, "logs": [
                {"id": 2, "timestamp": 1758400001, "event": "waterDry"},
                {"id": 1, "timestamp": 1758400000, "event": "waterLeak"}
            ]}"#,
        )
        .unwrap();
        assert_eq!(
            t300.logs.iter().map(|log| &log.event).collect::<Vec<_>>(),
            vec![&T300Event::WaterDry, &T300Event::WaterLeak]
        );
    }

    #[test]
    fn test_serializes_triggered_at_as_rfc3339_with_flat_event() {
        let log = TriggerLog {
            id: 12,
            triggered_at: utc(1758400000),
            event: S200Event::Rotation {
                params: S200RotationParams {
                    rotation_degrees: 90,
                },
            },
        };

        let json = serde_json::to_value(&log).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "id": 12,
                "triggered_at": "2025-09-20T20:26:40Z",
                "event": "rotation",
                "params": {"rotate_deg": 90}
            })
        );
    }

    #[test]
    fn test_empty_page_parses() {
        let parsed: TriggerLogsResult<T100Event> =
            serde_json::from_str(r#"{"start_id": 0, "sum": 0, "logs": []}"#).unwrap();

        assert!(parsed.logs.is_empty());
    }
}
