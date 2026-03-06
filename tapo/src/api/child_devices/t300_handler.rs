use crate::error::{Error, TapoResponseError};
use crate::requests::{GetTriggerLogsParams, TapoParams, TapoRequest};
use crate::responses::{T300Log, T300Result, TriggerLogsResult};

tapo_child_handler! {
    /// Handler for the [T300](https://www.tapo.com/en/search/?q=T300) devices.
    T300Handler(T300Result),
}

impl T300Handler {
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
    ) -> Result<TriggerLogsResult<T300Log>, Error> {
        let child_params = GetTriggerLogsParams::new(page_size, start_id);
        let child_request = TapoRequest::GetTriggerLogs(Box::new(TapoParams::new(child_params)));

        self.client
            .read()
            .await
            .control_child::<TriggerLogsResult<T300Log>>(self.device_id.clone(), child_request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }
}
