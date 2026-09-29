use pyo3::prelude::*;
use tapo::S200Handler;
use tapo::responses::S200Result;

use crate::responses::TriggerLogsS200Result;

py_child_handler! {
    PyS200Handler(S200Handler, S200Result),
    py_name = "S200Handler",
    trigger_logs = TriggerLogsS200Result,
}
