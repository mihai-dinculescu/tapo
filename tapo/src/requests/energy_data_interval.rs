use chrono::NaiveDate;

/// Energy data interval.
pub enum EnergyDataInterval {
    /// Hourly interval. `start_date` and `end_date` are an inclusive interval that must not be greater than 8 days.
    Hourly {
        /// Interval start date.
        start_date: NaiveDate,
        /// Interval end date. Inclusive.
        /// Must not be greater by more than 8 days than `start_date`.
        end_date: NaiveDate,
    },
    /// Daily interval. `start_date` must be the first day of a quarter.
    Daily {
        /// Must be the first day of a quarter.
        start_date: NaiveDate,
    },
    /// Monthly interval. `start_date` must be the first day of a year.
    Monthly {
        /// Must be the first day of a year.
        start_date: NaiveDate,
    },
}
