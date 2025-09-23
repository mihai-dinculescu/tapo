use chrono::{DateTime, Utc};

/// Power data interval.
pub enum PowerDataInterval {
    /// Every 5 minutes interval. `start_date_time` and `end_date_time` describe an exclusive interval.
    /// If the result would yield more than 144 entries (i.e. 12 hours),
    /// the `end_date_time` will be adjusted to an earlier date and time.
    Every5Minutes {
        /// Start date and time in UTC.
        /// If it is not aligned to the 5 minute mark, it will be rounded to the next 5 minute mark.
        start_date_time: DateTime<Utc>,
        /// End date and time in UTC.
        end_date_time: DateTime<Utc>,
    },
    /// Hourly interval. `start_date_time` and `end_date_time` describe an exclusive interval.
    /// If the result would yield more than 144 entries (i.e. 6 days),
    /// the `end_date_time` will be adjusted to an earlier date and time.
    Hourly {
        /// Start date and time in UTC.
        /// If it is not aligned to the hour mark, it will be rounded to the next hour mark.
        start_date_time: DateTime<Utc>,
        /// End date and time in UTC.
        end_date_time: DateTime<Utc>,
    },
}
