use serde::{Deserialize, Serialize};

use crate::error::{Error, TapoResponseError};

/// Implemented by all Tapo Responses.
pub(crate) trait TapoResponseExt {}

impl TapoResponseExt for serde_json::Value {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TapoResponse<T: TapoResponseExt> {
    pub error_code: i64,
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

pub(crate) fn validate_response(error_code: i64) -> Result<(), Error> {
    match error_code {
        0 => Ok(()),
        -1501 => Err(Error::Tapo(TapoResponseError::Unauthorized {
            kind: "INVALID_CREDENTIALS",
            description:
                "Please verify that your email and password are correct—both are case-sensitive."
                    .to_string(),
        })),
        9999 => Err(Error::Tapo(TapoResponseError::Unauthorized {
            kind: "SESSION_TIMEOUT",
            description: "Session has expired. Re-authentication is required.".to_string(),
        })),
        code => Err(Error::Tapo(TapoResponseError::DeviceError {
            code,
            kind: error_kind(code),
        })),
    }
}

fn error_kind(code: i64) -> &'static str {
    match code {
        -1002 => "UNKNOWN_METHOD",
        -1003 => "JSON_DECODE_FAIL",
        -1004 => "JSON_ENCODE_FAIL",
        -1008 => "PARAMS",
        -1010 => "PUBLIC_KEY",
        _ => "UNKNOWN",
    }
}
