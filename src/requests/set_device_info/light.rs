use serde::Serialize;

use crate::api::ApiClientExt;
use crate::error::Error;

/// Builder that is used by the [`LightHandler::set`] API to set multiple properties in a single request.
#[derive(Debug, Serialize)]
pub(crate) struct LightSetDeviceInfoParams<'a> {
    #[serde(skip)]
    client: &'a dyn ApiClientExt,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<u8>,
}

impl<'a> LightSetDeviceInfoParams<'a> {
    /// Turns *on* the device. [`send`] must be called at the end to apply the changes.
    pub fn on(mut self) -> Self {
        self.device_on = Some(true);
        self
    }

    /// Turns *off* the device. [`send`] must be called at the end to apply the changes.
    pub fn off(mut self) -> Self {
        self.device_on = Some(false);
        self
    }

    /// Sets the *brightness*. [`send`] must be called at the end to apply the changes.
    /// The device will also be turned *on*, unless [`off`] is called.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub fn brightness(mut self, value: u8) -> Self {
        self.brightness = Some(value);
        self
    }

    /// Performs a request to apply the changes to the device.
    pub async fn send(self) -> Result<(), Error> {
        self.validate()?;
        let json = serde_json::to_value(&self)?;
        self.client.set_device_info(json).await
    }
}

impl<'a> LightSetDeviceInfoParams<'a> {
    pub(crate) fn new(client: &'a dyn ApiClientExt) -> Self {
        Self {
            client,
            device_on: None,
            brightness: None,
        }
    }

    fn validate(&self) -> Result<(), Error> {
        if self.device_on.is_none() && self.brightness.is_none() {
            return Err(Error::Validation {
                field: "DeviceInfoParams".to_string(),
                message: "requires at least one property".to_string(),
            });
        }

        if let Some(brightness) = self.brightness {
            if !(1..=100).contains(&brightness) {
                return Err(Error::Validation {
                    field: "brightness".to_string(),
                    message: "must be between 1 and 100".to_string(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    #[derive(Debug)]
    struct MockApiClient;

    #[async_trait]
    impl ApiClientExt for MockApiClient {
        async fn set_device_info(&self, _: serde_json::Value) -> Result<(), Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn no_property_validation() {
        let params = LightSetDeviceInfoParams::new(&MockApiClient);
        let result = params.send().await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "DeviceInfoParams" && message == "requires at least one property"
        ));
    }

    #[tokio::test]
    async fn brightness_validation() {
        let params = LightSetDeviceInfoParams::new(&MockApiClient);
        let result = params.brightness(0).send().await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "brightness" && message == "must be between 1 and 100"
        ));

        let params = LightSetDeviceInfoParams::new(&MockApiClient);
        let result = params.brightness(101).send().await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "brightness" && message == "must be between 1 and 100"
        ));
    }
}
