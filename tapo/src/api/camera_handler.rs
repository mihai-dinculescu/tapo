use crate::responses::DeviceInfoCameraResult;

tapo_handler! {
    /// Handler for Tapo cameras without PTZ, such as the
    /// [C120](https://www.tapo.com/en/search/?q=C120).
    CameraHandler(DeviceInfoCameraResult),
}

camera_stream_handlers!(CameraHandler);
