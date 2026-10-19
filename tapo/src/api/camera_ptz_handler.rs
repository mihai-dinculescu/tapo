use crate::error::Error;
use crate::requests::{
    SmartCamAddMotorPositionParams, SmartCamDeletePresetParams, SmartCamGetPresetConfigParams,
    SmartCamMotorMoveParams, SmartCamMotorMoveToPresetParams, TapoParams, TapoRequest,
};
use crate::responses::{DeviceInfoCameraResult, Preset, PresetRaw};

tapo_handler! {
    /// Handler for Tapo cameras with PTZ, such as the
    /// [C210](https://www.tapo.com/en/search/?q=C210),
    /// [C220](https://www.tapo.com/en/search/?q=C220),
    /// [C225](https://www.tapo.com/en/search/?q=C225),
    /// [C325WB](https://www.tapo.com/en/search/?q=C325WB),
    /// [C520WS](https://www.tapo.com/en/search/?q=C520WS),
    /// [TC40](https://www.tapo.com/en/search/?q=TC40),
    /// and [TC70](https://www.tapo.com/en/search/?q=TC70).
    CameraPtzHandler(DeviceInfoCameraResult),
}

camera_device_management_handlers!(CameraPtzHandler);
camera_stream_handlers!(CameraPtzHandler);

impl CameraPtzHandler {
    /// Moves the camera by the given pan and tilt values.
    ///
    /// Positive `pan` moves right, negative moves left. `0` will not move on this axis.
    /// Positive `tilt` moves up, negative moves down. `0` will not move on this axis.
    ///
    /// If unsure of the value, `10` for both `pan` and `tilt` are good values for small nudges.
    pub async fn pan_tilt(&self, pan: i32, tilt: i32) -> Result<(), Error> {
        let request = TapoRequest::SmartCamMotorMove(TapoParams::new(
            SmartCamMotorMoveParams::new(pan, tilt),
        ));

        self.client
            .read()
            .await
            .execute_smart_cam_command(request)
            .await
    }

    /// Saves the current camera position as a named preset.
    pub async fn save_preset(&self, name: &str) -> Result<(), Error> {
        let request = TapoRequest::SmartCamAddMotorPosition(TapoParams::new(
            SmartCamAddMotorPositionParams::new(name),
        ));

        self.client
            .read()
            .await
            .execute_smart_cam_command(request)
            .await
    }

    /// Moves the camera to a saved preset position by its ID.
    pub async fn goto_preset(&self, id: &str) -> Result<(), Error> {
        let request = TapoRequest::SmartCamMotorMoveToPreset(TapoParams::new(
            SmartCamMotorMoveToPresetParams::new(id),
        ));

        self.client
            .read()
            .await
            .execute_smart_cam_command(request)
            .await
    }

    /// Deletes a preset by its ID.
    pub async fn delete_preset(&self, id: &str) -> Result<(), Error> {
        let request =
            TapoRequest::SmartCamDeletePreset(TapoParams::new(SmartCamDeletePresetParams::new(id)));

        self.client
            .read()
            .await
            .execute_smart_cam_command(request)
            .await
    }

    /// Returns the list of saved PTZ presets.
    pub async fn get_presets(&self) -> Result<Vec<Preset>, Error> {
        let request = TapoRequest::SmartCamGetPresetConfig(TapoParams::new(
            SmartCamGetPresetConfigParams::new(),
        ));

        let raw: PresetRaw = self
            .client
            .read()
            .await
            .execute_smart_cam_section_request(request)
            .await?;

        Ok(raw.into_presets())
    }
}
