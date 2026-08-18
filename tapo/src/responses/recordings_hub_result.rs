use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Recording date list result (`searchDateWithVideo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordingDateListHubResultRaw {
    #[serde(default)]
    playback: RecordingDateListRaw,
}

impl RecordingDateListHubResultRaw {
    pub fn dates(self) -> Result<Vec<NaiveDate>, chrono::ParseError> {
        self.playback
            .search_results
            .into_iter()
            .flat_map(RecordingDateEntryRaw::into_dates)
            .map(|date| NaiveDate::parse_from_str(&date, "%Y%m%d"))
            .collect()
    }
}

impl TapoResponseExt for RecordingDateListHubResultRaw {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct RecordingDateListRaw {
    #[serde(default)]
    search_results: Vec<RecordingDateEntryRaw>,
}

/// The Tapo app parses each entry as a single-key section object wrapping the
/// date (e.g. `{"search_video_date": {"date": "20250101"}}`); accept both that
/// and the flat `{"date": "20250101"}` shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum RecordingDateEntryRaw {
    Date(RecordingDateRaw),
    Section(HashMap<String, RecordingDateRaw>),
}

impl RecordingDateEntryRaw {
    fn into_dates(self) -> Vec<String> {
        match self {
            RecordingDateEntryRaw::Date(entry) => vec![entry.date],
            RecordingDateEntryRaw::Section(entries) => {
                entries.into_values().map(|entry| entry.date).collect()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordingDateRaw {
    date: String,
}

/// Recording list result (`searchVideoWithUTC`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordingListHubResultRaw {
    #[serde(default)]
    playback: RecordingListRaw,
}

impl RecordingListHubResultRaw {
    /// Returns the recordings of this page and whether another page follows.
    pub fn into_parts(self) -> (Vec<RecordingHubResult>, bool) {
        let to_be_continued = self.playback.to_be_continued == Some(1);
        let recordings = self
            .playback
            .search_video_results
            .into_iter()
            .flat_map(RecordingEntryRaw::into_recordings)
            .collect();

        (recordings, to_be_continued)
    }
}

impl TapoResponseExt for RecordingListHubResultRaw {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct RecordingListRaw {
    #[serde(default)]
    search_video_results: Vec<RecordingEntryRaw>,
    #[serde(default)]
    to_be_continued: Option<i64>,
}

/// The Tapo app parses each entry either flat or as a single-key section
/// object wrapping the recording; accept both shapes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum RecordingEntryRaw {
    Recording(RecordingHubResult),
    Section(HashMap<String, RecordingHubResult>),
}

impl RecordingEntryRaw {
    fn into_recordings(self) -> Vec<RecordingHubResult> {
        match self {
            RecordingEntryRaw::Recording(recording) => vec![recording],
            RecordingEntryRaw::Section(entries) => entries.into_values().collect(),
        }
    }
}

/// Recording stored on a camera hub for a camera paired to it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RecordingHubResultRaw")]
pub struct RecordingHubResult {
    /// Start of the recording as a Unix timestamp (seconds).
    #[serde(rename = "startTime")]
    pub start_time: u64,
    /// End of the recording as a Unix timestamp (seconds).
    #[serde(rename = "endTime")]
    pub end_time: u64,
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

/// Devices send the recording type under a misspelled `vedio_type` key; some
/// firmware uses `video_type` instead. Accept either (or both) and fold them
/// into [`RecordingHubResult::video_type`].
#[derive(Debug, Clone, Deserialize)]
struct RecordingHubResultRaw {
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: u64,
    #[serde(default, rename = "vedio_type")]
    misspelled_video_type: Option<RecordingType>,
    #[serde(default)]
    video_type: Option<RecordingType>,
}

impl TryFrom<RecordingHubResultRaw> for RecordingHubResult {
    type Error = String;

    fn try_from(raw: RecordingHubResultRaw) -> Result<Self, Self::Error> {
        // The Tapo app reads `vedio_type` first and falls back to `video_type`.
        let video_type = raw
            .misspelled_video_type
            .or(raw.video_type)
            .ok_or("missing field `video_type`")?;

        Ok(Self {
            start_time: raw.start_time,
            end_time: raw.end_time,
            video_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_dates_parse_from_section_entries() {
        let json = r#"{
            "playback": {
                "search_results": [
                    {"search_video_date": {"date": "20260801"}},
                    {"search_video_date": {"date": "20260817"}}
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
    fn test_recording_dates_parse_from_flat_entries() {
        let json = r#"{"playback": {"search_results": [{"date": "20260815"}]}}"#;

        let parsed: RecordingDateListHubResultRaw = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed.dates().unwrap(),
            vec![NaiveDate::from_ymd_opt(2026, 8, 15).unwrap()]
        );
    }

    #[test]
    fn test_missing_search_results_parses_as_empty() {
        let json = r#"{"playback": {}}"#;

        let parsed: RecordingDateListHubResultRaw = serde_json::from_str(json).unwrap();

        assert!(parsed.dates().unwrap().is_empty());
    }

    #[test]
    fn test_invalid_recording_date_is_an_error() {
        let json = r#"{"playback": {"search_results": [{"date": "not-a-date"}]}}"#;

        let parsed: RecordingDateListHubResultRaw = serde_json::from_str(json).unwrap();

        assert!(parsed.dates().is_err());
    }

    #[test]
    fn test_recordings_parse_from_flat_entries() {
        let json = r#"{
            "playback": {
                "search_video_results": [
                    {"startTime": 1786694400, "endTime": 1786694460, "vedio_type": "2"}
                ],
                "to_be_continued": 1
            }
        }"#;

        let parsed: RecordingListHubResultRaw = serde_json::from_str(json).unwrap();
        let (recordings, to_be_continued) = parsed.into_parts();

        assert_eq!(recordings.len(), 1);
        assert_eq!(recordings[0].start_time, 1786694400);
        assert_eq!(recordings[0].end_time, 1786694460);
        assert_eq!(recordings[0].video_type, RecordingType::Motion);
        assert!(to_be_continued);
    }

    #[test]
    fn test_recording_type_parses_from_either_key() {
        let json = r#"{
            "playback": {
                "search_video_results": [
                    {"startTime": 0, "endTime": 1, "video_type": "1"},
                    {"startTime": 2, "endTime": 3, "vedio_type": "2", "video_type": "1"},
                    {"startTime": 4, "endTime": 5, "vedio_type": "99"}
                ]
            }
        }"#;

        let parsed: RecordingListHubResultRaw = serde_json::from_str(json).unwrap();
        let (recordings, _) = parsed.into_parts();

        assert_eq!(recordings[0].video_type, RecordingType::Timing);
        assert_eq!(recordings[1].video_type, RecordingType::Motion);
        assert_eq!(
            recordings[2].video_type,
            RecordingType::Other("99".to_string())
        );
    }

    #[test]
    fn test_missing_recording_type_is_an_error() {
        let json = r#"{"playback": {"search_video_results": [{"startTime": 0, "endTime": 1}]}}"#;

        assert!(serde_json::from_str::<RecordingListHubResultRaw>(json).is_err());
    }

    #[test]
    fn test_recordings_parse_from_section_entries() {
        let json = r#"{
            "playback": {
                "search_video_results": [
                    {"search_video_result": {"startTime": 1786694400, "endTime": 1786694460, "vedio_type": "1"}}
                ]
            }
        }"#;

        let parsed: RecordingListHubResultRaw = serde_json::from_str(json).unwrap();
        let (recordings, to_be_continued) = parsed.into_parts();

        assert_eq!(recordings.len(), 1);
        assert_eq!(recordings[0].start_time, 1786694400);
        assert!(!to_be_continued);
    }

    #[test]
    fn test_missing_search_video_results_parses_as_empty() {
        let json = r#"{"playback": {}}"#;

        let parsed: RecordingListHubResultRaw = serde_json::from_str(json).unwrap();
        let (recordings, to_be_continued) = parsed.into_parts();

        assert!(recordings.is_empty());
        assert!(!to_be_continued);
    }
}
