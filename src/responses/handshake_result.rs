use serde::Deserialize;

use super::TapoResponseExt;

#[derive(Debug, Deserialize)]
pub struct HandshakeResult {
    pub key: String,
}
impl TapoResponseExt for HandshakeResult {}
