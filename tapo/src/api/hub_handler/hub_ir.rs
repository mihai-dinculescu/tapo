use crate::error::Error;
use crate::requests::{SendIrCmdByIdParams, TapoParams, TapoRequest};

use super::HubHandler;

impl HubHandler {
    /// Fire a stored IR key on an H110 child remote (`SMART.TAPOREMOTE`).
    ///
    /// The H110 hub is accessed via [`HubHandler`] (same as the H100). IR remotes
    /// appear in [`HubHandler::get_child_device_list_json`](HubHandler::get_child_device_list_json)
    /// (requires the `debug` feature); each key's `name` field is the value passed as `key_name`.
    pub async fn send_ir_cmd_by_id(
        &self,
        child_device_id: impl Into<String>,
        key_name: impl Into<String>,
    ) -> Result<(), Error> {
        let request = TapoRequest::SendIrCmdById(TapoParams::new(SendIrCmdByIdParams::new(
            key_name,
        )));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(child_device_id.into(), request)
            .await?;

        Ok(())
    }
}
