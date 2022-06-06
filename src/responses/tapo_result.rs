use serde::Deserialize;

use super::TapoResponseExt;

#[derive(Debug, Deserialize)]
pub struct TapoResult {
    pub response: String,
}
impl TapoResponseExt for TapoResult {}
