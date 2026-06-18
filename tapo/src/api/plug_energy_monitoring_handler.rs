use std::time::Duration;

use crate::error::Error;
use crate::requests::{EnergyDataInterval, PowerDataInterval, ScheduleRule};
use crate::responses::{
    CurrentPowerResult, DeviceInfoPlugEnergyMonitoringResult, DeviceUsageEnergyMonitoringResult,
    EnergyDataResult, EnergyUsageResult, PowerDataResult, Timer,
};

tapo_handler! {
    /// Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
    /// [P110M](https://www.tapo.com/en/search/?q=P110M) and
    /// [P115](https://www.tapo.com/en/search/?q=P115) devices.
    PlugEnergyMonitoringHandler(DeviceInfoPlugEnergyMonitoringResult),
    on_off,
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl PlugEnergyMonitoringHandler {
    /// Returns *current power* as [`CurrentPowerResult`].
    pub async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        self.client.read().await.get_current_power().await
    }

    /// Returns *energy usage* as [`EnergyUsageResult`].
    pub async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        self.client.read().await.get_energy_usage().await
    }

    /// Returns *energy data* as [`EnergyDataResult`].
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        self.client.read().await.get_energy_data(interval).await
    }

    /// Returns *power data* as [`PowerDataResult`].
    pub async fn get_power_data(
        &self,
        interval: PowerDataInterval,
    ) -> Result<PowerDataResult, Error> {
        self.client.read().await.get_power_data(interval).await
    }

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

    /// Adds a new schedule rule to the device.  Returns the same rule
    /// with its device-assigned `id` filled in.  Schedule rules fire
    /// on the device itself, so they keep working even if the phone /
    /// Wi-Fi router / Tapo cloud is offline.
    pub async fn add_schedule_rule(&self, rule: ScheduleRule) -> Result<ScheduleRule, Error> {
        self.client.read().await.add_schedule_rule(rule).await
    }

    /// Edits an existing schedule rule.  `rule.id` must be set.
    pub async fn edit_schedule_rule(&self, rule: ScheduleRule) -> Result<(), Error> {
        self.client.read().await.edit_schedule_rule(rule).await
    }

    /// Returns every schedule rule currently stored on the device.
    pub async fn get_schedule_rules(&self) -> Result<Vec<ScheduleRule>, Error> {
        self.client.read().await.get_schedule_rules().await
    }

    /// Removes the schedule rule with the given id.
    pub async fn remove_schedule_rule(&self, id: impl Into<String>) -> Result<(), Error> {
        self.client
            .read()
            .await
            .remove_schedule_rule(id.into())
            .await
    }

    /// Removes every schedule rule from the device.
    pub async fn remove_all_schedule_rules(&self) -> Result<(), Error> {
        self.client.read().await.remove_all_schedule_rules().await
    }
}
