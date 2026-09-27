use serde::Deserialize;

use crate::responses::{TapoResponse, TapoResponseExt};

/// SmartCam `controlChild` item result. Unlike the Smart
/// [`ControlChildResult`](crate::responses::ControlChildResult), the field is
/// snake_case and wraps a single [`TapoResponse`] (not a multiple-request batch).
#[derive(Debug, Deserialize)]
pub(crate) struct SmartCamControlChildResult<T: TapoResponseExt> {
    /// Set instead of `response_data` when the hub refuses to relay the
    /// request, e.g. `-50021` for a model it does not support.
    #[serde(default)]
    pub err_code: i64,
    pub response_data: Option<TapoResponse<T>>,
}

impl<T: TapoResponseExt> TapoResponseExt for SmartCamControlChildResult<T> {}
