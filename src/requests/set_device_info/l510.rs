use serde::Serialize;

use crate::api::ApiClient;
use crate::devices::L510;

/// Builder that is used by the [`crate::ApiClient<L510>::set`] API to set multiple properties in a single request.
#[derive(Debug, Serialize)]
pub struct L510SetDeviceInfoParams<'a> {
    #[serde(skip)]
    client: &'a ApiClient<L510>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<u8>,
}

impl<'a> L510SetDeviceInfoParams<'a> {
    /// Turns *on* the device. `send` must be called at the end to apply the changes.
    pub fn on(mut self) -> Self {
        self.device_on = Some(true);
        self
    }

    /// Turns *off* the device. `send` must be called at the end to apply the changes.
    pub fn off(mut self) -> Self {
        self.device_on = Some(false);
        self
    }

    /// Sets the *brightness*. `send` must be called at the end to apply the changes.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub fn brightness(mut self, value: u8) -> anyhow::Result<Self> {
        self.brightness = Some(value);
        self.validate()
    }

    /// Performs a request to apply the changes to the device.
    pub async fn send(self) -> anyhow::Result<()> {
        let json = serde_json::to_value(&self)?;
        self.client.set_device_info_internal(json).await
    }
}

impl<'a> L510SetDeviceInfoParams<'a> {
    pub(crate) fn new(client: &'a ApiClient<L510>) -> Self {
        Self {
            client,
            device_on: None,
            brightness: None,
        }
    }

    fn validate(self) -> anyhow::Result<Self> {
        if self.brightness.is_none() {
            return Err(anyhow::anyhow!(
                "DeviceInfoParams requires at least one property"
            ));
        }

        if let Some(brightness) = self.brightness {
            if !(1..=100).contains(&brightness) {
                return Err(anyhow::anyhow!("'brightness' must be between 1 and 100"));
            }
        }

        Ok(self)
    }
}
