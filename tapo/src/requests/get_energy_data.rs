use serde::Serialize;

use crate::requests::EnergyDataInterval;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetEnergyDataParams {
    start_timestamp: u64,
    end_timestamp: u64,
    interval: u64,
}

impl GetEnergyDataParams {
    pub fn new(interval: EnergyDataInterval) -> Self {
        match interval {
            EnergyDataInterval::Hourly {
                start_date,
                end_date,
            } => Self {
                start_timestamp: start_date.and_hms_opt(0, 0, 0).unwrap().timestamp() as u64,
                end_timestamp: end_date.and_hms_opt(23, 59, 59).unwrap().timestamp() as u64,
                interval: 60,
            },
            EnergyDataInterval::Daily { start_date } => {
                let timestamp = start_date.and_hms_opt(0, 0, 0).unwrap().timestamp() as u64;
                Self {
                    start_timestamp: timestamp,
                    end_timestamp: timestamp,
                    interval: 1440,
                }
            }
            EnergyDataInterval::Monthly { start_date } => {
                let timestamp = start_date.and_hms_opt(0, 0, 0).unwrap().timestamp() as u64;
                Self {
                    start_timestamp: timestamp,
                    end_timestamp: timestamp,
                    interval: 43200,
                }
            }
        }
    }
}
