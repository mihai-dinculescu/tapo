use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GenericSetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_on: Option<bool>,
}

impl GenericSetDeviceInfoParams {
    pub fn device_on(value: bool) -> anyhow::Result<Self> {
        Self {
            device_on: Some(value),
        }
        .validate()
    }

    pub fn validate(self) -> anyhow::Result<Self> {
        if self.device_on.is_none() {
            return Err(anyhow::anyhow!(
                "DeviceInfoParams requires at least one property"
            ));
        }

        Ok(self)
    }
}
