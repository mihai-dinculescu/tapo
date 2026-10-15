//! RTSP stream URLs and snapshots, shared by the camera handlers.

use std::time::Duration;

use crate::api::rtsp_snapshot::grab_mjpeg_frame;
use crate::error::Error;
use crate::responses::{RtspStreamUrl, Snapshot};

/// Returns the RTSP stream URLs of the camera at `ip`, with the credentials
/// URL-encoded into them.
pub(crate) fn rtsp_stream_url(ip: &str, username: &str, password: &str) -> RtspStreamUrl {
    RtspStreamUrl {
        hd: rtsp_url(ip, "stream1", username, password),
        sd: rtsp_url(ip, "stream2", username, password),
        mjpeg: rtsp_url(ip, "stream8", username, password),
    }
}

/// Captures a JPEG snapshot from the MJPEG stream of the camera at `ip`.
pub(crate) async fn snapshot(ip: &str, username: &str, password: &str) -> Result<Snapshot, Error> {
    // retina rejects URLs with embedded credentials when creds are also
    // passed via SessionOptions, so use the bare base URL here.
    let url = rtsp_url_base(ip, "stream8");
    let creds = retina::client::Credentials {
        username: username.into(),
        password: password.into(),
    };
    let data = grab_mjpeg_frame(&url, creds, Duration::from_secs(5)).await?;

    Ok(Snapshot {
        data,
        content_type: "image/jpeg".into(),
    })
}

fn rtsp_url_base(ip: &str, stream: &str) -> String {
    format!("rtsp://{ip}:554/{stream}")
}

fn rtsp_url(ip: &str, stream: &str, username: &str, password: &str) -> String {
    // safe: `ip` is the address the handler logged in to, so the URL parses
    let mut url = reqwest::Url::parse(&rtsp_url_base(ip, stream)).expect("valid RTSP base URL");
    // safe: an `rtsp://` URL with a host always accepts credentials
    url.set_username(username).expect("valid username");
    url.set_password(Some(password)).expect("valid password");
    url.to_string()
}
