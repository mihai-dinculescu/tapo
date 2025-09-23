use chrono::{DateTime, Duration, Timelike as _, Utc};
use serde::Serialize;

use crate::requests::PowerDataInterval;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetPowerDataParams {
    start_timestamp: u64,
    end_timestamp: u64,
    interval: u64,
}

impl GetPowerDataParams {
    pub fn new(interval: PowerDataInterval) -> Self {
        match interval {
            PowerDataInterval::Every5Minutes {
                start_date_time,
                end_date_time,
            } => Self {
                start_timestamp: get_5_minute_interval_start(start_date_time).timestamp() as u64,
                end_timestamp: get_5_minute_interval_start(end_date_time).timestamp() as u64,
                interval: 5,
            },
            PowerDataInterval::Hourly {
                start_date_time,
                end_date_time,
            } => Self {
                start_timestamp: get_hourly_interval_start(start_date_time).timestamp() as u64,
                end_timestamp: get_hourly_interval_start(end_date_time).timestamp() as u64,
                interval: 60,
            },
        }
    }
}

fn get_5_minute_interval_start(date: DateTime<Utc>) -> DateTime<Utc> {
    // Seconds since start of the hour
    let secs_into_hour = (date.minute() as i64) * 60 + date.second() as i64;
    let rem = secs_into_hour % 300; // 300 = 5 * 60

    // If already exactly on a 5‑minute boundary (and second == 0) keep as-is.
    if rem == 0 && date.second() == 0 {
        return date
            .with_second(0)
            .expect("set second")
            .with_nanosecond(0)
            .expect("set nanos");
    }

    // Otherwise add the remaining seconds to reach the next 5‑minute boundary.
    let add = 300 - rem;
    let adjusted = date + Duration::seconds(add);

    adjusted
        .with_second(0)
        .expect("Failed to set second")
        .with_nanosecond(0)
        .expect("Failed to set nanos")
}

fn get_hourly_interval_start(date: DateTime<Utc>) -> DateTime<Utc> {
    // If already exactly on an hour boundary, keep it.
    if date.minute() == 0 && date.second() == 0 && date.nanosecond() == 0 {
        return date;
    }

    // Truncate to the current hour then add one hour (chrono handles day rollover).
    let hour_start = date
        .with_minute(0)
        .expect("Failed to set minute")
        .with_second(0)
        .expect("Failed to set second")
        .with_nanosecond(0)
        .expect("Failed to set nanos");

    hour_start + Duration::hours(1)
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike as _, TimeZone as _};

    use super::*;

    #[test]
    fn test_get_5_minute_interval_start() {
        // Exact 5-minute interval, no change
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 15, 0, 0).unwrap();
        let adjusted = get_5_minute_interval_start(date);
        assert_eq!(adjusted.hour(), 15);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);

        // Non-exact 5-minute interval, round up to next 5-minute mark
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 14, 3, 45).unwrap();
        let adjusted = get_5_minute_interval_start(date);
        assert_eq!(adjusted.hour(), 14);
        assert_eq!(adjusted.minute(), 5);
        assert_eq!(adjusted.second(), 0);

        // Non-exact 5-minute interval, round up to next hour
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 14, 57, 30).unwrap();
        let adjusted = get_5_minute_interval_start(date);
        assert_eq!(adjusted.hour(), 15);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);

        // Non-exact 5-minute interval, round up to next day
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 23, 58, 59).unwrap();
        let adjusted = get_5_minute_interval_start(date);
        assert_eq!(adjusted.hour(), 0);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);
        assert_eq!(adjusted.day(), 2);
    }

    #[test]
    fn test_get_hourly_interval_start() {
        // Exact hour, no change
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 15, 0, 0).unwrap();
        let adjusted = get_hourly_interval_start(date);
        assert_eq!(adjusted.hour(), 15);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);

        // Non-exact hour, round up to next hour
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 14, 15, 0).unwrap();
        let adjusted = get_hourly_interval_start(date);
        assert_eq!(adjusted.hour(), 15);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);

        // Non-exact hour, round up to next hour
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 14, 0, 30).unwrap();
        let adjusted = get_hourly_interval_start(date);
        assert_eq!(adjusted.hour(), 15);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);

        // Non-exact hour, round up to next day
        let date = Utc.with_ymd_and_hms(2025, 1, 1, 23, 30, 59).unwrap();
        let adjusted = get_hourly_interval_start(date);
        assert_eq!(adjusted.hour(), 0);
        assert_eq!(adjusted.minute(), 0);
        assert_eq!(adjusted.second(), 0);
        assert_eq!(adjusted.day(), 2);
    }
}
