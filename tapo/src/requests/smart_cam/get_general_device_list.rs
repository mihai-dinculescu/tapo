use serde::Serialize;

use crate::requests::EmptyObjectParams;

/// `module → section → data` envelope for `getGeneralDeviceList`:
/// `{"general_camera_manage": {"paired_general_device_list": {}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetGeneralDeviceListParams {
    general_camera_manage: GeneralCameraManageParams,
}

#[derive(Debug, Serialize)]
struct GeneralCameraManageParams {
    paired_general_device_list: EmptyObjectParams,
}

impl SmartCamGetGeneralDeviceListParams {
    pub fn new() -> Self {
        Self {
            general_camera_manage: GeneralCameraManageParams {
                paired_general_device_list: EmptyObjectParams {},
            },
        }
    }
}
