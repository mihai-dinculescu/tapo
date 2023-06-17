use time::{Date, OffsetDateTime};

/// Energy data interval.
pub enum EnergyDataInterval {
    /// Hourly interval. `start_datetime` is `end_datetime` is an inclusive interval that must not be greater than 8 days.
    Hourly {
        /// Interval start date and time.
        start_datetime: OffsetDateTime,
        /// Interval end date and time. Inclusive.
        /// Must not be greater by more than 8 days than `start_datetime`.
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
