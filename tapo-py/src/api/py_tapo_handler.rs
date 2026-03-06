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
}
