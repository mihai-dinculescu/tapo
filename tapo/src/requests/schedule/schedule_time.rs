use serde::{Deserialize, Serialize};

/// When a schedule rule fires within a day.
///
/// Each variant carries only the fields that apply to it, so a clock rule
/// cannot hold a sunrise offset and vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, eq, hash, frozen)
)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleTime {
    /// At a wall-clock time, in the device's own timezone.
    Clock {
        /// Hour of the day, `0..=23`.
        hour: u8,
        /// Minute of the hour, `0..=59`.
        minute: u8,
    },
    /// At an offset from civil sunrise, computed by the device.
    Sunrise {
        /// Minutes from sunrise; negative fires before it.
        offset_minutes: i16,
    },
    /// At an offset from civil sunset, computed by the device.
    Sunset {
        /// Minutes from sunset; negative fires before it.
        offset_minutes: i16,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_values_hash_equally() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ScheduleTime::Clock {
            hour: 6,
            minute: 30,
        });
        set.insert(ScheduleTime::Clock {
            hour: 6,
            minute: 30,
        });
        assert_eq!(set.len(), 1);
    }
}
