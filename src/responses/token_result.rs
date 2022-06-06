use serde::Deserialize;

use super::TapoResponseExt;

#[derive(Debug, Deserialize)]
pub struct TokenResult {
    pub token: String,
}
impl TapoResponseExt for TokenResult {}
