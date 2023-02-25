use time::{Date, OffsetDateTime};

/// Energy data interval.
pub enum EnergyDataInterval {
    /// Hourly interval. `start_datetime` and `end_datetime` are inclusive and the difference between them must not be greater than 8 days.
    Hourly {
        start_datetime: OffsetDateTime,
        end_datetime: OffsetDateTime,
    },
    /// Daily interval. `start_date` must be the first day of a quarter.
    Daily {
        /// Must be the first day of a quarter.
        start_date: Date,
    },
    /// Monthly interval. `start_date` must be the first day of a year.
    Monthly {
        /// Must be the first day of a year.
        start_date: Date,
    },
}
