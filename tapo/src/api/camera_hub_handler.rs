use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::io::AsyncWrite;
use tokio::sync::RwLock;

use crate::api::ApiClient;
use crate::error::{Error, TapoResponseError};
use crate::requests::{
    SmartCamGetGeneralDeviceListParams, SmartCamGetTimezoneParams,
    SmartCamSearchDateWithVideoParams, SmartCamSearchVideoWithUtcParams, TapoParams, TapoRequest,
};
use crate::responses::{
    DeviceInfoCameraHubResult, GeneralDeviceHubResult, GeneralDeviceListHubResultRaw,
    RecordingDateHubResult, RecordingDateListHubResultRaw, RecordingDownloadResult,
    RecordingHubResult, RecordingListHubResultRaw, TimezoneHubResult, TimezoneHubResultRaw,
};
use crate::utils::unix_timestamp_seconds;

/// Handler for camera hubs, such as the
/// [H200](https://www.tapo.com/en/search/?q=H200) and
/// [H500](https://www.tapo.com/en/search/?q=H500).
#[derive(Debug)]
pub struct CameraHubHandler {
    client: Arc<RwLock<ApiClient>>,
    ip_address: String,
    /// Identifies this handler to the hub, new for every handler. It is sent
    /// as `player_id` in clip searches (`searchVideoWithUTC`), and on the
    /// media stream as `playerId` in the stream URI and `player_id` in the
    /// download request.
    player_id: String,
}

impl CameraHubHandler {
    pub(crate) fn new(client: Arc<RwLock<ApiClient>>, ip_address: String) -> Self {
        Self {
            client,
            ip_address,
            player_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

tapo_handler!(@methods CameraHubHandler(DeviceInfoCameraHubResult));
tapo_handler!(@handler_ext CameraHubHandler);

/// Hub handler methods.
impl CameraHubHandler {
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

    /// Returns the hub's *timezone* as [`TimezoneHubResult`].
    ///
    /// # Errors
    ///
    /// Returns an error if the hub reports a timezone name that is not in the
    /// IANA database.
    pub async fn get_timezone(&self) -> Result<TimezoneHubResult, Error> {
        let request =
            TapoRequest::SmartCamGetTimezone(TapoParams::new(SmartCamGetTimezoneParams::new()));

        self.client
            .read()
            .await
            .execute_smart_cam_multiple_request::<TimezoneHubResultRaw>(request)
            .await?
            .map(|result| result.timezone())
            .ok_or(Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns the days that have recordings stored on the hub for the given
    /// camera (`searchDateWithVideo`), as [`Vec<RecordingDateHubResult>`].
    /// Each day comes with the range of time it covers on the hub, ready to
    /// pass to [`CameraHubHandler::get_recordings`].
    ///
    /// The hub searches its own calendar, so the search runs over the days
    /// that the given range touches in the hub's timezone, whole. A range that
    /// starts at 00:30 covers all of that day, and a returned day can reach
    /// past the range at either end.
    ///
    /// Reading the hub's timezone takes one extra request
    /// (see [`CameraHubHandler::get_timezone`]).
    ///
    /// # Arguments
    ///
    /// * `child_device_id` - the `device_id` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    /// * `start_time` - the start of the search range.
    /// * `end_time` - the end of the search range.
    ///
    /// # Errors
    ///
    /// Returns an error if `end_time` is before `start_time`, or if the hub
    /// reports a timezone name that is not in the IANA database.
    pub async fn get_recording_dates(
        &self,
        child_device_id: impl Into<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<RecordingDateHubResult>, Error> {
        if end_time < start_time {
            return Err(Error::Validation {
                field: "end_time".to_string(),
                message: "Must not be before start_time".to_string(),
            });
        }

        let timezone = self.get_timezone().await?.zone_id;

        let request = TapoRequest::SmartCamSearchDateWithVideo(TapoParams::new(
            SmartCamSearchDateWithVideoParams::new(
                start_time.with_timezone(&timezone).date_naive(),
                end_time.with_timezone(&timezone).date_naive(),
                child_device_id.into(),
            ),
        ));

        let dates = self
            .client
            .read()
            .await
            .execute_smart_cam_multiple_request::<RecordingDateListHubResultRaw>(request)
            .await?
            .ok_or(Error::Tapo(TapoResponseError::EmptyResult))?
            .dates(timezone)?;

        Ok(dates)
    }

    /// Returns the recordings stored on the hub for the given camera, within
    /// the given time range (`searchVideoWithUTC`), as
    /// [`Vec<RecordingHubResult>`]. Like the Tapo app, all pages are fetched,
    /// up to 12,300 recordings.
    ///
    /// # Arguments
    ///
    /// * `child_device_id` - the `device_id` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    /// * `start_time` - the start of the search range.
    /// * `end_time` - the end of the search range.
    ///
    /// # Errors
    ///
    /// Returns an error if `end_time` is before `start_time`, or if either is
    /// before 1970.
    pub async fn get_recordings(
        &self,
        child_device_id: impl Into<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<RecordingHubResult>, Error> {
        // The Tapo app fetches pages of 100 (`start_index: 0`, `end_index: 99`)
        // and stops paging past a start index of 12288.
        const PAGE_SIZE: u64 = 100;
        const MAX_START_INDEX: u64 = 12288;

        if end_time < start_time {
            return Err(Error::Validation {
                field: "end_time".to_string(),
                message: "Must not be before start_time".to_string(),
            });
        }

        let start_time = unix_timestamp_seconds("start_time", start_time)?;
        let end_time = unix_timestamp_seconds("end_time", end_time)?;
        let child_device_id = child_device_id.into();

        let client = self.client.read().await;

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
                    self.player_id.clone(),
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

    /// Downloads a recording stored on the hub over the hub's media stream
    /// service (TCP port 8800), writing the media to `writer` as it arrives.
    ///
    /// The hub streams the recording as encrypted MPEG-TS; the parts are
    /// decrypted with keys derived from the session's key exchange, so
    /// writing to a file with a `.ts` extension produces a playable clip. The
    /// hub ends the clip itself, with twice the clip's length on top of the
    /// client's timeout as a backstop.
    ///
    /// The media is written exactly as the hub sent it, with nothing added or
    /// re-muxed, so a player may log a one-off `TS discontinuity` warning for
    /// the stream's PAT and PMT when it opens the file. It is harmless: both
    /// tables are repeated throughout the stream and the clip plays in full.
    ///
    /// # Arguments
    ///
    /// * `child_device_id` - the `device_id` of a camera returned by [`CameraHubHandler::get_general_device_list`].
    /// * `start_time` - the `start_time` of a recording returned by [`CameraHubHandler::get_recordings`].
    /// * `end_time` - the `end_time` of that recording.
    /// * `writer` - where the media is written, e.g. a `Vec<u8>` or a `tokio::fs::File`.
    ///
    /// # Errors
    ///
    /// Returns an error if `end_time` is not after `start_time`, if either is
    /// before 1970, if the hub rejects the request or sends no media, or if it
    /// sends encrypted media that cannot be decrypted.
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
    /// let end_time = chrono::Utc::now();
    /// let start_time = end_time - chrono::Duration::days(1);
    /// let recordings = hub
    ///     .get_recordings(camera.device_id.clone(), start_time, end_time)
    ///     .await?;
    ///
    /// if let Some(recording) = recordings.first() {
    ///     let mut media = Vec::new();
    ///     hub.download_recording(
    ///         camera.device_id,
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
        child_device_id: impl Into<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        writer: &mut W,
    ) -> Result<RecordingDownloadResult, Error> {
        self.client
            .read()
            .await
            .download_recording(
                &self.ip_address,
                &self.player_id,
                child_device_id.into(),
                start_time,
                end_time,
                writer,
            )
            .await
    }
}

hub_child_handlers!(
    CameraHubHandler,
    "h200",
    child_device_list_note = "Cameras paired to the hub are not included; use \
        [`CameraHubHandler::get_general_device_list`] for those.",
);
