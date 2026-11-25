use crate::responses::DeviceInfoCameraResult;

tapo_handler! {
    /// Handler for Tapo cameras without PTZ, such as the
    /// [C100](https://www.tapo.com/en/search/?q=C100),
    /// [C101](https://www.tapo.com/en/search/?q=C101),
    /// [C110](https://www.tapo.com/en/search/?q=C110),
    /// [C120](https://www.tapo.com/en/search/?q=C120),
    /// [C460](https://www.tapo.com/en/search/?q=C460),
    /// [C720](https://www.tapo.com/en/search/?q=C720),
    /// and [TC65](https://www.tapo.com/en/search/?q=TC65).
    CameraHandler(DeviceInfoCameraResult),
}

camera_device_management_handlers!(CameraHandler);
camera_detection_handlers!(CameraHandler, builder = c120);
camera_stream_handlers!(CameraHandler);
