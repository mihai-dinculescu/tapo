use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenResult {
    pub token: String,
}
impl TapoResponseExt for TokenResult {}
