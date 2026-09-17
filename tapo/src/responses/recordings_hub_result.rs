use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Recording date list result (`searchDateWithVideo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordingDateListHubResultRaw {
    playback: RecordingDateListRaw,
}

impl RecordingDateListHubResultRaw {
    pub fn dates(self) -> Result<Vec<NaiveDate>, chrono::ParseError> {
        self.playback
            .search_results
            .into_iter()
            .flat_map(HashMap::into_values)
            .map(|entry| NaiveDate::parse_from_str(&entry.date, "%Y%m%d"))
            .collect()
    }
}

impl TapoResponseExt for RecordingDateListHubResultRaw {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordingDateListRaw {
    /// Each date is wrapped in a single-key section object, e.g.
    /// `{"search_results_1": {"date": "20260906"}}`.
    #[serde(default)]
    search_results: Vec<HashMap<String, RecordingDateRaw>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordingDateRaw {
    date: String,
}

/// Recording list result (`searchVideoWithUTC`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordingListHubResultRaw {
    playback: RecordingListRaw,
}

impl RecordingListHubResultRaw {
    pub fn recordings(self) -> Vec<RecordingHubResult> {
        self.playback
            .search_video_results
            .into_iter()
            .flat_map(HashMap::into_values)
            .collect()
    }
}

impl TapoResponseExt for RecordingListHubResultRaw {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordingListRaw {
    /// Each recording is wrapped in a single-key section object, e.g.
    /// `{"search_video_results_1": {"startTime": ..., "endTime": ..., "video_type": "2"}}`.
    #[serde(default)]
    search_video_results: Vec<HashMap<String, RecordingHubResult>>,
}

/// Recording stored on a camera hub for a camera paired to it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingHubResult {
    /// Start of the recording.
    #[serde(
        rename = "startTime",
        deserialize_with = "chrono::serde::ts_seconds::deserialize"
    )]
    pub start_time: DateTime<Utc>,
    /// End of the recording.
    #[serde(
        rename = "endTime",
        deserialize_with = "chrono::serde::ts_seconds::deserialize"
    )]
    pub end_time: DateTime<Utc>,
    /// The type of event that produced the recording.
    pub video_type: RecordingType,
}

/// The type of event that produced a recording. The wire values are numeric
/// strings; the names follow the Tapo app's playback event table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingType {
    /// `1`: continuous (timing) recording.
    #[serde(rename = "1")]
    Timing,
    /// `2`: motion detection.
    #[serde(rename = "2")]
    Motion,
    /// `3`: camera tampering.
    #[serde(rename = "3")]
    Tamper,
    /// `4`: line crossing detection.
    #[serde(rename = "4")]
    LineCrossing,
    /// `5`: area intrusion detection.
    #[serde(rename = "5")]
    AreaIntrusion,
    /// `6`: person detection.
    #[serde(rename = "6")]
    Person,
    /// `7`: baby cry detection.
    #[serde(rename = "7")]
    BabyCry,
    /// `8`: vehicle detection.
    #[serde(rename = "8")]
    Vehicle,
    /// `9`: pet detection.
    #[serde(rename = "9")]
    Pet,
    /// `10`: ring alarm.
    #[serde(rename = "10")]
    RingAlarm,
    /// `11`: bark detection.
    #[serde(rename = "11")]
    Bark,
    /// `12`: meow detection.
    #[serde(rename = "12")]
    Meow,
    /// `13`: glass breaking detection.
    #[serde(rename = "13")]
    GlassBreaking,
    /// `14`: smoke alarm detection.
    #[serde(rename = "14")]
    Smoke,
    /// `15`: package delivered.
    #[serde(rename = "15")]
    PackageDelivered,
    /// `16`: package picked up.
    #[serde(rename = "16")]
    PackagePickedUp,
    /// `17`: missed doorbell ring.
    #[serde(rename = "17")]
    MissedDoorbellRing,
    /// `18`: answered doorbell ring.
    #[serde(rename = "18")]
    AnsweredDoorbellRing,
    /// `19`: anti-theft alarm.
    #[serde(rename = "19")]
    AntiTheft,
    /// `20`: face detection.
    #[serde(rename = "20")]
    Face,
    /// `21`: unfamiliar face detection.
    #[serde(rename = "21")]
    UnfamiliarFace,
    /// `22`: unfamiliar person detection.
    #[serde(rename = "22")]
    UnfamiliarPerson,
    /// `23`: baby leaving detection.
    #[serde(rename = "23")]
    BabyLeave,
    /// `24`: baby caregiver detection.
    #[serde(rename = "24")]
    BabyCaregiver,
    /// `25`: baby asleep detection.
    #[serde(rename = "25")]
    BabyAsleep,
    /// `26`: baby waking up detection.
    #[serde(rename = "26")]
    BabyAwake,
    /// `27`: covered face detection.
    #[serde(rename = "27")]
    FaceCover,
    /// `28`: leaving the safety fence detection.
    #[serde(rename = "28")]
    SafeFenceOut,
    /// `30`: baby motion detection.
    #[serde(rename = "30")]
    BabyMotion,
    /// `31`: panoramic video.
    #[serde(rename = "31")]
    PanoramicVideo,
    /// `33`: animal detection.
    #[serde(rename = "33")]
    Animal,
    /// A recording type this library does not know yet, as its raw wire value.
    #[serde(untagged)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_dates_parse_from_section_entries() {
        let json = r#"{
            "playback": {
                "search_results": [
                    {"search_results_1": {"date": "20260801"}},
                    {"search_results_2": {"date": "20260817"}}
                ]
            }
        }"#;

        let parsed: RecordingDateListHubResultRaw = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed.dates().unwrap(),
            vec![
                NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 17).unwrap()
            ]
        );
    }

    #[test]
    fn test_invalid_recording_date_is_an_error() {
        let json =
            r#"{"playback": {"search_results": [{"search_results_1": {"date": "not-a-date"}}]}}"#;

        let parsed: RecordingDateListHubResultRaw = serde_json::from_str(json).unwrap();

        assert!(parsed.dates().is_err());
    }

    #[test]
    fn test_recordings_parse_from_section_entries() {
        let json = r#"{
            "playback": {
                "search_video_results": [
                    {"search_video_results_1": {"startTime": 1786694400, "endTime": 1786694460, "video_type": "2"}}
                ]
            }
        }"#;

        let parsed: RecordingListHubResultRaw = serde_json::from_str(json).unwrap();
        let recordings = parsed.recordings();

        assert_eq!(recordings.len(), 1);
        assert_eq!(
            recordings[0].start_time,
            "2026-08-14T08:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            recordings[0].end_time,
            "2026-08-14T08:01:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(recordings[0].video_type, RecordingType::Motion);
    }

    #[test]
    fn test_unknown_recording_type_parses_as_other() {
        let json = r#"{
            "playback": {
                "search_video_results": [
                    {"search_video_results_1": {"startTime": 0, "endTime": 1, "video_type": "99"}}
                ]
            }
        }"#;

        let parsed: RecordingListHubResultRaw = serde_json::from_str(json).unwrap();
        let recordings = parsed.recordings();

        assert_eq!(
            recordings[0].video_type,
            RecordingType::Other("99".to_string())
        );
    }
}
