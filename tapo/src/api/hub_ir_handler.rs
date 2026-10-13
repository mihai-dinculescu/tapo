use crate::responses::DeviceInfoHubResult;

#[cfg(feature = "python")]
use super::HubHandler;

tapo_handler! {
    /// Handler for the [H110](https://www.tapo.com/en/search/?q=H110) devices.
    HubIrHandler(DeviceInfoHubResult),
    device_management,
}

hub_alarm_handlers!(HubIrHandler);

hub_child_handlers!(HubIrHandler, ChildDeviceHubIrResult, "h110");

/// IR remote handler builders.
impl HubIrHandler {
    hub_child_handlers!(@checked ChildDeviceHubIrResult, ir_remote, IrRemoteHandler, IrRemote, "h110");
    hub_child_handlers!(@unchecked HubIrHandler, ir_remote_unchecked, ir_remote, IrRemoteHandler);
}

#[cfg(feature = "python")]
impl From<&HubIrHandler> for HubHandler {
    fn from(handler: &HubIrHandler) -> Self {
        HubHandler::new(handler.client.clone())
    }
}
