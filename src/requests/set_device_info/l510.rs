use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct L510SetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<u8>,
}

impl L510SetDeviceInfoParams {
    pub(crate) fn brightness(value: u8) -> anyhow::Result<Self> {
        Self {
            brightness: Some(value),
            ..Default::default()
        }
        .validate()
    }

    pub(crate) fn validate(self) -> anyhow::Result<Self> {
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
