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
    if error_code == 0 {
        return Ok(());
    }

    let kind = error_kind(error_code);

    let error = match kind {
        "LOGIN" => TapoResponseError::Unauthorized {
            kind,
            description:
                "Please verify that your email and password are correct—both are case-sensitive."
                    .to_string(),
        },
        "SESSION_TIMEOUT" | "SESSION_EXPIRED" => TapoResponseError::session_expired(kind),
        _ => TapoResponseError::DeviceError {
            code: error_code,
            kind,
        },
    };

    Err(Error::Tapo(error))
}

fn error_kind(code: i64) -> &'static str {
    match code {
        // Smart Devices
        -1002 => "UNKNOWN_METHOD",
        -1003 => "JSON_DECODE_FAIL",
        -1004 => "JSON_ENCODE_FAIL",
        -1008 => "PARAMS",
        -1010 => "PUBLIC_KEY",
        -1501 => "LOGIN",
        9999 => "SESSION_TIMEOUT",
        // SmartCam Devices
        -40106 => "UNSUPPORTED_METHOD",
        -40109 => "ONE_SECOND_REPEAT_REQUEST",
        -40203 => "BIND_SENSOR_EXISTS",
        -40210 => "PROTOCOL_FORMAT_ERROR",
        -40321 => "IP_CONFLICT",
        -40401 => "SESSION_EXPIRED",
        -40404 => "DEVICE_BLOCKED",
        -40405 => "DEVICE_FACTORY",
        -40406 => "OUT_OF_LIMIT",
        -40407 => "OTHER_ERROR",
        -40408 => "SYSTEM_BLOCKED",
        -40409 => "NONCE_EXPIRED",
        -40412 => "HOMEKIT_LOGIN_FAIL",
        -40413 => "INVALID_NONCE",
        -40414 => "NEED_LOGIN_BY_LOCAL_PASSWORD",
        -60506 => "LOCAL_ACCOUNT_OLD_PWD_ERROR",
        -69051 => "DIAGNOSE_TYPE_NOT_SUPPORT",
        -69052 => "DIAGNOSE_TASK_FULL",
        -69053 => "DIAGNOSE_TASK_BUSY",
        -69055 => "DIAGNOSE_INTERNAL_ERROR",
        -69056 => "DIAGNOSE_ID_NOT_FOUND",
        -69057 => "DIAGNOSE_TASK_NULL",
        -69060 => "CLOUD_LINK_DOWN",
        -69061 => "ONVIF_SET_WRONG_TIME",
        -69062 => "CLOUD_NTP_NO_RESPONSE",
        -69063 => "CLOUD_GET_WRONG_TIME",
        -69064 => "SNTP_SRV_NO_RESPONSE",
        -69065 => "SNTP_GET_WRONG_TIME",
        -69076 => "LINK_UNCONNECTED",
        -69077 => "WIFI_SIGNAL_WEAK",
        -69078 => "LOCAL_NETWORK_POOR",
        -69079 => "CLOUD_NETWORK_POOR",
        -69080 => "INTER_NETWORK_POOR",
        -69081 => "DNS_TIMEOUT",
        -69082 => "DNS_ERROR",
        -69083 => "PING_NO_RESPONSE",
        -69084 => "DHCP_MULTI_SERVER",
        -69085 => "DHCP_ERROR",
        -69094 => "STREAM_SESSION_CLOSE",
        -69095 => "STREAM_BITRATE_EXCEPTION",
        -69096 => "STREAM_FULL",
        -69097 => "STREAM_NO_INTERNET",
        -72101 => "HARDWIRED_NOT_FOUND",
        -80101 => "ENCRYPT_NOT_SUPPORT",
        -90000 => "FFS_NONE_PWD",
        -90001 => "TSS_NONE_PWD",
        40108 => "TIMEOUT_ERROR",
        _ => "UNKNOWN",
    }
}
