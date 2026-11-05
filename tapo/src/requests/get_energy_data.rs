use chrono::{Duration, TimeZone};
use serde::Serialize;

use crate::error::Error;
use crate::requests::EnergyDataInterval;
use crate::utils::{local_day, local_midnight};

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetEnergyDataParams {
    start_timestamp: u64,
    end_timestamp: u64,
    interval: u64,
}

impl GetEnergyDataParams {
    /// Reads the interval's dates in `timezone`, the device's.
    pub fn new<Tz: TimeZone>(interval: EnergyDataInterval, timezone: &Tz) -> Result<Self, Error> {
        let params = match interval {
            EnergyDataInterval::Hourly {
                start_date,
                end_date,
            } => {
                // The device returns as many whole 24-hour days as it takes to
                // cover the span, starting from local midnight of the start's
                // date. A day the clocks go back over is longer than 24 hours,
                // so a span from the first midnight to the last second of
                // `end_date` would count one day too many. Starting the span
                // as much later as the days run over 24 hours each keeps it
                // within the requested number of days, and the device still
                // returns the first day from midnight. The overrun is a few
                // hours, so the start stays inside `start_date`.
                let (start, _) = local_day(start_date, timezone)?;
                let (_, end) = local_day(end_date, timezone)?;
                let days = (end_date - start_date).num_days() + 1;
                let start = start.max(end + Duration::seconds(1) - Duration::days(days));

                Self {
                    start_timestamp: start.timestamp() as u64,
                    end_timestamp: end.timestamp() as u64,
                    interval: 60,
                }
            }
            EnergyDataInterval::Daily { start_date } => {
                let timestamp = local_midnight(start_date, timezone).timestamp() as u64;
                Self {
                    start_timestamp: timestamp,
                    end_timestamp: timestamp,
                    interval: 1440,
                }
            }
            EnergyDataInterval::Monthly { start_date } => {
                let timestamp = local_midnight(start_date, timezone).timestamp() as u64;
                Self {
                    start_timestamp: timestamp,
                    end_timestamp: timestamp,
                    interval: 43200,
                }
            }
        };

        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime, Utc};
    use chrono_tz::America::{New_York, Santiago};
    use chrono_tz::Antarctica::Troll;
    use chrono_tz::Tz;

    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn hourly(start_date: NaiveDate, end_date: NaiveDate, timezone: &Tz) -> GetEnergyDataParams {
        GetEnergyDataParams::new(
            EnergyDataInterval::Hourly {
                start_date,
                end_date,
            },
            timezone,
        )
        .unwrap()
    }

    /// The number of days of hourly data that the device returns for `params`.
    /// As measured on a P110M, it returns as many whole 24-hour days as it
    /// takes to cover `end_timestamp - start_timestamp`, from local midnight of
    /// the start's date: 86400 seconds is one day, 86401 seconds is two.
    fn days_returned(params: &GetEnergyDataParams) -> u64 {
        (params.end_timestamp - params.start_timestamp).div_ceil(24 * 3600)
    }

    #[test]
    fn test_hourly_asks_for_one_day_whatever_its_length() {
        // Each day's length, and when its clocks change.
        for (day, timezone, why) in [
            (date(2026, 10, 31), New_York, "24 hours, control"),
            (date(2026, 11, 1), New_York, "25 hours, back at 2 a.m."),
            (date(2026, 9, 6), Santiago, "23 hours, skips midnight"),
            (date(2026, 4, 4), Santiago, "25 hours, back at midnight"),
            (
                date(2026, 10, 25),
                Troll,
                "26 hours, back 2 hours at 3 a.m.",
            ),
        ] {
            let days = days_returned(&hourly(day, day, &timezone));
            assert_eq!(days, 1, "{day} in {timezone}: {why}");
        }
    }

    #[test]
    fn test_hourly_asks_for_eight_days_across_the_clocks_going_back() {
        // The last of these 8 days, 2026-11-01, has 25 hours in New York.
        let params = hourly(date(2026, 10, 25), date(2026, 11, 1), &New_York);

        assert_eq!(days_returned(&params), 8);
    }

    #[test]
    fn test_hourly_covers_whole_days_across_dst_changes() {
        // 2026-06-01 has 24 hours in Santiago, 2026-09-06 has 23 and
        // 2026-04-04 has 25. Only the 25-hour day starts after its midnight,
        // by the hour it runs over.
        for (day, overrun_hours) in [
            (date(2026, 6, 1), 0),
            (date(2026, 9, 6), 0),
            (date(2026, 4, 4), 1),
        ] {
            let params = hourly(day, day, &Santiago);
            let local = |timestamp: u64| {
                Santiago
                    .timestamp_opt(timestamp as i64, 0)
                    .unwrap()
                    .naive_local()
            };

            // The start lies inside the day, and the end is its last second.
            assert_eq!(
                params.start_timestamp as i64,
                local_midnight(day, &Santiago).timestamp() + overrun_hours * 3600,
                "{day}"
            );
            assert_eq!(local(params.start_timestamp).date(), day);
            assert_eq!(
                local(params.end_timestamp + 1),
                day.succ_opt().unwrap().and_time(NaiveTime::MIN)
            );
        }
    }

    #[test]
    fn test_daily_with_skipped_midnight_starts_when_clocks_resume() {
        let date = NaiveDate::from_ymd_opt(2026, 9, 6).unwrap();

        let params =
            GetEnergyDataParams::new(EnergyDataInterval::Daily { start_date: date }, &Santiago)
                .unwrap();

        let expected = date
            .and_time(NaiveTime::MIN)
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp()
            + 4 * 3600;
        assert_eq!(params.start_timestamp, expected as u64);
    }
}
