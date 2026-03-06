use crate::responses::{DeviceInfoPlugResult, DeviceUsageResult};

tapo_handler! {
    /// Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
    /// [P105](https://www.tapo.com/en/search/?q=P105) devices.
    PlugHandler(DeviceInfoPlugResult),
    on_off,
    device_usage = DeviceUsageResult,
    device_management,
}
