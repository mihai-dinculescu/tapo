use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(from_py_object, name = "AlarmDuration", eq)]
pub enum PyAlarmDuration {
    Continuous,
    Once,
    Seconds,
}
