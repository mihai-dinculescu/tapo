use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(from_py_object, name = "AlarmDuration", eq, hash, frozen)]
pub enum PyAlarmDuration {
    Continuous,
    Once,
    Seconds,
}
