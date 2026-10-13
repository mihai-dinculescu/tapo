use crate::responses::DeviceInfoHubResult;

tapo_handler! {
    /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.
    HubHandler(DeviceInfoHubResult),
    device_management,
}

hub_alarm_handlers!(HubHandler);

hub_child_handlers!(HubHandler, ChildDeviceHubResult, "h100");

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
