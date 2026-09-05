use serde::Deserialize;

use super::{TapoResponse, TapoResponseExt};

#[derive(Debug, Deserialize)]
pub(crate) struct ControlChildResult<T> {
    #[serde(rename = "responseData")]
    pub response_data: T,
}

impl<T> TapoResponseExt for ControlChildResult<T> {}

/// SmartCam `controlChild` item result. Unlike the Smart [`ControlChildResult`],
/// the field is snake_case and wraps a single [`TapoResponse`] (not a
/// multiple-request batch).
#[derive(Debug, Deserialize)]
pub(crate) struct SmartCamControlChildResult<T: TapoResponseExt> {
    pub response_data: TapoResponse<T>,
}

impl<T: TapoResponseExt> TapoResponseExt for SmartCamControlChildResult<T> {}
