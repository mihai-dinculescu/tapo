use anyhow::Context as _;
use chrono::{DateTime, Days, Months, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::TapoResponseExt;
use crate::utils::{der_tapo_datetime_format, local_midnight, local_time, skipped_local_time};

/// Energy data result for the requested [`crate::requests::PowerDataInterval`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct EnergyDataResult {
    /// Local time of the device.
    pub local_time: NaiveDateTime,
    /// Start date and time of this result in UTC.
    /// This value is provided in the `get_energy_data` request and is passed through.
    /// Note that it may not align with the returned data if the method is used beyond its specified capabilities.
    pub start_date_time: DateTime<Utc>,
    /// List of energy data entries, up to the device's local time. It is shorter
    /// than the requested interval when the interval has not ended yet, and
    /// empty when it has not started.
    pub entries: Vec<EnergyDataIntervalResult>,
    /// Interval length in minutes.
    pub interval_length: u64,
}

impl TapoResponseExt for EnergyDataResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(EnergyDataResult);

/// Energy data result for a specific interval.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct EnergyDataIntervalResult {
    /// Start date and time of this interval in UTC.
    pub start_date_time: DateTime<Utc>,
    /// Energy in Watt Hours (Wh).
    pub energy: u64,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(EnergyDataIntervalResult);

#[derive(Debug, Deserialize)]
pub(crate) struct EnergyDataResultRaw {
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    pub data: Vec<u64>,
    pub start_timestamp: i64,
    pub interval: u64,
}

impl TapoResponseExt for EnergyDataResultRaw {}

enum Step {
    Hour,
    Day,
    Month,
}

impl EnergyDataResultRaw {
    /// Dates the entries in `timezone`, the device's, and leaves out the ones
    /// that start after the device's local time. The device returns those as
    /// 0 Wh, which would read as no energy used.
    pub fn in_timezone<Tz: TimeZone>(self, timezone: &Tz) -> Result<EnergyDataResult, Error> {
        let start = timezone
            .timestamp_opt(self.start_timestamp, 0)
            .single()
            .context("Failed to map start_timestamp to local time")?;
        let start_date_time = start.to_utc();

        let step = match self.interval {
            60 => Step::Hour,
            1440 => Step::Day,
            43200 => Step::Month,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported interval duration: {} minutes",
                    self.interval
                )
                .into());
            }
        };

        // Every interval is stepped by calendar date, because its length in
        // hours changes with DST.
        let start_date = start.date_naive();
        // During a repeated hour, the later reading keeps the current entry.
        // A device whose offsets disagree with `timezone` can report a time
        // that `timezone` skips. Reading it with the offset from before the
        // jump also gives the later reading.
        let now = timezone
            .from_local_datetime(&self.local_time)
            .latest()
            .map_or_else(
                || skipped_local_time(self.local_time, timezone),
                |now| now.to_utc(),
            );
        let mut entries = Vec::with_capacity(self.data.len());

        for (index, energy) in self.data.into_iter().enumerate() {
            let entry_start = match step {
                Step::Hour => local_time(
                    start_date
                        .checked_add_days(Days::new((index / 24) as u64))
                        .context("Energy data date is out of range")?
                        .and_hms_opt((index % 24) as u32, 0, 0)
                        .context("Energy data time is out of range")?,
                    timezone,
                ),
                Step::Day => {
                    let date = start_date
                        .checked_add_days(Days::new(index as u64))
                        .context("Energy data date is out of range")?;
                    local_midnight(date, timezone)
                }
                Step::Month => {
                    let date = start_date
                        .checked_add_months(Months::new(index as u32))
                        .context("Energy data date is out of range")?;
                    local_midnight(date, timezone)
                }
            };

            if entry_start > now {
                break;
            }

            entries.push(EnergyDataIntervalResult {
                start_date_time: entry_start,
                energy,
            });
        }

        Ok(EnergyDataResult {
            local_time: self.local_time,
            start_date_time,
            entries,
            interval_length: self.interval,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime, Timelike as _};
    use chrono_tz::America::Santiago;
    use chrono_tz::Europe::Berlin;

    use super::*;

    fn utc(month: u32, day: u32, hour: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(2026, month, day)
            .unwrap()
            .and_hms_opt(hour, 0, 0)
            .unwrap()
            .and_utc()
    }

    fn raw(start_date_time: DateTime<Utc>, count: usize, interval: u64) -> EnergyDataResultRaw {
        EnergyDataResultRaw {
            local_time: NaiveDate::from_ymd_opt(2100, 1, 1)
                .unwrap()
                .and_time(NaiveTime::MIN),
            data: vec![0; count],
            start_timestamp: start_date_time.timestamp(),
            interval,
        }
    }

    fn local_dates_and_times(result: &EnergyDataResult) -> Vec<NaiveDateTime> {
        result
            .entries
            .iter()
            .map(|entry| entry.start_date_time.with_timezone(&Santiago).naive_local())
            .collect()
    }

    #[test]
    fn test_daily_entries_start_at_local_midnight_across_dst_changes() {
        let at = |month, day, hour| {
            NaiveDate::from_ymd_opt(2026, month, day)
                .unwrap()
                .and_hms_opt(hour, 0, 0)
                .unwrap()
        };

        // Santiago skips midnight on 2026-09-06, so that day starts at 01:00.
        let expected = vec![at(9, 4, 0), at(9, 5, 0), at(9, 6, 1), at(9, 7, 0)];
        let start = Santiago.from_local_datetime(&at(9, 4, 0)).unwrap().to_utc();
        let result = raw(start, 4, 1440).in_timezone(&Santiago).unwrap();
        assert_eq!(local_dates_and_times(&result), expected);

        // Santiago repeats 23:00 on 2026-04-04, so that day has 25 hours.
        let expected = vec![at(4, 3, 0), at(4, 4, 0), at(4, 5, 0), at(4, 6, 0)];
        let start = Santiago.from_local_datetime(&at(4, 3, 0)).unwrap().to_utc();
        let result = raw(start, 4, 1440).in_timezone(&Santiago).unwrap();
        assert_eq!(local_dates_and_times(&result), expected);
    }

    #[test]
    fn test_hourly_entries_follow_local_calendar_hours_across_a_repeated_hour() {
        let date = |day| NaiveDate::from_ymd_opt(2026, 10, day).unwrap();

        // Berlin repeats 02:00 on 2026-10-25, so that day has 25 hours.
        let start = utc(10, 24, 22);
        let result = raw(start, 48, 60).in_timezone(&Berlin).unwrap();

        assert_eq!(result.entries.len(), 48);
        for (index, entry) in result.entries.iter().enumerate() {
            let local = entry.start_date_time.with_timezone(&Berlin).naive_local();
            assert_eq!(local.date(), date(25 + (index / 24) as u32));
            assert_eq!(local.hour(), (index % 24) as u32);
        }
        // The repeated hour is dated at its earlier occurrence.
        assert_eq!(result.entries[2].start_date_time, utc(10, 25, 0));
        assert_eq!(result.entries[23].start_date_time, utc(10, 25, 22));
        assert_eq!(result.entries[24].start_date_time, utc(10, 25, 23));
    }

    #[test]
    fn test_hourly_entry_for_the_skipped_hour_lands_on_the_jump() {
        let date = |day| NaiveDate::from_ymd_opt(2026, 3, day).unwrap();

        // Berlin skips 02:00 on 2026-03-29, so that day has 23 hours.
        let start = utc(3, 28, 23);
        let result = raw(start, 24, 60).in_timezone(&Berlin).unwrap();

        assert_eq!(result.entries[0].start_date_time, utc(3, 28, 23));
        assert_eq!(result.entries[1].start_date_time, utc(3, 29, 0));
        assert_eq!(result.entries[2].start_date_time, utc(3, 29, 1));
        assert_eq!(
            result.entries[3]
                .start_date_time
                .with_timezone(&Berlin)
                .naive_local(),
            date(29).and_hms_opt(3, 0, 0).unwrap()
        );
    }

    #[test]
    fn test_monthly_entries_start_on_the_first_of_each_month() {
        let first_day = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let start = local_midnight(first_day, &Santiago);

        let result = raw(start, 12, 43200).in_timezone(&Santiago).unwrap();

        for (index, date_time) in local_dates_and_times(&result).into_iter().enumerate() {
            assert_eq!(date_time.date(), first_day + Months::new(index as u32));
            assert_eq!(date_time.hour(), 0);
        }
    }

    #[test]
    fn test_entries_after_the_local_time_are_left_out() {
        let date = |day| NaiveDate::from_ymd_opt(2026, 10, day).unwrap();
        let start = local_midnight(date(1), &Santiago);
        let mut quarter = raw(start, 92, 1440);
        quarter.local_time = date(18).and_hms_opt(12, 0, 0).unwrap();

        let result = quarter.in_timezone(&Santiago).unwrap();

        let dates: Vec<_> = local_dates_and_times(&result)
            .into_iter()
            .map(|date_time| date_time.date())
            .collect();
        assert_eq!(dates, (1..=18).map(date).collect::<Vec<_>>());
    }

    #[test]
    fn test_skipped_local_time_is_read_with_the_offset_from_before_the_jump() {
        let at = |day, hour| {
            NaiveDate::from_ymd_opt(2026, 9, day)
                .unwrap()
                .and_hms_opt(hour, 0, 0)
                .unwrap()
        };

        // Santiago skips 00:00 to 01:00 on 2026-09-06.
        let start = local_midnight(at(2, 0).date(), &Santiago);
        let mut days = raw(start, 5, 1440);
        days.local_time = at(6, 0).with_minute(30).unwrap();

        let result = days.in_timezone(&Santiago).unwrap();

        assert_eq!(
            local_dates_and_times(&result),
            vec![at(2, 0), at(3, 0), at(4, 0), at(5, 0), at(6, 1)]
        );
    }

    #[test]
    fn test_unsupported_interval_is_an_error() {
        assert!(raw(Utc::now(), 1, 15).in_timezone(&Santiago).is_err());
        assert!(raw(Utc::now(), 0, 15).in_timezone(&Santiago).is_err());
    }
}
