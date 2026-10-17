use chrono::TimeZone;
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
                let (start, _) = local_day(start_date, timezone)?;
                let (_, end) = local_day(end_date, timezone)?;

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
    use chrono_tz::America::Santiago;

    use super::*;

    #[test]
    fn test_hourly_covers_whole_days_across_dst_changes() {
        // 2026-09-06 has 23 hours in Santiago, 2026-04-04 has 25.
        for (date, hours) in [((2026, 9, 6), 23), ((2026, 4, 4), 25)] {
            let date = NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap();

            let params = GetEnergyDataParams::new(
                EnergyDataInterval::Hourly {
                    start_date: date,
                    end_date: date,
                },
                &Santiago,
            )
            .unwrap();

            assert_eq!(
                params.end_timestamp - params.start_timestamp,
                hours * 3600 - 1
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
