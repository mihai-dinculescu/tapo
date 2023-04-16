use serde::Deserialize;

use crate::responses::TapoResponseExt;

#[derive(Debug, Deserialize)]
pub(crate) struct TokenResult {
    pub token: String,
}
impl TapoResponseExt for TokenResult {}
