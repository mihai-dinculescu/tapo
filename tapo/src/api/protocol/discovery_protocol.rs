use log::{debug, warn};
use reqwest::Client;

use crate::api::protocol::klap_protocol::KlapProtocol;
use crate::requests::{EmptyParams, TapoParams, TapoRequest};
use crate::responses::{validate_response, TapoResponse};
use crate::{Error, TapoResponseError};

use super::{passthrough_protocol::PassthroughProtocol, TapoProtocolType};

#[derive(Debug, Clone)]
pub(crate) struct DiscoveryProtocol {
    client: Client,
}

impl DiscoveryProtocol {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn discover(&mut self, url: &str) -> Result<TapoProtocolType, Error> {
        debug!("Testing the Passthrough protocol...");
        if self.is_passthrough_supported(url).await? {
            debug!("Supported. Setting up the Passthrough protocol...");
            Ok(TapoProtocolType::Passthrough(PassthroughProtocol::new(
                self.client.clone(),
            )?))
        } else {
            debug!("Not supported. Setting up the Klap protocol...");
            Ok(TapoProtocolType::Klap(KlapProtocol::new(
                self.client.clone(),
            )))
        }
    }

    async fn is_passthrough_supported(&self, url: &str) -> Result<bool, Error> {
        match self.test_passthrough(url).await {
            Err(Error::Tapo(TapoResponseError::Unknown(code))) => Ok(code != 1003),
            Err(err) => {
                warn!("Passthrough protocol test error: {err:?}");
                Err(err)
            }
            Ok(_) => Ok(true),
        }
    }

    async fn test_passthrough(&self, url: &str) -> Result<(), Error> {
        let request = TapoRequest::ComponentNegotiation(TapoParams::new(EmptyParams));
        let request_string = serde_json::to_string(&request)?;
        debug!("Component negotiation request: {request_string}");

        let response = self
            .client
            .post(url)
            .body(request_string)
            .send()
            .await?
            .json::<TapoResponse<serde_json::Value>>()
            .await?;

        debug!("Device responded with: {response:?}");

        validate_response(&response)?;

        Ok(())
    }
}
