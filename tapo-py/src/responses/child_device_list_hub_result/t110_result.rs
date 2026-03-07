use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tapo::responses::{T110Log, TriggerLogsResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object, get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsT110Result {
    start_id: u64,
    sum: u64,
    logs: Vec<T110Log>,
}

impl From<TriggerLogsResult<T110Log>> for TriggerLogsT110Result {
    fn from(result: TriggerLogsResult<T110Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

tapo::impl_to_dict!(TriggerLogsT110Result);
