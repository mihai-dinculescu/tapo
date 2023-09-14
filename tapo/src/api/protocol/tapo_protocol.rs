use std::fmt;

use async_trait::async_trait;
use isahc::HttpClient;
use serde::de::DeserializeOwned;

use crate::requests::TapoRequest;
use crate::responses::TapoResponseExt;
use crate::Error;

use super::{
    discovery_protocol::DiscoveryProtocol, klap_protocol::KlapProtocol,
    passthrough_protocol::PassthroughProtocol,
};

#[derive(Debug, Clone)]
pub(crate) struct TapoProtocol {
    protocol: TapoProtocolType,
}

#[async_trait]
pub(crate) trait TapoProtocolExt {
    async fn login(&mut self, url: String) -> Result<(), Error>;
    async fn refresh_session(&mut self) -> Result<(), Error>;
    async fn execute_request<R>(
        &self,
        request: TapoRequest,
        with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt;
    fn clone_as_discovery(&self) -> DiscoveryProtocol;
}

#[derive(Debug)]
pub(crate) enum TapoProtocolType {
    Discovery(DiscoveryProtocol),
    Passthrough(PassthroughProtocol),
    Klap(KlapProtocol),
}

impl Clone for TapoProtocolType {
    fn clone(&self) -> Self {
        match self {
            Self::Discovery(protocol) => Self::Discovery(protocol.clone()),
            Self::Passthrough(protocol) => Self::Discovery(protocol.clone_as_discovery()),
            Self::Klap(protocol) => Self::Discovery(protocol.clone_as_discovery()),
        }
    }
}

#[async_trait]
impl TapoProtocolExt for TapoProtocol {
    async fn login(&mut self, url: String) -> Result<(), Error> {
        if let TapoProtocolType::Discovery(protocol) = &mut self.protocol {
            self.protocol = protocol.discover(&url).await?;
        }

        match &mut self.protocol {
            TapoProtocolType::Passthrough(protocol) => protocol.login(url).await,
            TapoProtocolType::Klap(protocol) => protocol.login(url).await,
            _ => Err(anyhow::anyhow!("The protocol discovery should have happened already").into()),
        }
    }

    async fn refresh_session(&mut self) -> Result<(), Error> {
        match &mut self.protocol {
            TapoProtocolType::Passthrough(protocol) => protocol.refresh_session().await,
            TapoProtocolType::Klap(protocol) => protocol.refresh_session().await,
            _ => Err(anyhow::anyhow!("The protocol discovery should have happened already").into()),
        }
    }

    async fn execute_request<R>(
        &self,
        request: TapoRequest,
        with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        match &self.protocol {
            TapoProtocolType::Passthrough(protocol) => {
                protocol.execute_request(request, with_token).await
            }
            TapoProtocolType::Klap(protocol) => protocol.execute_request(request, with_token).await,
            _ => Err(anyhow::anyhow!("The protocol discovery should have happened already").into()),
        }
    }

    fn clone_as_discovery(&self) -> DiscoveryProtocol {
        match &self.protocol {
            TapoProtocolType::Discovery(protocol) => protocol.clone(),
            TapoProtocolType::Passthrough(protocol) => protocol.clone_as_discovery(),
            TapoProtocolType::Klap(protocol) => protocol.clone_as_discovery(),
        }
    }
}

impl TapoProtocol {
    pub fn new(client: HttpClient, username: String, password: String) -> Self {
        Self {
            protocol: TapoProtocolType::Discovery(DiscoveryProtocol::new(
                client, username, password,
            )),
        }
    }
}
