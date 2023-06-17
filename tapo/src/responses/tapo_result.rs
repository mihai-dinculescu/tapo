use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TapoResult {
    pub response: String,
}
impl TapoResponseExt for TapoResult {}
