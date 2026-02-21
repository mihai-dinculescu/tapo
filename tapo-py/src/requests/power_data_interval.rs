use pyo3::prelude::*;

#[derive(Clone, PartialEq)]
#[pyclass(from_py_object, name = "PowerDataInterval", eq, eq_int)]
pub enum PyPowerDataInterval {
    Every5Minutes,
    Hourly,
}
