#[allow(unused_imports)]
use pyo3::prelude::*;
use tapo::PowerStripPlugHandler;
use tapo::responses::PowerStripPlugResult;

py_child_handler! {
    PyPowerStripPlugHandler(PowerStripPlugHandler, PowerStripPlugResult),
    py_name = "PowerStripPlugHandler",
    on_off,
}
