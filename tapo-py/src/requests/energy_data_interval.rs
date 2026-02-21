use pyo3::prelude::*;

#[derive(Clone, PartialEq)]
#[pyclass(from_py_object, name = "EnergyDataInterval", eq, eq_int)]
pub enum PyEnergyDataInterval {
    Hourly,
    Daily,
    Monthly,
}
