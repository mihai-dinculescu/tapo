use serde::Deserialize;

use crate::responses::TapoResponseExt;

#[derive(Debug, Deserialize)]
pub(crate) struct TapoResult {
    pub response: String,
}
impl TapoResponseExt for TapoResult {}
