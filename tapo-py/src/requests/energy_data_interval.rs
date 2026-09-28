use pyo3::prelude::*;

#[derive(Clone, PartialEq, Hash)]
#[pyclass(from_py_object, name = "EnergyDataInterval", eq, eq_int, hash, frozen)]
pub enum PyEnergyDataInterval {
    Hourly,
    Daily,
    Monthly,
}
