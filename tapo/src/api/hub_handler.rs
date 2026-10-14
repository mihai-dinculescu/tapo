#[cfg(feature = "python")]
use std::sync::Arc;

#[cfg(feature = "python")]
use tokio::sync::RwLock;

#[cfg(feature = "python")]
use crate::api::ApiClient;
use crate::responses::DeviceInfoHubResult;

tapo_handler! {
    /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.
    HubHandler(DeviceInfoHubResult),
    device_management,
}

hub_alarm_handlers!(HubHandler);

hub_child_handlers!(HubHandler, ChildDeviceHubResult, "h100");

#[cfg(feature = "python")]
impl HubHandler {
    /// A handler that shares the authenticated session of another handler
    /// of the same hub, which reaches it at `ip` and `port`.
    pub(crate) fn with_shared_client(
        client: Arc<RwLock<ApiClient>>,
        ip: String,
        port: u16,
    ) -> Self {
        Self { client, ip, port }
    }
}

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
