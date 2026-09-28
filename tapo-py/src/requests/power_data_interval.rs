use pyo3::prelude::*;

#[derive(Clone, PartialEq, Hash)]
#[pyclass(from_py_object, name = "PowerDataInterval", eq, eq_int, hash, frozen)]
pub enum PyPowerDataInterval {
    Every5Minutes,
    Hourly,
}
