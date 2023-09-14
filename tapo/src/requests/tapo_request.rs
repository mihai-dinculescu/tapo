use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::requests::{
    ControlChildParams, GetEnergyDataParams, GetTriggerLogsParams, HandshakeParams, LightingEffect,
    LoginDeviceParams, MultipleRequestParams, SecurePassthroughParams,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method")]
pub(crate) enum TapoRequest {
    #[serde(rename = "component_nego")]
    ComponentNegotiation(TapoParams<EmptyParams>),
    Handshake(TapoParams<HandshakeParams>),
    LoginDevice(TapoParams<LoginDeviceParams>),
    #[serde(rename = "securePassthrough")]
    SecurePassthrough(TapoParams<SecurePassthroughParams>),
    SetDeviceInfo(Box<TapoParams<serde_json::Value>>),
    SetLightingEffect(Box<TapoParams<LightingEffect>>),
    GetDeviceInfo(TapoParams<EmptyParams>),
    GetDeviceUsage(TapoParams<EmptyParams>),
    GetEnergyUsage(TapoParams<EmptyParams>),
    GetEnergyData(TapoParams<GetEnergyDataParams>),
    GetCurrentPower(TapoParams<EmptyParams>),
    GetChildDeviceList(TapoParams<EmptyParams>),
    GetChildDeviceComponentList(TapoParams<EmptyParams>),
    ControlChild(Box<TapoParams<ControlChildParams>>),
    // Child requests
    #[serde(rename = "multipleRequest")]
    MultipleRequest(Box<TapoParams<MultipleRequestParams>>),
    GetTriggerLogs(Box<TapoParams<GetTriggerLogsParams>>),
    #[serde(rename = "get_temp_humidity_records")]
    GetTemperatureHumidityRecords(Box<TapoParams<EmptyParams>>),
}

#[derive(Debug, Serialize)]
pub(crate) struct EmptyParams;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TapoParams<T> {
    params: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_time_milis: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "terminalUUID")]
    terminal_uuid: Option<String>,
}

impl<T> TapoParams<T> {
    pub fn new(params: T) -> Self {
        Self {
            params,
            request_time_milis: None,
            terminal_uuid: None,
        }
    }

    pub fn set_request_time_mils(mut self) -> anyhow::Result<Self> {
        self.request_time_milis
            .replace(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64);
        Ok(self)
    }

    pub fn set_terminal_uuid(mut self, terminal_uuid: &str) -> Self {
        self.terminal_uuid.replace(terminal_uuid.to_string());
        self
    }
}
