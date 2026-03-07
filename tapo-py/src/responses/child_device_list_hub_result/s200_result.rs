use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tapo::responses::{S200Log, TriggerLogsResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object, get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsS200Result {
    start_id: u64,
    sum: u64,
    logs: Vec<S200Log>,
}

impl From<TriggerLogsResult<S200Log>> for TriggerLogsS200Result {
    fn from(result: TriggerLogsResult<S200Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

tapo::impl_to_dict!(TriggerLogsS200Result);
