use crate::error::{Error, TapoResponseError};
use crate::requests::{GetTriggerLogsParams, TapoParams, TapoRequest};
use crate::responses::{T100Log, T100Result, TriggerLogsResult};

tapo_child_handler! {
    /// Handler for the [T100](https://www.tapo.com/en/search/?q=T100) devices.
    T100Handler(T100Result),
}

impl T100Handler {
    /// Returns a list of *trigger logs*.
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
    ) -> Result<TriggerLogsResult<T100Log>, Error> {
        let child_params = GetTriggerLogsParams::new(page_size, start_id);
        let child_request = TapoRequest::GetTriggerLogs(Box::new(TapoParams::new(child_params)));

        self.client
            .read()
            .await
            .control_child(self.device_id.clone(), child_request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }
}
