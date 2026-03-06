use crate::error::Error;
#[cfg(feature = "debug")]
use crate::responses::ChildDeviceComponentList;
use crate::responses::{
    ChildDeviceListPowerStripResult, DeviceInfoPowerStripResult, PowerStripPlugResult,
};

use super::{Plug, PowerStripPlugHandler};

tapo_handler! {
    /// Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
    /// [P306](https://www.tp-link.com/us/search/?q=P306) devices.
    PowerStripHandler(DeviceInfoPowerStripResult),
    device_management,
}

impl PowerStripHandler {
    /// Returns *child device list* as [`Vec<PowerStripPlugResult>`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present,
    /// try [`PowerStripHandler::get_child_device_list_json`].
    pub async fn get_child_device_list(&self) -> Result<Vec<PowerStripPlugResult>, Error> {
        self.client
            .read()
            .await
            .get_child_device_list::<ChildDeviceListPowerStripResult>(0)
            .await
            .map(|r| r.plugs)
    }

    /// Returns *child device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    #[cfg(feature = "debug")]
    pub async fn get_child_device_list_json(&self) -> Result<serde_json::Value, Error> {
        self.client.read().await.get_child_device_list(0).await
    }

    /// Returns *child device component list* as [`Vec<ChildDeviceComponentList>`].
    /// This information is useful in debugging or when investigating new functionality to add.
    #[cfg(feature = "debug")]
    pub async fn get_child_device_component_list(
        &self,
    ) -> Result<Vec<ChildDeviceComponentList>, Error> {
        self.client
            .read()
            .await
            .get_child_device_component_list()
            .await
    }
}

/// Child device handler builders.
impl PowerStripHandler {
    /// Returns a [`PowerStripPlugHandler`] for the given [`Plug`].
    ///
    /// # Arguments
    ///
    /// * `identifier` - a PowerStrip plug identifier.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::{ApiClient, Plug};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Connect to the hub
    /// let power_strip = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p300("192.168.1.100")
    ///     .await?;
    /// // Get a handler for the child device
    /// let device_id = "0000000000000000000000000000000000000000".to_string();
    /// let device = power_strip.plug(Plug::ByDeviceId(device_id)).await?;
    /// // Get the device info of the child device
    /// let device_info = device.get_device_info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn plug(&self, identifier: Plug) -> Result<PowerStripPlugHandler, Error> {
        let children = self.get_child_device_list().await?;

        let device_id = match identifier {
            Plug::ByDeviceId(device_id) => children
                .iter()
                .find(|child| child.device_id == device_id)
                .ok_or_else(|| Error::DeviceNotFound)?
                .device_id
                .clone(),
            Plug::ByNickname(nickname) => children
                .iter()
                .find(|child| child.nickname == nickname)
                .ok_or_else(|| Error::DeviceNotFound)?
                .device_id
                .clone(),
            Plug::ByPosition(position) => children
                .iter()
                .find(|child| child.position == position)
                .ok_or_else(|| Error::DeviceNotFound)?
                .device_id
                .clone(),
        };

        Ok(PowerStripPlugHandler::new(self.client.clone(), device_id))
    }
}
