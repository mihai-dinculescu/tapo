use crate::error::Error;
use crate::requests::{GenericSetDeviceInfoParams, TapoParams, TapoRequest};
use crate::responses::PowerStripPlugResult;

tapo_child_handler! {
    /// Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
    /// [P306](https://www.tp-link.com/us/search/?q=P306) child plugs.
    PowerStripPlugHandler(PowerStripPlugResult),
}

impl PowerStripPlugHandler {
    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }
}
