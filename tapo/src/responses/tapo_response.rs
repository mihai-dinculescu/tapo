use serde::{Deserialize, Serialize};

use crate::error::{Error, TapoResponseError};

/// Implemented by all Tapo Responses.
pub(crate) trait TapoResponseExt {}

impl TapoResponseExt for serde_json::Value {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TapoResponse<T: TapoResponseExt> {
    pub error_code: i32,
    pub result: Option<T>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TapoMultipleResponse<T: TapoResponseExt> {
    pub result: TapoMultipleResult<T>,
}

impl<T> TapoResponseExt for TapoMultipleResponse<T> where T: TapoResponseExt {}

#[derive(Debug, Deserialize)]
pub(crate) struct TapoMultipleResult<T: TapoResponseExt> {
    pub responses: Vec<TapoResponse<T>>,
}

pub(crate) fn validate_response<T: TapoResponseExt>(
    response: &TapoResponse<T>,
) -> Result<(), Error> {
    match response.error_code {
        0 => Ok(()),
        -1002 => Err(Error::Tapo(TapoResponseError::InvalidRequest)),
        -1003 => Err(Error::Tapo(TapoResponseError::MalformedRequest)),
        -1008 => Err(Error::Tapo(TapoResponseError::InvalidParameters)),
        -1010 => Err(Error::Tapo(TapoResponseError::InvalidPublicKey)),
        -1501 => Err(Error::Tapo(TapoResponseError::InvalidCredentials)),
        9999 => Err(Error::Tapo(TapoResponseError::SessionTimeout)),
        code => Err(Error::Tapo(TapoResponseError::Unknown(code))),
    }
}
