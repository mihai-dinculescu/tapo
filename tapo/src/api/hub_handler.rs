use crate::responses::DeviceInfoHubResult;

tapo_handler! {
    /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, HubDevice};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    /// // Get a handler for a child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = hub.t31x(HubDevice::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    HubHandler(DeviceInfoHubResult),
    device_management,
}

hub_alarm_handlers!(HubHandler);

hub_child_handlers!(HubHandler, ChildDeviceHubResult);

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
