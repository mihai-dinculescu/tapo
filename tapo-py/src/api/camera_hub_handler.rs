use std::ops::Deref;
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::{
    DeviceInfoCameraHubResult, RecordingDateHubResult, RecordingDownloadResult, TimezoneHubResult,
};
use tapo::{CameraHubHandler, Error};
use tokio::io::AsyncWriteExt;

use crate::call_handler_method;
use crate::responses::{PyGeneralDeviceHubResult, PyRecordingHubResult};
use crate::utc_date_time::UtcDateTime;

py_handler! {
    PyCameraHubHandler(CameraHubHandler, DeviceInfoCameraHubResult),
    py_name = "CameraHubHandler",
}

py_hub_child_handlers!(PyCameraHubHandler, CameraHubHandler);

#[pymethods]
impl PyCameraHubHandler {
    pub async fn get_general_device_list(&self) -> PyResult<Vec<PyGeneralDeviceHubResult>> {
        let handler = self.inner.clone();
        let devices = call_handler_method!(
            handler.read().await.deref(),
            CameraHubHandler::get_general_device_list
        )?;
        Ok(devices.into_iter().map(Into::into).collect())
    }

    pub async fn get_general_device_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            CameraHubHandler::get_general_device_list_json
        )?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_timezone(&self) -> PyResult<TimezoneHubResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), CameraHubHandler::get_timezone)
    }

    pub async fn get_recording_dates(
        &self,
        child_device_id: String,
        start_time: UtcDateTime,
        end_time: UtcDateTime,
    ) -> PyResult<Vec<RecordingDateHubResult>> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            CameraHubHandler::get_recording_dates,
            child_device_id,
            start_time.0,
            end_time.0
        )
    }

    pub async fn get_recordings(
        &self,
        child_device_id: String,
        start_time: UtcDateTime,
        end_time: UtcDateTime,
    ) -> PyResult<Vec<PyRecordingHubResult>> {
        let handler = self.inner.clone();
        let recordings = call_handler_method!(
            handler.read().await.deref(),
            CameraHubHandler::get_recordings,
            child_device_id,
            start_time.0,
            end_time.0
        )?;
        Ok(recordings.into_iter().map(Into::into).collect())
    }

    pub async fn download_recording(
        &self,
        child_device_id: String,
        start_time: UtcDateTime,
        end_time: UtcDateTime,
        path: PathBuf,
    ) -> PyResult<RecordingDownloadResult> {
        let handler = self.inner.clone();

        let result = crate::runtime::tokio()
            .spawn(async move {
                let mut file = tokio::fs::File::create(&path)
                    .await
                    .map_err(|err| Error::Other(anyhow::Error::from(err)))?;

                let download = handler
                    .read()
                    .await
                    .download_recording(child_device_id, start_time.0, end_time.0, &mut file)
                    .await;
                let flush = file.flush().await;

                let result = download?;
                flush.map_err(|err| Error::Other(anyhow::Error::from(err)))?;

                Ok::<_, Error>(result)
            })
            .await
            .map_err(anyhow::Error::from)??;

        Ok(result)
    }
}
