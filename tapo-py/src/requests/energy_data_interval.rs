use pyo3::prelude::*;

#[derive(Clone, PartialEq)]
#[pyclass(name = "EnergyDataInterval", eq, eq_int)]
pub enum PyEnergyDataInterval {
    Hourly,
    Daily,
    Monthly,
}
