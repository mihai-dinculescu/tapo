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
                start_datetime,
                end_datetime,
            } => Self {
                start_timestamp: start_datetime.unix_timestamp() as u64,
                end_timestamp: end_datetime.unix_timestamp() as u64,
                interval: 60,
            },
            EnergyDataInterval::Daily { start_date } => Self {
                start_timestamp: start_date.midnight().assume_utc().unix_timestamp() as u64,
                end_timestamp: start_date.midnight().assume_utc().unix_timestamp() as u64,
                interval: 1440,
            },
            EnergyDataInterval::Monthly { start_date } => Self {
                start_timestamp: start_date.midnight().assume_utc().unix_timestamp() as u64,
                end_timestamp: start_date.midnight().assume_utc().unix_timestamp() as u64,
                interval: 43200,
            },
        }
    }
}
