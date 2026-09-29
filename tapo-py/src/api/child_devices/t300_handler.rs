use pyo3::prelude::*;
use tapo::T300Handler;
use tapo::responses::T300Result;

use crate::responses::TriggerLogsT300Result;

py_child_handler! {
    PyT300Handler(T300Handler, T300Result),
    py_name = "T300Handler",
    trigger_logs = TriggerLogsT300Result,
}
