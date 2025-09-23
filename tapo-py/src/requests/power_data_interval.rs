use pyo3::prelude::*;

#[derive(Clone, PartialEq)]
#[pyclass(name = "PowerDataInterval", eq, eq_int)]
pub enum PyPowerDataInterval {
    Every5Minutes,
    Hourly,
}
