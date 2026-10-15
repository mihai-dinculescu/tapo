use pyo3::prelude::*;
use tapo::CameraHandler;
use tapo::responses::DeviceInfoCameraResult;

py_handler! {
    PyCameraHandler(CameraHandler, DeviceInfoCameraResult),
    py_name = "CameraHandler",
}

py_camera_stream_handlers!(PyCameraHandler, CameraHandler);
