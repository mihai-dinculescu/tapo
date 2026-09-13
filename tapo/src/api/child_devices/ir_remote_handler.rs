use crate::error::Error;
use crate::requests::{SendIrCmdByIdParams, TapoParams, TapoRequest};
use crate::responses::IrRemoteResult;

tapo_child_handler! {
    /// Handler for the IR remotes paired with a
    /// [H110](https://www.tapo.com/en/search/?q=H110) hub.
    ///
    /// IR remotes are virtual child devices that are created by the Tapo app, either by
    /// picking an appliance from TP-Link's IR database or by learning the keys from a
    /// physical remote.
    IrRemoteHandler(IrRemoteResult),
    camel_case_device_info,
}

impl IrRemoteHandler {
    /// Sends one of the IR keys stored on this remote.
    ///
    /// # Arguments
    ///
    /// * `key_name` - the [`name`](crate::responses::IrRemoteKey::name) of a key
    ///   from this remote's [`key_list`](crate::responses::IrRemoteResult::key_list)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h110("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the IR remote
    /// let remote = hub
    ///     .ir_remote(HubDevice::ByNickname("Living Room TV".to_string()))
    ///     .await?;
    /// // Send one of the keys stored on the remote
    /// remote.send_ir_cmd_by_id("POWER").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_ir_cmd_by_id(&self, key_name: impl Into<String>) -> Result<(), Error> {
        let request =
            TapoRequest::SendIrCmdById(TapoParams::new(SendIrCmdByIdParams::new(key_name)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }
}
