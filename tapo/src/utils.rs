use std::str::FromStr;

use chrono::{
    DateTime, Duration, MappedLocalTime, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc,
};
use serde::{Deserialize, Deserializer};

use crate::error::Error;

pub fn der_tapo_datetime_format<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    if !s.contains('T') {
        s = s.replace(' ', "T");
    }
    let value = NaiveDateTime::from_str(&s).map_err(serde::de::Error::custom)?;

    Ok(value)
}

pub fn ok_or_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    Ok(Deserialize::deserialize(deserializer).unwrap_or_default())
}

/// Deserialize a boolean from either a JSON bool or an integer (0/1).
pub(crate) fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        _ => Err(serde::de::Error::custom("expected bool or integer")),
    }
}

/// Deserialize a UTC time from either a Unix timestamp in seconds or an RFC 3339 string.
pub(crate) fn datetime_from_unix_seconds_or_rfc3339<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(n) => n
            .as_i64()
            .and_then(|seconds| DateTime::from_timestamp(seconds, 0))
            .ok_or_else(|| serde::de::Error::custom("Unix timestamp out of range")),
        serde_json::Value::String(s) => DateTime::parse_from_rfc3339(&s)
            .map(|time| time.to_utc())
            .map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom(
            "expected Unix timestamp or RFC 3339 string",
        )),
    }
}

/// Deserialize an optional boolean from either a JSON bool, an integer (0/1), or absence.
pub(crate) fn option_bool_from_int_or_bool<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Bool(b)) => Ok(Some(b)),
        Some(serde_json::Value::Number(n)) => Ok(Some(n.as_i64().unwrap_or(0) != 0)),
        _ => Err(serde::de::Error::custom("expected bool, integer, or null")),
    }
}

/// Converts `time` to a Unix timestamp in seconds, as the camera hubs expect it.
/// `field` names the argument in the error returned for a time before 1970.
pub(crate) fn unix_timestamp_seconds(field: &str, time: DateTime<Utc>) -> Result<u64, Error> {
    u64::try_from(time.timestamp()).map_err(|_| Error::Validation {
        field: field.to_string(),
        message: "Must not be before 1970-01-01T00:00:00Z".to_string(),
    })
}

/// Local midnight of `date` in `timezone`, in UTC. A midnight that the clocks
/// go back over happens twice; the earlier of the two starts the day. A
/// midnight that the clocks jump forward over never happens, so the day starts
/// where they jump.
pub(crate) fn local_midnight<Tz: TimeZone>(date: NaiveDate, timezone: &Tz) -> DateTime<Utc> {
    let midnight = date.and_time(NaiveTime::MIN);

    match timezone.from_local_datetime(&midnight) {
        MappedLocalTime::Single(time) => time.to_utc(),
        MappedLocalTime::Ambiguous(earliest, _) => earliest.to_utc(),
        // When the jump starts at midnight, this lands on the moment the
        // clocks jumped.
        MappedLocalTime::None => skipped_local_time(midnight, timezone),
    }
}

/// `time`, a local time that the clocks jump forward over in `timezone`, read
/// with the offset from before the jump, in UTC. It lands as far past the jump
/// as `time` is past the start of the skipped range.
pub(crate) fn skipped_local_time<Tz: TimeZone>(
    time: NaiveDateTime,
    timezone: &Tz,
) -> DateTime<Utc> {
    let offset = timezone
        .offset_from_utc_datetime(&(time - Duration::days(1)))
        .fix();

    (time - offset).and_utc()
}

/// The range of time `date` covers in `timezone`, as `(start, end)` in UTC,
/// from local midnight to a second before the next local midnight.
pub(crate) fn local_day<Tz: TimeZone>(
    date: NaiveDate,
    timezone: &Tz,
) -> Result<(DateTime<Utc>, DateTime<Utc>), anyhow::Error> {
    let next_date = date
        .succ_opt()
        .ok_or_else(|| anyhow::anyhow!("{date} is the last date that can be represented"))?;

    Ok((
        local_midnight(date, timezone),
        local_midnight(next_date, timezone) - Duration::seconds(1),
    ))
}
