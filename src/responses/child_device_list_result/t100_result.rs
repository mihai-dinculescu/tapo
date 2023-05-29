use serde::{Deserialize, Serialize};

use crate::api::HubHandler;
use crate::error::Error;
use crate::requests::{EmptyParams, GetTriggerLogsParams, TapoParams, TapoRequest};
use crate::responses::{
    decode_value, DecodableResultExt, Status, TapoResponseExt, TriggerLogsResult,
};

/// T100 motion sensor.
///
/// Specific properties: `detected`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct T100Result {
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    pub detected: bool,
    pub device_id: String,
    pub fw_ver: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub jamming_rssi: i16,
    pub jamming_signal_level: u8,
    #[serde(rename = "lastOnboardingTimestamp")]
    pub last_onboarding_timestamp: u64,
    pub mac: String,
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub region: String,
    /// The time in seconds between each report.
    pub report_interval: u32,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status_follow_edge: bool,
    pub status: Status,
    pub r#type: String,
}

impl TapoResponseExt for T100Result {}

impl DecodableResultExt for Box<T100Result> {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// T100 Log.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
#[allow(missing_docs)]
pub enum T100Log {
    Motion { id: u64, timestamp: u64 },
}

impl T100Result {
    /// Gets *device info* as [`T100Result`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self, handler: &HubHandler) -> Result<T100Result, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        handler.control_child(self.device_id.clone(), request).await
    }

    /// Returns a list of trigger logs.
    ///
    /// # Arguments
    ///
    /// * `page_size` - the maximum number of log items to return
    /// * `start_id` - the log item `id` from which to start returning results in reverse chronological order (newest first)
    ///
    /// Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.
    pub async fn get_trigger_logs(
        &self,
        handler: &HubHandler,
        page_size: u64,
        start_id: u64,
    ) -> Result<TriggerLogsResult<T100Log>, Error> {
        let child_params = GetTriggerLogsParams::new(page_size, start_id);
        let child_request = TapoRequest::GetTriggerLogs(Box::new(TapoParams::new(child_params)));

        handler
            .control_child(self.device_id.clone(), child_request)
            .await
    }
}
