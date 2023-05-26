use serde::Deserialize;

use super::TapoResponseExt;

#[derive(Debug, Deserialize)]
pub(crate) struct ControlChildResult<T> {
    #[serde(rename = "responseData")]
    pub response_data: T,
}

impl<T> TapoResponseExt for ControlChildResult<T> {}
