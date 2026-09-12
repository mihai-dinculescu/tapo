use tokio::io::AsyncWrite;

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

use crate::responses::RecordingDownloadResult;

#[cfg(feature = "debug")]
use crate::responses::ChildDeviceComponentList;

tapo_handler! {
    /// Handler for camera hubs, such as the
    /// [H200](https://www.tapo.com/en/search/?q=H200) and
    /// [H500](https://www.tapo.com/en/search/?q=H500).
    CameraHubHandler(DeviceInfoCameraHubResult),
    ip_address,
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
    /// given time range, as [`Vec<RecordingHubResult>`]. Like the Tapo app, all
    /// pages are fetched, up to 12,300 recordings.
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
        // and stops paging past a start index of 12288.
        const PAGE_SIZE: u64 = 100;
        const MAX_START_INDEX: u64 = 12288;

        let child_device_id = child_device_id.into();
        let child_device_mac = child_device_mac.into();

        let client = self.client.read().await;
        let player_id = client.player_id().to_string();

        let mut results = Vec::new();
        let mut start_index = 0;

        while start_index <= MAX_START_INDEX {
            let request = TapoRequest::SmartCamSearchVideoWithUtc(TapoParams::new(
                SmartCamSearchVideoWithUtcParams::new(
                    start_time,
                    end_time,
                    start_index,
                    start_index + PAGE_SIZE - 1,
                    child_device_id.clone(),
                    child_device_mac.clone(),
                    player_id.clone(),
                ),
            ));

            let recordings = client
                .execute_smart_cam_multiple_request::<RecordingListHubResultRaw>(request)
                .await?
                .ok_or(Error::Tapo(TapoResponseError::EmptyResult))?
                .recordings();

            // The H200 sends no `to_be_continued` flag, so like the Tapo app
            // (`PlaybackHubRepository`) in that case, a full page means that
            // another may follow.
            let page_is_full = recordings.len() as u64 >= PAGE_SIZE;
            results.extend(recordings);

            if !page_is_full {
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

    /// Downloads a recording stored on the hub over the hub's media stream
    /// service (TCP port 8800), writing the media to `writer` as it arrives.
    ///
    /// The hub streams the recording as encrypted MPEG-TS; the parts are
    /// decrypted with keys derived from the session's key exchange, so
    /// writing to a file with a `.ts` extension produces a playable clip. The
    /// hub plays on past the recording's end, so the method stops once the
    /// stream's clock has covered the clip's length (or when the hub reports
    /// the end of the footage), with twice the clip's length on top of the
    /// client's timeout as a backstop.
    ///
    /// Fails if the hub sends no media, or if it sends encrypted media that
    /// cannot be decrypted.
    ///
    /// # Arguments
    ///
    /// * `child_device_mac` - the `mac` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    /// * `start_time` - the `start_time` of a recording returned by [`CameraHubHandler::search_video_with_utc`].
    /// * `end_time` - the `end_time` of that recording.
    /// * `writer` - where the media is written, e.g. a `Vec<u8>` or a `tokio::fs::File`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let hub = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h200("192.168.1.100")
    ///     .await?;
    ///
    /// let camera = hub
    ///     .get_general_device_list()
    ///     .await?
    ///     .into_iter()
    ///     .next()
    ///     .expect("no camera is paired to the hub");
    ///
    /// let end_time = chrono::Utc::now().timestamp() as u64;
    /// let start_time = end_time - 24 * 60 * 60;
    /// let recordings = hub
    ///     .search_video_with_utc(start_time, end_time, camera.device_id, camera.mac.clone())
    ///     .await?;
    ///
    /// if let Some(recording) = recordings.first() {
    ///     let mut media = Vec::new();
    ///     hub.download_recording(
    ///         camera.mac,
    ///         recording.start_time,
    ///         recording.end_time,
    ///         &mut media,
    ///     )
    ///     .await?;
    ///     std::fs::write("recording.ts", media)?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_recording<W: AsyncWrite + Unpin + Send>(
        &self,
        child_device_mac: impl Into<String>,
        start_time: u64,
        end_time: u64,
        writer: &mut W,
    ) -> Result<RecordingDownloadResult, Error> {
        self.client
            .read()
            .await
            .download_recording(
                &self.ip_address,
                child_device_mac.into(),
                start_time,
                end_time,
                writer,
            )
            .await
    }
}

hub_child_handlers!(CameraHubHandler, "h200");
