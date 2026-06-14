use crate::error::Error;
use crate::responses::{ChildDeviceHubResult, ChildDeviceListHubResult, DeviceInfoCameraHubResult};

tapo_handler! {
    /// Handler for camera hubs, such as the
    /// [H200](https://www.tapo.com/en/search/?q=H200) and
    /// [H500](https://www.tapo.com/en/search/?q=H500).
    CameraHubHandler(DeviceInfoCameraHubResult),
}

/// Hub handler methods.
impl CameraHubHandler {
    /// Returns *child device list* as [`ChildDeviceHubResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API
    /// or to support all the possible devices connected to the hub.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`CameraHubHandler::get_child_device_list_json`].
    pub async fn get_child_device_list(&self) -> Result<Vec<ChildDeviceHubResult>, Error> {
        let mut results = Vec::new();
        let mut start_index = 0;
        let mut fetch = true;

        while fetch {
            let devices = self
                .client
                .read()
                .await
                .get_child_device_list::<ChildDeviceListHubResult>(start_index)
                .await
                .map(|r| r.devices)?;

            fetch = devices.len() == 10;
            start_index += 10;
            results.extend(devices);
        }

        Ok(results)
    }

    /// Returns *child device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    ///
    /// # Arguments
    ///
    /// * `start_index` - the index to start fetching the child device list.
    ///   It should be `0` for the first page, `10` for the second, and so on.
    #[cfg(feature = "debug")]
    pub async fn get_child_device_list_json(
        &self,
        start_index: u64,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .read()
            .await
            .get_child_device_list(start_index)
            .await
    }
}

hub_child_handlers!(CameraHubHandler, "h200");
