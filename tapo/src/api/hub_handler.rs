use crate::error::Error;
use crate::requests::{AlarmDuration, AlarmRingtone, AlarmVolume, PlayAlarmParams};
use crate::responses::{ChildDeviceHubResult, DeviceInfoHubResult};

use super::IrRemoteHandler;

tapo_handler! {
    /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) and
    /// [H110](https://www.tapo.com/en/search/?q=H110) devices.
    HubHandler(DeviceInfoHubResult),
    device_management,
}

/// Hub handler methods.
impl HubHandler {
    /// Returns a list of ringtones (alarm types) supported by the hub.
    /// This information is useful in debugging or when investigating new functionality to add.
    #[cfg(feature = "debug")]
    pub async fn get_supported_ringtone_list(&self) -> Result<Vec<String>, Error> {
        self.client
            .read()
            .await
            .get_supported_alarm_type_list()
            .await
            .map(|response| response.alarm_type_list)
    }

    /// Start playing the hub alarm.
    pub async fn play_alarm(
        &self,
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: AlarmDuration,
    ) -> Result<(), Error> {
        self.client
            .read()
            .await
            .play_alarm(PlayAlarmParams::new(ringtone, volume, duration)?)
            .await
    }

    /// Stop playing the hub alarm, if it's currently playing.
    pub async fn stop_alarm(&self) -> Result<(), Error> {
        self.client.read().await.stop_alarm().await
    }
}

hub_child_handlers!(HubHandler, "h100");

/// IR remote handler builders.
impl HubHandler {
    /// Returns an [`IrRemoteHandler`] for the given [`HubDevice`].
    ///
    /// IR remotes are only available on the [H110](https://www.tapo.com/en/search/?q=H110) hub,
    /// and they must be configured in the Tapo app first.
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
                ChildDeviceHubResult::IrRemote(c) => match &identifier {
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
    /// (e.g. from a prior [`HubHandler::get_child_device_list`] call) to avoid
    /// the extra validation round-trip performed by [`HubHandler::ir_remote`].
    pub fn ir_remote_unchecked(&self, device_id: String) -> IrRemoteHandler {
        IrRemoteHandler::new(self.client.clone(), device_id)
    }
}

/// Hub Device.
pub enum HubDevice {
    /// By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
}
