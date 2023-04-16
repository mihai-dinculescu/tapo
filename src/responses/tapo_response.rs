use serde::Deserialize;

use crate::error::{Error, TapoResponseError};

/// Implemented by all Tapo Responses.
pub(crate) trait TapoResponseExt {}

impl TapoResponseExt for serde_json::Value {}

#[derive(Debug, Deserialize)]
pub(crate) struct TapoResponse<T: TapoResponseExt> {
    pub error_code: i32,
    pub result: Option<T>,
}

pub(crate) fn validate_response<T: TapoResponseExt>(
    response: &TapoResponse<T>,
) -> Result<(), Error> {
    match response.error_code {
        0 => Ok(()),
        -1501 => Err(Error::Tapo(TapoResponseError::InvalidCredentials)),
        code => Err(Error::Tapo(TapoResponseError::Unknown(code))),
    }
}
