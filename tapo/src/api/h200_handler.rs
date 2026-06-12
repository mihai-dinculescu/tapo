use crate::responses::DeviceInfoH200Result;

tapo_handler! {
    /// Handler for the [H200](https://www.tapo.com/en/search/?q=H200) devices.
    H200Handler(DeviceInfoH200Result),
}

/// Hub handler methods.
impl H200Handler {
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
    ) -> Result<serde_json::Value, crate::error::Error> {
        self.client
            .read()
            .await
            .get_child_device_list(start_index)
            .await
    }
}
