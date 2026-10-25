/// Generates common PyO3 handler boilerplate for Tapo device handlers.
///
/// # Usage
///
/// ```ignore
/// py_handler! {
///     PyPlugHandler(PlugHandler, DeviceInfoPlugResult),
///     py_name = "PlugHandler",
///     on_off,
///     device_management,
///     device_usage = DeviceUsageResult,
/// }
/// ```
///
/// # Generated code
///
/// * Struct with `inner: Arc<RwLock<Handler>>` field and `#[derive(Clone)]`
/// * `#[pyo3::pyclass(from_py_object, name = ...)]` attribute
/// * `new(handler)` constructor
/// * `address` getter
/// * `refresh_session()` method
/// * `get_device_info()` method (typed)
/// * `get_device_info_json()` method (returns `Py<PyDict>`)
/// * `impl PyHandlerExt`
/// * `on()` and `off()` methods (if `on_off` specified)
/// * `device_reboot()` and `device_reset()` methods (if `device_management` specified)
/// * `get_device_usage()` method (if `device_usage = Type` specified)
macro_rules! py_handler {
    // on_off + device_management + device_usage
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
        on_off,
        device_management,
        device_usage = $device_usage:ty,
    ) => {
        py_handler!(@base $py_name($handler, $device_info), $pyname);
        py_handler!(@on_off $py_name, $handler);
        py_handler!(@device_management $py_name, $handler);
        py_handler!(@device_usage $py_name, $handler, $device_usage);
    };

    // on_off only
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
        on_off,
    ) => {
        py_handler!(@base $py_name($handler, $device_info), $pyname);
        py_handler!(@on_off $py_name, $handler);
    };

    // device_management only
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
        device_management,
    ) => {
        py_handler!(@base $py_name($handler, $device_info), $pyname);
        py_handler!(@device_management $py_name, $handler);
    };

    // No options
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
    ) => {
        py_handler!(@base $py_name($handler, $device_info), $pyname);
    };

    // Internal: base struct + core methods + PyHandlerExt
    (@base $py_name:ident($handler:ident, $device_info:ty), $pyname:literal) => {
        #[derive(Clone)]
        #[pyo3::pyclass(from_py_object, name = $pyname)]
        pub struct $py_name {
            inner: std::sync::Arc<tokio::sync::RwLock<$handler>>,
        }

        impl $py_name {
            pub fn new(handler: $handler) -> Self {
                Self {
                    inner: std::sync::Arc::new(tokio::sync::RwLock::new(handler)),
                }
            }
        }

        #[pyo3::pymethods]
        impl $py_name {
            #[getter]
            pub fn address(&self, py: pyo3::Python<'_>) -> (String, u16) {
                py.detach(|| self.inner.blocking_read().address())
            }

            pub async fn refresh_session(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::DerefMut;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.write().await.deref_mut(),
                    $handler::refresh_session,
                    discard_result
                )
            }

            pub async fn get_device_info(&self) -> pyo3::prelude::PyResult<$device_info> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_device_info
                )
            }

            pub async fn get_device_info_json(
                &self,
            ) -> pyo3::prelude::PyResult<pyo3::Py<pyo3::types::PyDict>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                let result = $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_device_info_json,
                )?;
                pyo3::prelude::Python::attach(|py| {
                    tapo::python::serde_object_to_py_dict(py, &result)
                })
            }

            pub async fn get_component_list(
                &self,
            ) -> pyo3::prelude::PyResult<Vec<tapo::responses::Component>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_component_list
                )
            }
        }

        impl crate::api::PyHandlerExt for $py_name {
            fn get_inner_handler(
                &self,
            ) -> std::sync::Arc<tokio::sync::RwLock<impl tapo::HandlerExt + 'static>> {
                std::sync::Arc::clone(&self.inner)
            }
        }
    };

    // Internal: on/off
    (@on_off $py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn on(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.read().await.deref(), $handler::on)
            }

            pub async fn off(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.read().await.deref(), $handler::off)
            }
        }
    };

    // Internal: device_management
    (@device_management $py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn device_reboot(&self, delay_s: u16) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::device_reboot,
                    delay_s
                )
            }

            pub async fn device_reset(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::device_reset
                )
            }
        }
    };

    // Internal: device_usage
    (@device_usage $py_name:ident, $handler:ident, $device_usage:ty) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn get_device_usage(&self) -> pyo3::prelude::PyResult<$device_usage> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_device_usage
                )
            }
        }
    };
}

/// Generates common PyO3 handler boilerplate for Tapo child device handlers.
///
/// # Usage
///
/// ```ignore
/// py_child_handler! {
///     PyT100Handler(T100Handler, T100Result),
///     py_name = "T100Handler",
/// }
/// ```
///
/// # Generated code
///
/// * Struct with `inner: Arc<Handler>` field and `#[derive(Clone)]`
/// * `#[pyo3::pyclass(from_py_object, name = ...)]` attribute
/// * `new(handler)` constructor
/// * `get_device_info()` method (typed)
/// * `get_device_info_json()` method (returns `Py<PyDict>`)
/// * `on()` and `off()` methods (if `on_off` specified)
/// * `get_trigger_logs()` method (if `trigger_logs = PyResultType` specified)
macro_rules! py_child_handler {
    // With on_off
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
        on_off,
    ) => {
        py_child_handler!(@base $py_name($handler, $device_info), $pyname);
        py_child_handler!(@on_off $py_name, $handler);
    };

    // With trigger_logs
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
        trigger_logs = $trigger_logs:ty,
    ) => {
        py_child_handler!(@base $py_name($handler, $device_info), $pyname);
        py_child_handler!(@trigger_logs $py_name, $handler, $trigger_logs);
    };

    // No options
    (
        $py_name:ident($handler:ident, $device_info:ty),
        py_name = $pyname:literal,
    ) => {
        py_child_handler!(@base $py_name($handler, $device_info), $pyname);
    };

    // Internal: base struct + core methods
    (@base $py_name:ident($handler:ident, $device_info:ty), $pyname:literal) => {
        #[derive(Clone)]
        #[pyo3::pyclass(from_py_object, name = $pyname)]
        pub struct $py_name {
            inner: std::sync::Arc<$handler>,
        }

        impl $py_name {
            pub fn new(handler: $handler) -> Self {
                Self {
                    inner: std::sync::Arc::new(handler),
                }
            }
        }

        #[pyo3::pymethods]
        impl $py_name {
            pub async fn get_device_info(&self) -> pyo3::prelude::PyResult<$device_info> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.deref(), $handler::get_device_info)
            }

            pub async fn get_device_info_json(
                &self,
            ) -> pyo3::prelude::PyResult<pyo3::Py<pyo3::types::PyDict>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                let result = $crate::call_handler_method!(
                    handler.deref(),
                    $handler::get_device_info_json
                )?;
                pyo3::prelude::Python::attach(|py| {
                    tapo::python::serde_object_to_py_dict(py, &result)
                })
            }

            pub async fn get_component_list(
                &self,
            ) -> pyo3::prelude::PyResult<Vec<tapo::responses::Component>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.deref(),
                    $handler::get_component_list
                )
            }
        }
    };

    // Internal: on/off
    (@on_off $py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn on(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.deref(), $handler::on)
            }

            pub async fn off(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.deref(), $handler::off)
            }
        }
    };

    // Internal: trigger logs for hub sensors
    (@trigger_logs $py_name:ident, $handler:ident, $trigger_logs:ty) => {
        #[pyo3::pymethods]
        impl $py_name {
            #[pyo3(signature = (page_size, start_id=0))]
            pub async fn get_trigger_logs(
                &self,
                page_size: u64,
                start_id: u64,
            ) -> pyo3::prelude::PyResult<$trigger_logs> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.deref(),
                    $handler::get_trigger_logs,
                    page_size,
                    start_id
                )
                .map(|result| result.into())
            }
        }
    };
}

/// Generates the hub alarm methods for a handler created with
/// [`py_handler!`], mirroring `hub_alarm_handlers!` in the `tapo` crate.
///
/// # Usage
///
/// ```ignore
/// py_hub_alarm_handlers!(PyHubHandler, HubHandler);
/// ```
///
/// # Generated code
///
/// * `get_supported_ringtone_list()` method
/// * `play_alarm(ringtone, volume, duration, seconds)` method
/// * `stop_alarm()` method
macro_rules! py_hub_alarm_handlers {
    ($py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn get_supported_ringtone_list(
                &self,
            ) -> pyo3::prelude::PyResult<Vec<String>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_supported_ringtone_list
                )
            }

            #[pyo3(signature = (ringtone, volume, duration, seconds=None))]
            pub async fn play_alarm(
                &self,
                ringtone: tapo::requests::AlarmRingtone,
                volume: tapo::requests::AlarmVolume,
                duration: $crate::requests::PyAlarmDuration,
                seconds: Option<u32>,
            ) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;

                let handler = self.inner.clone();
                let duration = duration.into_duration(seconds)?;

                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::play_alarm,
                    ringtone,
                    volume,
                    duration
                )
            }

            pub async fn stop_alarm(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.read().await.deref(), $handler::stop_alarm)
            }
        }
    };
}

/// Generates the device management methods for a handler created with
/// [`py_handler!`], mirroring `camera_device_management_handlers!` in the `tapo` crate.
///
/// # Usage
///
/// ```ignore
/// py_camera_device_management_handlers!(PyCameraHandler, CameraHandler);
/// ```
///
/// # Generated code
///
/// * `device_reboot()` method
/// * `device_reset()` method
macro_rules! py_camera_device_management_handlers {
    ($py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn device_reboot(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.read().await.deref(), $handler::device_reboot)
            }

            pub async fn device_reset(&self) -> pyo3::prelude::PyResult<()> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(handler.read().await.deref(), $handler::device_reset)
            }
        }
    };
}

/// Generates the camera detection methods for a handler created with
/// [`py_handler!`], mirroring `camera_detection_handlers!` in the `tapo` crate.
///
/// # Usage
///
/// ```ignore
/// py_camera_detection_handlers!(PyCameraHandler, CameraHandler);
/// ```
///
/// # Generated code
///
/// * `get_detections(start_time, end_time)` method
macro_rules! py_camera_detection_handlers {
    ($py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn get_detections(
                &self,
                start_time: $crate::utc_date_time::UtcDateTime,
                end_time: $crate::utc_date_time::UtcDateTime,
            ) -> pyo3::prelude::PyResult<Vec<$crate::responses::PyDetectionResult>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                let detections = $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_detections,
                    start_time.0,
                    end_time.0
                )?;
                Ok(detections.into_iter().map(Into::into).collect())
            }
        }
    };
}

/// Generates the camera stream methods for a handler created with
/// [`py_handler!`], mirroring `camera_stream_handlers!` in the `tapo` crate.
///
/// # Usage
///
/// ```ignore
/// py_camera_stream_handlers!(PyCameraHandler, CameraHandler);
/// ```
///
/// # Generated code
///
/// * `get_rtsp_stream_url(username, password)` method
/// * `get_snapshot(username, password)` method
macro_rules! py_camera_stream_handlers {
    ($py_name:ident, $handler:ident) => {
        #[pyo3::pymethods]
        impl $py_name {
            pub async fn get_rtsp_stream_url(
                &self,
                username: String,
                password: String,
            ) -> tapo::responses::RtspStreamUrl {
                use std::ops::Deref;
                let handler = self.inner.read().await;
                handler.deref().get_rtsp_stream_url(&username, &password)
            }

            pub async fn get_snapshot(
                &self,
                username: String,
                password: String,
            ) -> pyo3::prelude::PyResult<tapo::responses::Snapshot> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_snapshot,
                    &username,
                    &password
                )
            }
        }
    };
}

/// Generates the hub child device methods for a handler created with
/// [`py_handler!`], mirroring `hub_child_handlers!` in the `tapo` crate.
///
/// # Usage
///
/// ```ignore
/// py_hub_child_handlers!(PyHubHandler, HubHandler, ChildDeviceHubResult);
/// py_hub_child_handlers!(PyHubIrHandler, HubIrHandler, ChildDeviceHubIrResult, IrRemote);
/// ```
///
/// The third argument is the child device result enum returned by the handler's
/// `get_child_device_list`. It must have the `KE100`, `S200`, `S210`, `T100`,
/// `T110`, `T300`, `T31X` and `Other` variants; any further variants are listed
/// after it.
///
/// # Generated code
///
/// * `get_child_device_list()` method (returns a `Py<PyList>` of typed child results)
/// * `get_child_device_list_json(start_index)` method (returns `Py<PyDict>`)
/// * `get_child_device_component_list()` method
/// * `ke100()`, `s200()`, `s210()`, `t100()`, `t110()`, `t300()` and `t31x()`
///   child handler builders, taking a `device_id` or a `nickname`
/// * the `_unchecked` counterparts, taking a `device_id`
macro_rules! py_hub_child_handlers {
    ($py_name:ident, $handler:ident, $child:ident $(, $extra_variant:ident)* $(,)?) => {
        py_hub_child_handlers!(
            @impl $py_name, $handler,
            $child [$($extra_variant,)* KE100, S200, S210, T100, T110, T300, T31X, Other],
            (ke100, ke100_unchecked, PyKE100Handler),
            (s200, s200_unchecked, PyS200Handler),
            (s210, s210_unchecked, PyS210Handler),
            (t100, t100_unchecked, PyT100Handler),
            (t110, t110_unchecked, PyT110Handler),
            (t300, t300_unchecked, PyT300Handler),
            (t31x, t31x_unchecked, PyT31XHandler),
        );
    };

    (
        @impl $py_name:ident, $handler:ident,
        $child:ident [$($variant:ident),* $(,)?],
        $(($method:ident, $unchecked:ident, $py_handler:ident),)*
    ) => {
        impl $py_name {
            pub(crate) fn parse_identifier(
                device_id: Option<String>,
                nickname: Option<String>,
            ) -> pyo3::prelude::PyResult<tapo::HubDevice> {
                match (device_id, nickname) {
                    (Some(device_id), _) => Ok(tapo::HubDevice::ByDeviceId(device_id)),
                    (None, Some(nickname)) => Ok(tapo::HubDevice::ByNickname(nickname)),
                    _ => Err(tapo::Error::Validation {
                        field: "identifier".to_string(),
                        message: "Either a device_id or nickname must be provided".to_string(),
                    }
                    .into()),
                }
            }
        }

        #[pyo3::pymethods]
        impl $py_name {
            pub async fn get_child_device_list(
                &self,
            ) -> pyo3::prelude::PyResult<pyo3::Py<pyo3::types::PyList>> {
                use pyo3::prelude::*;
                use std::ops::Deref;
                use tapo::responses::$child;

                let handler = self.inner.clone();
                let children = $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_child_device_list
                )?;

                Python::attach(|py| {
                    let results = pyo3::types::PyList::empty(py);

                    for child in children {
                        match child {
                            $(
                                $child::$variant(device) => {
                                    results.append(device.into_pyobject(py)?)?;
                                }
                            )*
                            // The child result enum is non-exhaustive. Every type it has today is handled above.
                            _ => unreachable!(),
                        }
                    }

                    Ok(results.into())
                })
            }

            pub async fn get_child_device_list_json(
                &self,
                start_index: u64,
            ) -> pyo3::prelude::PyResult<pyo3::Py<pyo3::types::PyDict>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                let result = $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_child_device_list_json,
                    start_index
                )?;
                pyo3::prelude::Python::attach(|py| {
                    tapo::python::serde_object_to_py_dict(py, &result)
                })
            }

            pub async fn get_child_device_component_list(
                &self,
            ) -> pyo3::prelude::PyResult<Vec<tapo::responses::ChildDeviceComponentList>> {
                use std::ops::Deref;
                let handler = self.inner.clone();
                $crate::call_handler_method!(
                    handler.read().await.deref(),
                    $handler::get_child_device_component_list
                )
            }

            $(
                #[pyo3(signature = (device_id=None, nickname=None))]
                pub async fn $method(
                    &self,
                    device_id: Option<String>,
                    nickname: Option<String>,
                ) -> pyo3::prelude::PyResult<$crate::api::$py_handler> {
                    use std::ops::Deref;
                    let handler = self.inner.clone();
                    let identifier = $py_name::parse_identifier(device_id, nickname)?;

                    let child_handler = $crate::call_handler_method!(
                        handler.read().await.deref(),
                        $handler::$method,
                        identifier
                    )?;
                    Ok($crate::api::$py_handler::new(child_handler))
                }
            )*

            $(
                pub async fn $unchecked(
                    &self,
                    device_id: String,
                ) -> pyo3::prelude::PyResult<$crate::api::$py_handler> {
                    let child = self.inner.read().await.$unchecked(device_id);
                    Ok($crate::api::$py_handler::new(child))
                }
            )*
        }
    };
}
