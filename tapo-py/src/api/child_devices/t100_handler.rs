use pyo3::prelude::*;
use tapo::T100Handler;
use tapo::responses::T100Result;

use crate::responses::TriggerLogsT100Result;

py_child_handler! {
    PyT100Handler(T100Handler, T100Result),
    py_name = "T100Handler",
    trigger_logs = TriggerLogsT100Result,
}
