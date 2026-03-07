use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tapo::responses::{T100Log, TriggerLogsResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object, get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsT100Result {
    start_id: u64,
    sum: u64,
    logs: Vec<T100Log>,
}

impl From<TriggerLogsResult<T100Log>> for TriggerLogsT100Result {
    fn from(result: TriggerLogsResult<T100Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

tapo::impl_to_dict!(TriggerLogsT100Result);
