use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::responses::{RecordingType, TapoResponseExt};
use crate::utils::datetime_from_unix_seconds_or_rfc3339;

/// Detection list result (`searchDetectionList`).
#[derive(Debug, Deserialize)]
pub(crate) struct DetectionListResultRaw {
    playback: DetectionListRaw,
}

impl DetectionListResultRaw {
    pub fn detections(self) -> Vec<DetectionResult> {
        self.playback
            .search_detection_list
            .into_iter()
            .map(Into::into)
            .collect()
    }

    /// Whether the camera has more pages after this one.
    pub fn to_be_continued(&self) -> bool {
        self.playback.to_be_continued == 1
    }
}

impl TapoResponseExt for DetectionListResultRaw {}

#[derive(Debug, Deserialize)]
struct DetectionListRaw {
    /// A plain array, unlike the clip search, which wraps each item in a
    /// single-key section object.
    #[serde(default)]
    search_detection_list: Vec<DetectionRaw>,
    /// `1` while more pages follow, `0` on the last one.
    to_be_continued: u8,
}

#[derive(Debug, Deserialize)]
struct DetectionRaw {
    #[serde(deserialize_with = "datetime_from_unix_seconds_or_rfc3339")]
    start_time: DateTime<Utc>,
    #[serde(deserialize_with = "datetime_from_unix_seconds_or_rfc3339")]
    end_time: DateTime<Utc>,
    /// A code from the Tapo app's playback event table, as a number.
    alarm_type: u64,
    /// A bit mask of every event type in the span: bit *n* stands for the
    /// code *n + 1*.
    events_1: u64,
}

impl From<DetectionRaw> for DetectionResult {
    fn from(raw: DetectionRaw) -> Self {
        Self {
            start_time: raw.start_time,
            end_time: raw.end_time,
            trigger: RecordingType::from_code(raw.alarm_type),
            events: RecordingType::from_mask(raw.events_1),
        }
    }
}

/// A detection stored on a camera's SD card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    /// Start of the detection.
    pub start_time: DateTime<Utc>,
    /// End of the detection.
    pub end_time: DateTime<Utc>,
    /// The event the camera files the detection under. When a person and
    /// motion are both detected, this is [`RecordingType::Person`].
    pub trigger: RecordingType,
    /// Every event detected in the span, e.g. `[Motion, Person]`.
    pub events: Vec<RecordingType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detections_parse_from_the_plain_array() {
        let json = r#"{
            "playback": {
                "search_detection_list": [
                    {"alarm_type": 6, "start_time": 1792786149, "end_time": 1792786217, "events_1": 34}
                ],
                "snapshot_enable": true,
                "to_be_continued": 1,
                "total_num": 100
            }
        }"#;

        let parsed: DetectionListResultRaw = serde_json::from_str(json).unwrap();
        assert!(parsed.to_be_continued());

        let detections = parsed.detections();

        assert_eq!(detections.len(), 1);
        assert_eq!(
            detections[0].start_time,
            "2026-10-23T20:09:09Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            detections[0].end_time,
            "2026-10-23T20:10:17Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(detections[0].trigger, RecordingType::Person);
        assert_eq!(
            detections[0].events,
            vec![RecordingType::Motion, RecordingType::Person]
        );
    }

    #[test]
    fn test_unknown_alarm_type_parses_as_other() {
        let json = r#"{
            "playback": {
                "search_detection_list": [
                    {"alarm_type": 99, "start_time": 0, "end_time": 1, "events_1": 0}
                ],
                "to_be_continued": 0
            }
        }"#;

        let parsed: DetectionListResultRaw = serde_json::from_str(json).unwrap();
        assert!(!parsed.to_be_continued());

        let detections = parsed.detections();

        assert_eq!(
            detections[0].trigger,
            RecordingType::Other("99".to_string())
        );
        assert!(detections[0].events.is_empty());
    }

    #[test]
    fn test_empty_list_parses_to_no_detections() {
        let json = r#"{
            "playback": {
                "search_detection_list": [],
                "snapshot_enable": true,
                "to_be_continued": 0,
                "total_num": 0
            }
        }"#;

        let parsed: DetectionListResultRaw = serde_json::from_str(json).unwrap();

        assert!(!parsed.to_be_continued());
        assert!(parsed.detections().is_empty());
    }

    #[test]
    fn test_detection_round_trips_through_its_own_json() {
        let detection = DetectionResult {
            start_time: "2026-10-23T19:29:09Z".parse::<DateTime<Utc>>().unwrap(),
            end_time: "2026-10-23T19:30:17Z".parse::<DateTime<Utc>>().unwrap(),
            trigger: RecordingType::Person,
            events: vec![RecordingType::Motion, RecordingType::Person],
        };

        let json = serde_json::to_string(&detection).unwrap();

        assert_eq!(
            json,
            r#"{"start_time":"2026-10-23T19:29:09Z","end_time":"2026-10-23T19:30:17Z","trigger":"Person","events":["Motion","Person"]}"#
        );

        let parsed: DetectionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.start_time, detection.start_time);
        assert_eq!(parsed.end_time, detection.end_time);
        assert_eq!(parsed.trigger, detection.trigger);
        assert_eq!(parsed.events, detection.events);
    }
}
