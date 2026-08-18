use chrono::NaiveDate;

use crate::error::{Error, TapoResponseError};
use crate::requests::{
    SmartCamGetGeneralDeviceListParams, SmartCamSearchDateWithVideoParams,
    SmartCamSearchVideoWithUtcParams, TapoParams, TapoRequest,
};
use crate::responses::{
    ChildDeviceHubResult, ChildDeviceListHubResult, DeviceInfoCameraHubResult,
    GeneralDeviceHubResult, GeneralDeviceListHubResultRaw, RecordingDateListHubResultRaw,
    RecordingHubResult, RecordingListHubResultRaw,
};

#[cfg(feature = "debug")]
use crate::responses::ChildDeviceComponentList;

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

    /// Returns *general device list* as [`GeneralDeviceHubResult`].
    /// These are the standalone Wi-Fi cameras paired to the hub.
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`CameraHubHandler::get_general_device_list_json`].
    pub async fn get_general_device_list(&self) -> Result<Vec<GeneralDeviceHubResult>, Error> {
        let request = TapoRequest::SmartCamGetGeneralDeviceList(TapoParams::new(
            SmartCamGetGeneralDeviceListParams::new(),
        ));

        self.client
            .read()
            .await
            .execute_smart_cam_multiple_request::<GeneralDeviceListHubResultRaw>(request)
            .await?
            .map(|result| result.devices())
            .ok_or(Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns *general device list* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API for the
    /// standalone Wi-Fi cameras paired to the hub.
    #[cfg(feature = "debug")]
    pub async fn get_general_device_list_json(&self) -> Result<serde_json::Value, Error> {
        let request = TapoRequest::SmartCamGetGeneralDeviceList(TapoParams::new(
            SmartCamGetGeneralDeviceListParams::new(),
        ));

        self.client
            .read()
            .await
            .execute_smart_cam_multiple_request::<serde_json::Value>(request)
            .await?
            .ok_or(Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns the dates that have recordings stored on the hub for the given camera,
    /// within the given date range, as [`Vec<chrono::NaiveDate>`].
    /// The dates are calendar days in the hub's local timezone, not UTC.
    ///
    /// # Arguments
    ///
    /// * `start_date` - the first date of the search range.
    /// * `end_date` - the last date of the search range (inclusive).
    /// * `child_device_id` - the `device_id` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    /// * `child_device_mac` - the `mac` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    pub async fn search_date_with_video(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        child_device_id: impl Into<String>,
        child_device_mac: impl Into<String>,
    ) -> Result<Vec<NaiveDate>, Error> {
        let request = TapoRequest::SmartCamSearchDateWithVideo(TapoParams::new(
            SmartCamSearchDateWithVideoParams::new(
                start_date,
                end_date,
                child_device_id.into(),
                child_device_mac.into(),
            ),
        ));

        let dates = self
            .client
            .read()
            .await
            .execute_smart_cam_multiple_request::<RecordingDateListHubResultRaw>(request)
            .await?
            .ok_or(Error::Tapo(TapoResponseError::EmptyResult))?
            .dates()
            .map_err(anyhow::Error::from)?;

        Ok(dates)
    }

    /// Returns the recordings stored on the hub for the given camera, within the
    /// given time range, as [`Vec<RecordingHubResult>`]. All pages are fetched.
    ///
    /// # Arguments
    ///
    /// * `start_time` - the start of the search range as a Unix timestamp (seconds).
    /// * `end_time` - the end of the search range as a Unix timestamp (seconds).
    /// * `child_device_id` - the `device_id` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    /// * `child_device_mac` - the `mac` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    pub async fn search_video_with_utc(
        &self,
        start_time: u64,
        end_time: u64,
        child_device_id: impl Into<String>,
        child_device_mac: impl Into<String>,
    ) -> Result<Vec<RecordingHubResult>, Error> {
        // The Tapo app fetches pages of 100 (`start_index: 0`, `end_index: 99`)
        // and keeps going while the response reports `to_be_continued`.
        const PAGE_SIZE: u64 = 100;

        let child_device_id = child_device_id.into();
        let child_device_mac = child_device_mac.into();

        let mut results = Vec::new();
        let mut start_index = 0;

        loop {
            let request = TapoRequest::SmartCamSearchVideoWithUtc(TapoParams::new(
                SmartCamSearchVideoWithUtcParams::new(
                    start_time,
                    end_time,
                    start_index,
                    start_index + PAGE_SIZE - 1,
                    child_device_id.clone(),
                    child_device_mac.clone(),
                ),
            ));

            let (recordings, to_be_continued) = self
                .client
                .read()
                .await
                .execute_smart_cam_multiple_request::<RecordingListHubResultRaw>(request)
                .await?
                .ok_or(Error::Tapo(TapoResponseError::EmptyResult))?
                .into_parts();

            // An empty page also stops the loop, in case a device claims
            // `to_be_continued` without ever returning more recordings.
            let page_is_empty = recordings.is_empty();
            results.extend(recordings);

            if !to_be_continued || page_is_empty {
                break;
            }
            start_index += PAGE_SIZE;
        }

        Ok(results)
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

hub_child_handlers!(CameraHubHandler, "h200");
