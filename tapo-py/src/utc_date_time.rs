use chrono::{DateTime, Utc};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyTzInfo};

/// A timezone-aware Python `datetime` in any timezone, converted to UTC.
///
/// PyO3's own `DateTime<Utc>` extraction only accepts `tzinfo=timezone.utc`.
/// A naive `datetime` (no `tzinfo`, or one whose `utcoffset()` is `None`) raises `TypeError`,
/// rather than being read as local time.
pub struct UtcDateTime(pub DateTime<Utc>);

impl<'py> FromPyObject<'_, 'py> for UtcDateTime {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dt = ob.cast::<PyDateTime>()?;

        if dt.call_method0("utcoffset")?.is_none() {
            return Err(PyTypeError::new_err(
                "expected a timezone-aware datetime, got a naive one",
            ));
        }

        let utc = PyTzInfo::utc(ob.py())?;
        dt.call_method1("astimezone", (utc,))?.extract().map(Self)
    }
}
