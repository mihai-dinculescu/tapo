use pyo3::prelude::*;
use tapo::Error;
use tapo::requests::AlarmDuration;

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(from_py_object, name = "AlarmDuration", eq, hash, frozen)]
pub enum PyAlarmDuration {
    Continuous,
    Once,
    Seconds,
}

impl PyAlarmDuration {
    /// Converts into the [`AlarmDuration`] that the `tapo` crate takes,
    /// using `seconds` for [`PyAlarmDuration::Seconds`].
    pub fn into_duration(self, seconds: Option<u32>) -> PyResult<AlarmDuration> {
        match self {
            PyAlarmDuration::Continuous => Ok(AlarmDuration::Continuous),
            PyAlarmDuration::Once => Ok(AlarmDuration::Once),
            PyAlarmDuration::Seconds => seconds.map(AlarmDuration::Seconds).ok_or_else(|| {
                Error::Validation {
                    field: "seconds".to_string(),
                    message:
                        "A value must be provided for seconds when duration = AlarmDuration.Seconds"
                            .to_string(),
                }
                .into()
            }),
        }
    }
}
