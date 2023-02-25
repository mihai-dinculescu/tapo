use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::requests::{
    GetDeviceInfoParams, GetDeviceUsageParams, GetEnergyDataParams, GetEnergyUsageParams,
    HandshakeParams, LoginDeviceParams, SecurePassthroughParams,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method")]
pub(crate) enum TapoRequest {
    Handshake(TapoParams<HandshakeParams>),
    LoginDevice(TapoParams<LoginDeviceParams>),
    #[serde(rename = "securePassthrough")]
    SecurePassthrough(TapoParams<SecurePassthroughParams>),
    SetDeviceInfo(TapoParams<serde_json::Value>),
    GetDeviceInfo(TapoParams<GetDeviceInfoParams>),
    GetDeviceUsage(TapoParams<GetDeviceUsageParams>),
    GetEnergyUsage(TapoParams<GetEnergyUsageParams>),
    GetEnergyData(TapoParams<GetEnergyDataParams>),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TapoParams<T> {
    params: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_time_mils: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "terminalUUID")]
    terminal_uuid: Option<String>,
}

impl<T> TapoParams<T> {
    pub fn new(params: T) -> Self {
        Self {
            params,
            request_time_mils: None,
            terminal_uuid: None,
        }
    }

    pub fn set_request_time_mils(mut self) -> anyhow::Result<Self> {
        self.request_time_mils
            .replace(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64);
        Ok(self)
    }

    pub fn set_terminal_uuid(mut self, terminal_uuid: &str) -> Self {
        self.terminal_uuid.replace(terminal_uuid.to_string());
        self
    }
}
