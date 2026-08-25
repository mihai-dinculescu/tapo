use std::fmt;
use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

/// The days of the week a repeating [`ScheduleRule`](super::ScheduleRule)
/// fires on.
///
/// Combine the individual days with `|`, or use one of the preset groups:
///
/// ```
/// use tapo::requests::DaysOfWeek;
///
/// let midweek = DaysOfWeek::MON | DaysOfWeek::WED;
/// assert!(midweek.contains(DaysOfWeek::MON));
/// assert!(!midweek.contains(DaysOfWeek::FRI));
///
/// assert_eq!(DaysOfWeek::WEEKEND, DaysOfWeek::SUN | DaysOfWeek::SAT);
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, eq, hash, frozen)
)]
#[serde(transparent)]
pub struct DaysOfWeek(u8);

impl DaysOfWeek {
    /// No days. A repeating rule with no days would never fire, so the
    /// `*_weekly` builders reject it.
    pub const NONE: Self = Self(0);
    /// Sunday.
    pub const SUN: Self = Self(1 << 0);
    /// Monday.
    pub const MON: Self = Self(1 << 1);
    /// Tuesday.
    pub const TUE: Self = Self(1 << 2);
    /// Wednesday.
    pub const WED: Self = Self(1 << 3);
    /// Thursday.
    pub const THU: Self = Self(1 << 4);
    /// Friday.
    pub const FRI: Self = Self(1 << 5);
    /// Saturday.
    pub const SAT: Self = Self(1 << 6);
    /// Monday through Friday.
    pub const WEEKDAYS: Self =
        Self(Self::MON.0 | Self::TUE.0 | Self::WED.0 | Self::THU.0 | Self::FRI.0);
    /// Saturday and Sunday.
    pub const WEEKEND: Self = Self(Self::SUN.0 | Self::SAT.0);
    /// Every day of the week.
    pub const EVERY_DAY: Self = Self(Self::WEEKDAYS.0 | Self::WEEKEND.0);

    /// Builds a set from a device bitmask, ignoring any bits above Saturday.
    /// Only bits 0..=6 are meaningful to the device, so this is the inverse
    /// of [`DaysOfWeek::bits`].
    ///
    /// # Arguments
    ///
    /// * `bits` - a device bitmask; bit 0 is Sunday through bit 6, Saturday.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(bits & Self::EVERY_DAY.0)
    }
}

/// Methods with a `&self` receiver need no pyo3 attribute, so they are shared
/// with Python directly rather than through wrappers.
#[cfg_attr(feature = "python", pyo3::pymethods)]
impl DaysOfWeek {
    /// Returns the device bitmask for this set: bit 0 is Sunday through
    /// bit 6, Saturday.
    pub const fn bits(&self) -> u8 {
        self.0
    }

    /// Returns `true` if every day in `other` is also in this set.
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns `true` if this set contains no days.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl DaysOfWeek {
    /// The individual days, Sunday first, paired with their names.
    const fn all_days() -> [(Self, &'static str); 7] {
        [
            (Self::SUN, "SUN"),
            (Self::MON, "MON"),
            (Self::TUE, "TUE"),
            (Self::WED, "WED"),
            (Self::THU, "THU"),
            (Self::FRI, "FRI"),
            (Self::SAT, "SAT"),
        ]
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DaysOfWeek {
    // `#[classattr]` and `#[staticmethod]` are inner attributes of
    // `#[pymethods]`, so they cannot be written behind `cfg_attr` and these
    // cannot join the shared impl above. Distinct Rust names avoid colliding
    // with the consts and constructor they re-export.
    #[classattr]
    #[pyo3(name = "NONE")]
    const PY_NONE: DaysOfWeek = DaysOfWeek::NONE;
    #[classattr]
    #[pyo3(name = "SUN")]
    const PY_SUN: DaysOfWeek = DaysOfWeek::SUN;
    #[classattr]
    #[pyo3(name = "MON")]
    const PY_MON: DaysOfWeek = DaysOfWeek::MON;
    #[classattr]
    #[pyo3(name = "TUE")]
    const PY_TUE: DaysOfWeek = DaysOfWeek::TUE;
    #[classattr]
    #[pyo3(name = "WED")]
    const PY_WED: DaysOfWeek = DaysOfWeek::WED;
    #[classattr]
    #[pyo3(name = "THU")]
    const PY_THU: DaysOfWeek = DaysOfWeek::THU;
    #[classattr]
    #[pyo3(name = "FRI")]
    const PY_FRI: DaysOfWeek = DaysOfWeek::FRI;
    #[classattr]
    #[pyo3(name = "SAT")]
    const PY_SAT: DaysOfWeek = DaysOfWeek::SAT;
    #[classattr]
    #[pyo3(name = "WEEKDAYS")]
    const PY_WEEKDAYS: DaysOfWeek = DaysOfWeek::WEEKDAYS;
    #[classattr]
    #[pyo3(name = "WEEKEND")]
    const PY_WEEKEND: DaysOfWeek = DaysOfWeek::WEEKEND;
    #[classattr]
    #[pyo3(name = "EVERY_DAY")]
    const PY_EVERY_DAY: DaysOfWeek = DaysOfWeek::EVERY_DAY;

    #[staticmethod]
    #[pyo3(name = "from_bits_truncate")]
    fn py_from_bits_truncate(bits: u8) -> Self {
        Self::from_bits_truncate(bits)
    }

    fn __or__(&self, other: &Self) -> Self {
        *self | *other
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

impl fmt::Debug for DaysOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DaysOfWeek(")?;

        if self.is_empty() {
            f.write_str("NONE")?;
        } else {
            let mut first = true;
            for (day, name) in Self::all_days() {
                if self.contains(day) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    f.write_str(name)?;
                    first = false;
                }
            }
        }

        f.write_str(")")
    }
}

impl<'de> Deserialize<'de> for DaysOfWeek {
    /// Truncates rather than failing, so a firmware that sets a bit above
    /// Saturday cannot produce a set that would be sent back to the device.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_bits_truncate(u8::deserialize(deserializer)?))
    }
}

impl BitOr for DaysOfWeek {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for DaysOfWeek {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_and_contain() {
        let midweek = DaysOfWeek::MON | DaysOfWeek::WED;
        assert_eq!(midweek.bits(), 0b0000_1010);
        assert!(midweek.contains(DaysOfWeek::MON));
        assert!(!midweek.contains(DaysOfWeek::TUE));
        assert!(!midweek.is_empty());
        assert!(DaysOfWeek::NONE.is_empty());

        let mut acc = DaysOfWeek::SUN;
        acc |= DaysOfWeek::SAT;
        assert_eq!(acc, DaysOfWeek::WEEKEND);

        assert_eq!(DaysOfWeek::WEEKDAYS.bits(), 0b0011_1110);
        assert_eq!(DaysOfWeek::EVERY_DAY.bits(), 0b0111_1111);
    }

    #[test]
    fn from_device_ignores_high_bits() {
        assert_eq!(
            DaysOfWeek::from_bits_truncate(0b1000_1010),
            DaysOfWeek::MON | DaysOfWeek::WED
        );
    }

    #[test]
    fn deserialize_cannot_smuggle_high_bits() {
        let days: DaysOfWeek = serde_json::from_str("255").expect("deserialize");
        assert_eq!(days, DaysOfWeek::EVERY_DAY);
    }

    #[test]
    fn equal_values_hash_equally() {
        use std::collections::HashSet;

        // Equal sets built by different routes must collapse to one entry,
        // which is what the pyclass `hash` option relies on.
        let mut set = HashSet::new();
        set.insert(DaysOfWeek::MON | DaysOfWeek::WED);
        set.insert(DaysOfWeek::WED | DaysOfWeek::MON);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&DaysOfWeek::from_bits_truncate(0b0000_1010)));
    }

    #[test]
    fn debug_names_the_days() {
        assert_eq!(
            format!("{:?}", DaysOfWeek::MON | DaysOfWeek::WED),
            "DaysOfWeek(MON | WED)"
        );
        assert_eq!(format!("{:?}", DaysOfWeek::NONE), "DaysOfWeek(NONE)");
    }
}
