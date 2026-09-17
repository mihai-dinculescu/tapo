use std::collections::HashMap;

use chrono::{DateTime, Duration, MappedLocalTime, NaiveDate, NaiveTime, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Recording date list result (`searchDateWithVideo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordingDateListHubResultRaw {
    playback: RecordingDateListRaw,
}

impl RecordingDateListHubResultRaw {
    /// The hub reports days on its own calendar, so `timezone` is the hub's,
    /// as reported by `getTimezone`.
    pub fn dates(self, timezone: Tz) -> Result<Vec<RecordingDateHubResult>, anyhow::Error> {
        self.playback
            .search_results
            .into_iter()
            .flat_map(HashMap::into_values)
            .map(|entry| {
                let date = NaiveDate::parse_from_str(&entry.date, "%Y%m%d")?;
                hub_day(date, timezone)
            })
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

/// A day on a camera hub's calendar that has recordings for a camera paired to it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingDateHubResult {
    /// The day, on the hub's calendar.
    pub date: NaiveDate,
    /// When the day starts on the hub, in UTC.
    pub start_time: DateTime<Utc>,
    /// The last second of the day on the hub, in UTC.
    pub end_time: DateTime<Utc>,
}

/// Turns a day on the hub's calendar into the range of time it covers, from
/// local midnight to a second before the next local midnight.
fn hub_day(date: NaiveDate, timezone: Tz) -> Result<RecordingDateHubResult, anyhow::Error> {
    let next_date = date
        .succ_opt()
        .ok_or_else(|| anyhow::anyhow!("{date} is the last date that can be represented"))?;

    Ok(RecordingDateHubResult {
        date,
        start_time: local_midnight(date, timezone),
        end_time: local_midnight(next_date, timezone) - Duration::seconds(1),
    })
}

/// Local midnight of `date` in `timezone`, in UTC. A midnight that the clocks
/// go back over happens twice; the earlier of the two starts the day. A
/// midnight that the clocks jump forward over never happens, so the day starts
/// where they jump.
fn local_midnight(date: NaiveDate, timezone: Tz) -> DateTime<Utc> {
    let midnight = date.and_time(NaiveTime::MIN);

    match timezone.from_local_datetime(&midnight) {
        MappedLocalTime::Single(time) => time.to_utc(),
        MappedLocalTime::Ambiguous(earliest, _) => earliest.to_utc(),
        MappedLocalTime::None => {
            // Reading midnight with the offset from before the jump lands on
            // the moment the clocks jumped.
            let offset = timezone
                .offset_from_utc_datetime(&(midnight - Duration::days(1)))
                .fix();

            (midnight - offset).and_utc()
        }
    }
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

    /// The day `date` covers on a hub in `timezone`, as `(start_time, end_time)`.
    fn day_range(timezone: Tz, date: &str) -> (String, String) {
        let date = date.parse::<NaiveDate>().unwrap();
        let day = hub_day(date, timezone).unwrap();

        assert_eq!(day.date, date);

        (day.start_time.to_rfc3339(), day.end_time.to_rfc3339())
    }

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
        let dates = parsed.dates(Tz::Europe__Paris).unwrap();

        assert_eq!(
            dates.iter().map(|day| day.date).collect::<Vec<_>>(),
            vec![
                NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 17).unwrap()
            ]
        );
        assert_eq!(
            dates[0].start_time,
            "2026-07-31T22:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            dates[0].end_time,
            "2026-08-01T21:59:59Z".parse::<DateTime<Utc>>().unwrap()
        );
    }

    #[test]
    fn test_invalid_recording_date_is_an_error() {
        let json =
            r#"{"playback": {"search_results": [{"search_results_1": {"date": "not-a-date"}}]}}"#;

        let parsed: RecordingDateListHubResultRaw = serde_json::from_str(json).unwrap();

        assert!(parsed.dates(Tz::Europe__Paris).is_err());
    }

    #[test]
    fn test_hub_day_covers_the_local_day() {
        // Paris is two hours ahead of UTC in September.
        assert_eq!(
            day_range(Tz::Europe__Paris, "2026-09-08"),
            (
                "2026-09-07T22:00:00+00:00".to_string(),
                "2026-09-08T21:59:59+00:00".to_string()
            )
        );
    }

    #[test]
    fn test_hub_day_covers_a_day_the_clocks_go_back_on() {
        // The clocks go back at 03:00 local on 2026-10-25, a 25 hour day.
        assert_eq!(
            day_range(Tz::Europe__Paris, "2026-10-25"),
            (
                "2026-10-24T22:00:00+00:00".to_string(),
                "2026-10-25T22:59:59+00:00".to_string()
            )
        );
    }

    #[test]
    fn test_hub_day_covers_a_day_the_clocks_go_forward_on() {
        // The clocks go forward at 02:00 local on 2026-03-29, a 23 hour day.
        assert_eq!(
            day_range(Tz::Europe__Paris, "2026-03-29"),
            (
                "2026-03-28T23:00:00+00:00".to_string(),
                "2026-03-29T21:59:59+00:00".to_string()
            )
        );
    }

    #[test]
    fn test_hub_day_starts_where_a_skipped_midnight_jumps_to() {
        // Chile's clocks go forward at midnight on 2026-09-06, so the day
        // starts at 01:00 local, when they jump.
        assert_eq!(
            day_range(Tz::America__Santiago, "2026-09-06"),
            (
                "2026-09-06T04:00:00+00:00".to_string(),
                "2026-09-07T02:59:59+00:00".to_string()
            )
        );
    }

    #[test]
    fn test_hub_day_starts_at_the_first_of_two_midnights() {
        // Cuba's clocks go back at 01:00 local on 2026-11-01, so midnight
        // happens twice and the first one starts the day.
        assert_eq!(
            day_range(Tz::America__Havana, "2026-11-01"),
            (
                "2026-11-01T04:00:00+00:00".to_string(),
                "2026-11-02T04:59:59+00:00".to_string()
            )
        );
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
