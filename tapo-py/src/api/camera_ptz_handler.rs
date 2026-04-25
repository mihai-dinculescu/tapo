use std::ops::Deref;

use pyo3::prelude::*;
use tapo::CameraPtzHandler;
use tapo::responses::{DeviceInfoCameraResult, Preset, RtspStreamUrl, Snapshot};

use crate::call_handler_method;

py_handler! {
    PyCameraPtzHandler(CameraPtzHandler, DeviceInfoCameraResult),
    py_name = "CameraPtzHandler",
}

#[pymethods]
impl PyCameraPtzHandler {
    pub async fn get_rtsp_stream_url(&self, username: String, password: String) -> RtspStreamUrl {
        let handler = self.inner.clone();
        let handler = handler.read().await;
        handler.deref().get_rtsp_stream_url(&username, &password)
    }

    pub async fn get_snapshot(&self, username: String, password: String) -> PyResult<Snapshot> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            CameraPtzHandler::get_snapshot,
            &username,
            &password
        )
    }

    pub async fn pan_tilt(&self, pan: i32, tilt: i32) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            CameraPtzHandler::pan_tilt,
            pan,
            tilt
        )
    }

    pub async fn save_preset(&self, name: String) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            CameraPtzHandler::save_preset,
            &name
        )
    }

    pub async fn goto_preset(&self, id: String) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            CameraPtzHandler::goto_preset,
            &id
        )
    }

    pub async fn delete_preset(&self, id: String) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            CameraPtzHandler::delete_preset,
            &id
        )
    }

    pub async fn get_presets(&self) -> PyResult<Vec<Preset>> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), CameraPtzHandler::get_presets)
    }
}
