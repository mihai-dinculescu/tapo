use rmcp::ErrorData as McpError;
use thiserror::Error;

#[derive(Debug, Error)]
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

    #[error("Capability '{capability}' must be invoked via the dedicated `{tool}` tool")]
    WrongTool { capability: String, tool: String },
}

impl From<TapoMcpError> for McpError {
    fn from(err: TapoMcpError) -> Self {
        let message = error_message(&err);
        let data = Some(serde_json::json!({ "error": format!("{err:?}") }));
        match err {
            TapoMcpError::Internal(tapo::Error::Validation { .. }) => {
                McpError::invalid_params(message, data)
            }
            TapoMcpError::Internal(_)
            | TapoMcpError::InternalDiscovery(_)
            | TapoMcpError::Serialization(_) => McpError::internal_error(message, data),
            TapoMcpError::DeviceMismatch { .. } => McpError::invalid_params(message, data),
            TapoMcpError::DeviceNotFound { .. } => McpError::resource_not_found(message, data),
            TapoMcpError::WrongDeviceType { .. } => McpError::invalid_params(message, data),
            TapoMcpError::CameraCredentialsMissing => McpError::invalid_params(message, data),
            TapoMcpError::WrongTool { .. } => McpError::invalid_params(message, data),
        }
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
fn error_message(err: &(dyn std::error::Error + 'static)) -> String {
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
    fn error_message_leaves_leaf_errors_unchanged() {
        let err = TapoMcpError::CameraCredentialsMissing;

        assert_eq!(error_message(&err), err.to_string());
    }
}
