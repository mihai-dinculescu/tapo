use pyo3::prelude::*;

#[derive(Clone, PartialEq, Hash)]
#[pyclass(from_py_object, name = "PowerDataInterval", eq, hash, frozen)]
pub enum PyPowerDataInterval {
    Every5Minutes,
    Hourly,
}
