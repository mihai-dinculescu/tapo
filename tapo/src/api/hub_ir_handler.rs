use crate::error::Error;
use crate::responses::{ChildDeviceHubIrResult, DeviceInfoHubResult};

use super::{HubDevice, IrRemoteHandler};

tapo_handler! {
    /// Handler for the [H110](https://www.tapo.com/en/search/?q=H110) devices.
    ///
    /// It supports everything that [`HubHandler`](crate::HubHandler) does, and also
    /// handles the IR remotes paired with the hub through [`HubIrHandler::ir_remote`].
    HubIrHandler(DeviceInfoHubResult),
    device_management,
}

hub_alarm_handlers!(HubIrHandler);

hub_child_handlers!(HubIrHandler, ChildDeviceHubIrResult, "h110");

/// IR remote handler builders.
impl HubIrHandler {
    /// Returns an [`IrRemoteHandler`] for the given [`HubDevice`].
    ///
    /// IR remotes must be configured in the Tapo app first.
    ///
    /// # Arguments
    ///
    /// * `identifier` - a hub device identifier
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
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.ir_remote(HubDevice::ByDeviceId(device_id)).await?;
    /// // Send one of the keys stored on the remote
    /// device.send_ir_cmd_by_id("POWER").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ir_remote(&self, identifier: HubDevice) -> Result<IrRemoteHandler, Error> {
        let device_id = self
            .get_child_device_list()
            .await?
            .into_iter()
            .find_map(|child| match child {
                ChildDeviceHubIrResult::IrRemote(c) => match &identifier {
                    HubDevice::ByDeviceId(id) if c.device_id == *id => Some(c.device_id),
                    HubDevice::ByNickname(nickname) if c.nickname == *nickname => Some(c.device_id),
                    _ => None,
                },
                _ => None,
            })
            .ok_or(Error::DeviceNotFound)?;
        Ok(IrRemoteHandler::new(self.client.clone(), device_id))
    }

    /// Returns an [`IrRemoteHandler`] for the given `device_id` without first
    /// listing the hub's children to verify the device exists or matches the
    /// requested model. The device id is trusted; if it is wrong or refers to
    /// a different model, subsequent operations on the returned handler will
    /// fail at request time. Use this when you already have a valid device id
    /// (e.g. from a prior [`HubIrHandler::get_child_device_list`] call) to avoid
    /// the extra validation round-trip performed by [`HubIrHandler::ir_remote`].
    pub fn ir_remote_unchecked(&self, device_id: String) -> IrRemoteHandler {
        IrRemoteHandler::new(self.client.clone(), device_id)
    }
}

#[cfg(feature = "python")]
impl HubIrHandler {
    /// Returns a [`HubHandler`](crate::HubHandler) that shares this handler's
    /// authenticated session. `tapo-py` uses it to build the `HubHandler` base
    /// class of the Python `HubIrHandler`.
    #[doc(hidden)]
    pub fn to_hub_handler(&self) -> crate::HubHandler {
        crate::HubHandler::new(self.client.clone())
    }
}
