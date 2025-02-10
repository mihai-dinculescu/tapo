use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "AlarmDuration", eq)]
pub enum PyAlarmDuration {
    Continuous,
    Once,
    Seconds,
}
