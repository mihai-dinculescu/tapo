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

    #[error("Unsupported capability '{capability}' for device id '{id}'")]
    UnsupportedCapability { id: String, capability: String },
}

impl From<TapoMcpError> for McpError {
    fn from(err: TapoMcpError) -> Self {
        let data = Some(serde_json::json!({ "error": format!("{err:?}") }));
        match err {
            TapoMcpError::Internal(tapo::Error::Validation { .. }) => {
                McpError::invalid_params(err.to_string(), data)
            }
            TapoMcpError::Internal(_)
            | TapoMcpError::InternalDiscovery(_)
            | TapoMcpError::Serialization(_) => McpError::internal_error(err.to_string(), data),
            TapoMcpError::DeviceMismatch { .. } => McpError::invalid_params(err.to_string(), data),
            TapoMcpError::DeviceNotFound { .. } => {
                McpError::resource_not_found(err.to_string(), data)
            }
            TapoMcpError::UnsupportedCapability { .. } => {
                McpError::invalid_params(err.to_string(), data)
            }
        }
    }
}
