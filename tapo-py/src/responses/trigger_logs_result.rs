use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tapo::responses::{S200Log, T100Log, T110Log, T300Log, TriggerLogsResult};

/// Generates a Python class mirroring [`TriggerLogsResult`] for one log type,
/// since pyo3 classes cannot be generic.
macro_rules! py_trigger_logs_result {
    ($name:ident, $log:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[pyclass(from_py_object, get_all)]
        #[allow(missing_docs)]
        pub struct $name {
            start_id: u64,
            sum: u64,
            logs: Vec<$log>,
        }

        impl From<TriggerLogsResult<$log>> for $name {
            fn from(result: TriggerLogsResult<$log>) -> Self {
                Self {
                    start_id: result.start_id,
                    sum: result.sum,
                    logs: result.logs,
                }
            }
        }

        tapo::impl_to_dict!($name);
    };
}

py_trigger_logs_result!(TriggerLogsS200Result, S200Log);
py_trigger_logs_result!(TriggerLogsT100Result, T100Log);
py_trigger_logs_result!(TriggerLogsT110Result, T110Log);
py_trigger_logs_result!(TriggerLogsT300Result, T300Log);
