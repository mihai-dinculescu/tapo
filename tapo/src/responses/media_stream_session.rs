use serde::{Deserialize, Serialize};

/// An authenticated session with a camera hub's media stream service (TCP
/// port 8800), which the Tapo app uses for live view and for playing back
/// recordings stored on the hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStreamSession {
    /// The session identifier issued by the hub (`X-Session-Id`), when the
    /// hub reports one. The H200 does not.
    pub session_id: Option<String>,
    /// The heartbeat interval requested by the hub, in seconds (`X-Hb`),
    /// when the hub reports one.
    pub heartbeat_interval_s: Option<u64>,
    /// The key exchange value issued by the hub (`Key-Exchange`), when the
    /// hub reports one. The Tapo app echoes it back as `X-Chain` in the
    /// first control message of the session.
    pub key_exchange: Option<String>,
}
