use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::{
    S200Event, S200RotationParams, T100Event, T110Event, T300Event, TriggerLog, TriggerLogsResult,
};

// PyO3 cannot expose a Rust enum that mixes unit variants with a data-carrying
// one, so this plain enum mirrors `S200Event`; the rotation's `params` are
// exposed on `PyS200Log` instead.
/// S200B and S200D trigger log event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass(name = "S200Event", from_py_object, eq, hash, frozen)]
#[allow(missing_docs)]
pub enum PyS200Event {
    Rotation,
    SingleClick,
    DoubleClick,
    LowBattery,
}

impl From<&S200Event> for PyS200Event {
    fn from(value: &S200Event) -> Self {
        match value {
            S200Event::Rotation { .. } => Self::Rotation,
            S200Event::SingleClick => Self::SingleClick,
            S200Event::DoubleClick => Self::DoubleClick,
            S200Event::LowBattery => Self::LowBattery,
        }
    }
}

/// Generates a Python class wrapping [`TriggerLog`] for one event type, since
/// pyo3 classes cannot be generic. `to_dict` serializes the wrapped value.
macro_rules! py_trigger_log {
    ($name:ident, $py_name:literal, $event:ty, $py_event:ty, |$e:ident| $to_py_event:expr) => {
        #[derive(Debug, Clone, Serialize)]
        #[pyclass(name = $py_name, from_py_object)]
        #[serde(transparent)]
        #[allow(missing_docs)]
        pub struct $name(TriggerLog<$event>);

        impl From<TriggerLog<$event>> for $name {
            fn from(log: TriggerLog<$event>) -> Self {
                Self(log)
            }
        }

        #[pymethods]
        impl $name {
            /// Hub-assigned id of the log item. The hub numbers the logs of all its sensors from one counter, so newer items have larger ids.
            #[getter]
            fn id(&self) -> u64 {
                self.0.id
            }

            /// When the event happened, in UTC.
            #[getter]
            fn triggered_at(&self) -> DateTime<Utc> {
                self.0.triggered_at
            }

            /// What happened.
            #[getter]
            fn event(&self) -> $py_event {
                let $e = &self.0.event;
                $to_py_event
            }
        }

        tapo::impl_to_dict!($name);
    };
}

py_trigger_log!(PyS200Log, "S200Log", S200Event, PyS200Event, |event| {
    PyS200Event::from(event)
});
py_trigger_log!(PyT100Log, "T100Log", T100Event, T100Event, |event| {
    event.clone()
});
py_trigger_log!(PyT110Log, "T110Log", T110Event, T110Event, |event| {
    event.clone()
});
py_trigger_log!(PyT300Log, "T300Log", T300Event, T300Event, |event| {
    event.clone()
});

#[pymethods]
impl PyS200Log {
    /// The rotation details when `event` is `S200Event.Rotation`, `None` otherwise.
    #[getter]
    fn params(&self) -> Option<S200RotationParams> {
        match &self.0.event {
            S200Event::Rotation { params } => Some(params.clone()),
            _ => None,
        }
    }
}

/// Generates a Python class mirroring [`TriggerLogsResult`] for one event type,
/// since pyo3 classes cannot be generic.
macro_rules! py_trigger_logs_result {
    ($name:ident, $event:ty, $log:ty) => {
        #[derive(Debug, Clone, Serialize)]
        #[pyclass(from_py_object, get_all)]
        #[allow(missing_docs)]
        pub struct $name {
            start_id: u64,
            sum: u64,
            logs: Vec<$log>,
        }

        impl From<TriggerLogsResult<$event>> for $name {
            fn from(result: TriggerLogsResult<$event>) -> Self {
                Self {
                    start_id: result.start_id,
                    sum: result.sum,
                    logs: result.logs.into_iter().map(Into::into).collect(),
                }
            }
        }

        tapo::impl_to_dict!($name);
    };
}

py_trigger_logs_result!(TriggerLogsS200Result, S200Event, PyS200Log);
py_trigger_logs_result!(TriggerLogsT100Result, T100Event, PyT100Log);
py_trigger_logs_result!(TriggerLogsT110Result, T110Event, PyT110Log);
py_trigger_logs_result!(TriggerLogsT300Result, T300Event, PyT300Log);
