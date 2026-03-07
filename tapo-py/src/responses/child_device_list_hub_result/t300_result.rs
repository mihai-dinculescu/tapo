use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tapo::responses::{T300Log, TriggerLogsResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object, get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsT300Result {
    start_id: u64,
    sum: u64,
    logs: Vec<T300Log>,
}

impl From<TriggerLogsResult<T300Log>> for TriggerLogsT300Result {
    fn from(result: TriggerLogsResult<T300Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

tapo::impl_to_dict!(TriggerLogsT300Result);
