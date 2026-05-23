use std::time::Duration;

use crate::error::Error;
use crate::responses::{DeviceInfoPlugResult, DeviceUsageResult, Timer};

tapo_handler! {
    /// Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
    /// [P105](https://www.tapo.com/en/search/?q=P105) devices.
    PlugHandler(DeviceInfoPlugResult),
    on_off,
    device_usage = DeviceUsageResult,
    device_management,
}

impl PlugHandler {
    /// Arms the plug's countdown timer (the "Timer" feature in the
    /// Tapo app), replacing any timer that is currently armed.
    /// After `delay`, the plug transitions to `turn_on`.
    pub async fn set_timer(&self, delay: Duration, turn_on: bool) -> Result<Timer, Error> {
        self.client.read().await.set_timer(delay, turn_on).await
    }

    /// Returns the armed timer, or `None` if no timer is armed.
    pub async fn get_timer(&self) -> Result<Option<Timer>, Error> {
        self.client.read().await.get_timer().await
    }

    /// Cancels the armed timer (the "Stop" button in the Tapo app),
    /// or returns successfully if no timer was armed.
    pub async fn clear_timer(&self) -> Result<(), Error> {
        self.client.read().await.clear_timer().await
    }
}
