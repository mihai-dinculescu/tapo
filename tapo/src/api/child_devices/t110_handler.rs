use crate::api::HubHandler;
use crate::error::Error;
use crate::requests::{EmptyParams, GetTriggerLogsParams, TapoParams, TapoRequest};
use crate::responses::T110Result;
use crate::responses::{T110Log, TriggerLogsResult};

/// Handler for the [T110](https://www.tapo.com/en/search/?q=T110) devices.
pub struct T110Handler<'h> {
    hub_handler: &'h HubHandler,
    device_id: String,
}

impl<'h> T110Handler<'h> {
    pub(crate) fn new(hub_handler: &'h HubHandler, device_id: String) -> Self {
        Self {
            hub_handler,
            device_id,
        }
    }

    /// Returns *device info* as [`T110Result`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<T110Result, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.hub_handler
            .control_child(self.device_id.clone(), request)
            .await
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
        page_size: u64,
        start_id: u64,
    ) -> Result<TriggerLogsResult<T110Log>, Error> {
        let child_params = GetTriggerLogsParams::new(page_size, start_id);
        let child_request = TapoRequest::GetTriggerLogs(Box::new(TapoParams::new(child_params)));

        self.hub_handler
            .control_child(self.device_id.clone(), child_request)
            .await
    }
}
