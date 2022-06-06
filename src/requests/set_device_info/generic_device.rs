use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct GenericSetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) device_on: Option<bool>,
}

impl GenericSetDeviceInfoParams {
    pub(crate) fn device_on(value: bool) -> anyhow::Result<Self> {
        Self {
            device_on: Some(value),
        }
        .validate()
    }

    pub(crate) fn validate(self) -> anyhow::Result<Self> {
        if self.device_on.is_none() {
            return Err(anyhow::anyhow!(
                "DeviceInfoParams requires at least one property"
            ));
        }

        Ok(self)
    }
}
