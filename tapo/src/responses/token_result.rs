use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

#[derive(Serialize, Deserialize)]
pub(crate) struct TokenResult {
    pub token: String,
}
impl TapoResponseExt for TokenResult {}
