use crate::responses::DeviceInfoHubResult;

tapo_handler! {
    /// Handler for the [H110](https://www.tapo.com/en/search/?q=H110) devices.
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
    /// // Get a handler for a child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t31x(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    HubIrHandler(DeviceInfoHubResult),
    device_management,
}

hub_alarm_handlers!(HubIrHandler);

hub_child_handlers!(HubIrHandler, ChildDeviceHubIrResult);

/// IR remote handler builders.
impl HubIrHandler {
    hub_child_handlers!(@checked ChildDeviceHubIrResult, ir_remote, IrRemoteHandler, IrRemote);
    hub_child_handlers!(@unchecked HubIrHandler, ir_remote_unchecked, ir_remote, IrRemoteHandler);
}
