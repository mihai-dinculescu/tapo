use serde::{Deserialize, Serialize};

use super::TapoResponseExt;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HandshakeResult {
    pub key: String,
}
impl TapoResponseExt for HandshakeResult {}
