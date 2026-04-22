use crate::error::Error;
use crate::requests::{SmartCamDoParams, SmartCamGetParams};
use crate::responses::{DeviceInfoCameraResult, Preset, PresetRaw, RtspStreamUrl};

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
    ip_address,
}

impl CameraPtzHandler {
    /// Returns the RTSP stream URLs for the camera.
    ///
    /// The credentials are the **camera account** credentials set in the Tapo app
    /// (Camera Settings > Advanced Settings > Camera Account), not the TP-Link cloud account credentials.
    /// They will be URL-encoded automatically.
    pub fn get_rtsp_stream_url(&self, username: &str, password: &str) -> RtspStreamUrl {
        RtspStreamUrl {
            hd: self.rtsp_url("stream1", username, password),
            sd: self.rtsp_url("stream2", username, password),
            mjpeg: self.rtsp_url("stream8", username, password),
        }
    }

    fn rtsp_url(&self, stream: &str, username: &str, password: &str) -> String {
        let mut url = reqwest::Url::parse(&format!("rtsp://{}:554/{stream}", self.ip_address))
            .expect("valid RTSP base URL");
        url.set_username(username).expect("valid username");
        url.set_password(Some(password)).expect("valid password");
        url.to_string()
    }

    /// Moves the camera by the given pan and tilt values.
    ///
    /// Positive `pan` moves right, negative moves left. `0` will not move on this axis.
    /// Positive `tilt` moves up, negative moves down. `0` will not move on this axis.
    ///
    /// If unsure of the value, `10` for both `pan` and `tilt` are good values for small nudges.
    pub async fn pan_tilt(&self, pan: i32, tilt: i32) -> Result<(), Error> {
        self.client
            .read()
            .await
            .execute_smart_cam_do(SmartCamDoParams::motor_move(pan, tilt))
            .await
    }

    /// Saves the current camera position as a named preset.
    pub async fn save_preset(&self, name: &str) -> Result<(), Error> {
        self.client
            .read()
            .await
            .execute_smart_cam_do(SmartCamDoParams::set_preset(name))
            .await
    }

    /// Moves the camera to a saved preset position by its ID.
    pub async fn goto_preset(&self, id: &str) -> Result<(), Error> {
        self.client
            .read()
            .await
            .execute_smart_cam_do(SmartCamDoParams::goto_preset(id))
            .await
    }

    /// Deletes a preset by its ID.
    pub async fn delete_preset(&self, id: &str) -> Result<(), Error> {
        self.client
            .read()
            .await
            .execute_smart_cam_do(SmartCamDoParams::remove_preset(id))
            .await
    }

    /// Returns the list of saved PTZ presets.
    pub async fn get_presets(&self) -> Result<Vec<Preset>, Error> {
        let raw: PresetRaw = self
            .client
            .read()
            .await
            .execute_smart_cam_get(SmartCamGetParams::preset())
            .await?;

        Ok(raw.into_presets())
    }
}
