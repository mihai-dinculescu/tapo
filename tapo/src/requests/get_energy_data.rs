use serde::Serialize;

use crate::requests::EnergyDataInterval;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetEnergyDataParams {
    start_timestamp: u64,
    end_timestamp: u64,
    interval: u64,
}

impl GetEnergyDataParams {
    // safe: and_hms_opt with fixed valid HMS values always returns Some.
    // safe: and_local_timezone with midnight/23:59:59 is unambiguous in practice
    // (DST transitions do not occur at these times in any known timezone).
    pub fn new(interval: EnergyDataInterval) -> Self {
        let timezone = chrono::Local::now().timezone();

        match interval {
            EnergyDataInterval::Hourly {
                start_date,
                end_date,
            } => Self {
                start_timestamp: start_date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(timezone)
                    .unwrap()
                    .timestamp() as u64,
                end_timestamp: end_date
                    .and_hms_opt(23, 59, 59)
                    .unwrap()
                    .and_local_timezone(timezone)
                    .unwrap()
                    .timestamp() as u64,
                interval: 60,
            },
            EnergyDataInterval::Daily { start_date } => {
                let timestamp = start_date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(timezone)
                    .unwrap()
                    .timestamp() as u64;
                Self {
                    start_timestamp: timestamp,
                    end_timestamp: timestamp,
                    interval: 1440,
                }
            }
            EnergyDataInterval::Monthly { start_date } => {
                let timestamp = start_date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(timezone)
                    .unwrap()
                    .timestamp() as u64;
                Self {
                    start_timestamp: timestamp,
                    end_timestamp: timestamp,
                    interval: 43200,
                }
            }
        }
    }
}
