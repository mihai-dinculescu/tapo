use rmcp::ErrorData as McpError;
use rmcp::model::{ContentBlock, IntoContents};
use strum::IntoStaticStr;
use tapo::TapoResponseError;
use thiserror::Error;

#[derive(Debug, Error, IntoStaticStr)]
pub enum TapoMcpError {
    #[error(transparent)]
    Internal(#[from] tapo::Error),

    #[error(transparent)]
    InternalDiscovery(tapo::DiscoveryError),

    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error(
        "Device mismatch: expected id '{expected_id}' at ip '{expected_ip}', found id '{found_id}' at ip '{found_ip}'"
    )]
    DeviceMismatch {
        expected_id: String,
        expected_ip: String,
        found_id: String,
        found_ip: String,
    },

    #[error("Device not found: id '{id}' at ip '{ip}'")]
    DeviceNotFound { id: String, ip: String },

    #[error("Device timed out: id '{id}' at ip '{ip}' did not finish within {seconds} seconds")]
    DeviceTimeout {
        id: String,
        ip: String,
        seconds: u64,
    },

    #[error(
        "Device timed out: id '{id}' at ip '{ip}' did not finish within {seconds} seconds. {}",
        control_progress(*.applied, *.total, *.in_flight)
    )]
    ControlTimeout {
        id: String,
        ip: String,
        seconds: u64,
        applied: usize,
        total: usize,
        /// The name of the capability after the applied ones, when applying it
        /// had started as the timeout fired, so the device may have applied it too.
        in_flight: Option<&'static str>,
    },

    #[error(
        "{}",
        failed_progress(*.applied, *.total, .failed, .source)
    )]
    ControlFailed {
        applied: usize,
        total: usize,
        /// The name of the capability the device failed to apply.
        failed: &'static str,
        #[source]
        source: tapo::Error,
    },

    #[error("Capability '{capability}' requires {expected}; device id '{id}' is not eligible")]
    WrongDeviceType {
        id: String,
        capability: String,
        /// Human-readable description of the device type the capability needs.
        expected: String,
    },

    #[error(
        "Camera account credentials not configured: set TAPO_MCP_CAMERA_USERNAME and TAPO_MCP_CAMERA_PASSWORD (Camera Settings > Advanced Settings > Camera Account in the Tapo app)"
    )]
    CameraCredentialsMissing,

    #[error("Invalid parameter '{name}': {message}")]
    InvalidParameter { name: String, message: String },

    #[error("Device discovery unavailable: {reason}")]
    DiscoveryUnavailable { reason: String },
}

fn applied_progress(applied: usize, total: usize) -> String {
    let noun = if total == 1 {
        "capability"
    } else {
        "capabilities"
    };
    format!("Applied {applied} of {total} {noun}")
}

fn control_progress(applied: usize, total: usize, in_flight: Option<&str>) -> String {
    let progress = format!("{} before the timeout", applied_progress(applied, total));
    match in_flight {
        Some(name) => format!(
            "{progress}; capability {} ({name}) may also have been applied",
            applied + 1
        ),
        None => progress,
    }
}

fn failed_progress(applied: usize, total: usize, failed: &str, source: &tapo::Error) -> String {
    let progress = applied_progress(applied, total);
    let position = applied + 1;
    if may_have_applied(source) {
        format!(
            "{progress}; capability {position} ({failed}) failed and may still have been applied"
        )
    } else {
        format!("{progress} before capability {position} ({failed}) failed")
    }
}

/// Whether the device may have applied a request that failed with `source`.
fn may_have_applied(source: &tapo::Error) -> bool {
    match source {
        // The device processed the request and answered with an error, or
        // refused the session.
        tapo::Error::Tapo(
            TapoResponseError::DeviceError { .. } | TapoResponseError::Unauthorized { .. },
        ) => false,
        // The request never reached the device.
        tapo::Error::Validation { .. } => false,
        tapo::Error::Http(err) => !err.is_connect(),
        _ => true,
    }
}

/// A tool's failure becomes a tool result with `is_error` set and the error
/// message as text, so the model sees what went wrong.
impl IntoContents for TapoMcpError {
    fn into_contents(self) -> Vec<ContentBlock> {
        vec![ContentBlock::text(error_message(&self))]
    }
}

/// Only reading the `tapo://devices` resource fails with a protocol error, since
/// resource reads have no `is_error` equivalent.
impl From<TapoMcpError> for McpError {
    fn from(err: TapoMcpError) -> Self {
        let message = error_message(&err);
        let data = Some(serde_json::json!({ "error": format!("{err:?}") }));
        McpError::internal_error(message, data)
    }
}

/// Formats an error followed by its chain of causes, joined with `: `.
///
/// reqwest's `Display` stops at the request URL, so a transport failure reads
/// as "error sending request for url (...)" with no hint of whether the
/// connection was refused, dropped mid-request, or timed out. Walking the
/// source chain brings that detail into the message the MCP client shows.
///
/// A cause whose text is already part of the message so far is skipped, since
/// some errors (e.g. `tapo::DiscoveryError`) embed their source in their own
/// `Display` output.
pub(crate) fn error_message(err: &(dyn std::error::Error + 'static)) -> String {
    anyhow::Chain::new(err)
        .map(ToString::to_string)
        .fold(String::new(), |mut message, cause| {
            if !message.contains(&cause) {
                if !message.is_empty() {
                    message.push_str(": ");
                }
                message.push_str(&cause);
            }
            message
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_message_appends_causes() {
        let err = TapoMcpError::Internal(tapo::Error::Other(
            anyhow::anyhow!("connection reset by peer").context("error sending request"),
        ));

        assert_eq!(
            error_message(&err),
            "error sending request: connection reset by peer"
        );
    }

    #[test]
    fn error_message_skips_causes_already_in_message() {
        let err = TapoMcpError::InternalDiscovery(tapo::DiscoveryError {
            ip: "192.168.1.126".to_string(),
            source: tapo::Error::Other(
                anyhow::anyhow!("connection reset by peer").context("error sending request"),
            ),
        });

        assert_eq!(
            error_message(&err),
            "Failed to discover device at 192.168.1.126: error sending request: connection reset by peer"
        );
    }

    #[test]
    fn control_timeout_names_the_capability_in_flight() {
        let err = TapoMcpError::ControlTimeout {
            id: "id".to_string(),
            ip: "192.168.1.100".to_string(),
            seconds: 70,
            applied: 1,
            total: 2,
            in_flight: Some("Brightness"),
        };

        assert_eq!(
            err.to_string(),
            "Device timed out: id 'id' at ip '192.168.1.100' did not finish within 70 seconds. Applied 1 of 2 capabilities before the timeout; capability 2 (Brightness) may also have been applied"
        );
    }

    #[test]
    fn control_timeout_before_any_capability_was_sent() {
        let err = TapoMcpError::ControlTimeout {
            id: "id".to_string(),
            ip: "192.168.1.100".to_string(),
            seconds: 70,
            applied: 0,
            total: 1,
            in_flight: None,
        };

        assert_eq!(
            err.to_string(),
            "Device timed out: id 'id' at ip '192.168.1.100' did not finish within 70 seconds. Applied 0 of 1 capability before the timeout"
        );
    }

    #[test]
    fn control_failed_says_how_many_capabilities_were_applied() {
        let err = TapoMcpError::ControlFailed {
            applied: 1,
            total: 2,
            failed: "Color",
            source: tapo::Error::Tapo(TapoResponseError::DeviceError {
                code: -1008,
                kind: "INVALID_PARAMS",
            }),
        };

        assert_eq!(
            err.to_string(),
            "Applied 1 of 2 capabilities before capability 2 (Color) failed"
        );
    }

    #[test]
    fn control_failed_may_have_applied_the_failed_capability() {
        let err = TapoMcpError::ControlFailed {
            applied: 1,
            total: 2,
            failed: "Color",
            source: tapo::Error::Other(anyhow::anyhow!("connection reset by peer")),
        };

        assert_eq!(
            error_message(&err),
            "Applied 1 of 2 capabilities; capability 2 (Color) failed and may still have been applied: connection reset by peer"
        );
    }

    #[test]
    fn error_message_leaves_leaf_errors_unchanged() {
        let err = TapoMcpError::CameraCredentialsMissing;

        assert_eq!(error_message(&err), err.to_string());
    }
}
