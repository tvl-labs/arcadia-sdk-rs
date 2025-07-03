use crate::{
    error::Error,
    types::intents::{Intent, SignedIntent},
};
use alloy::signers::{Signature, Signer};
use jsonrpsee::{
    core::client::ClientT,
    http_client::{HttpClient, HttpClientBuilder},
    rpc_params,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fmt::Display, time::Duration};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout: Duration,
    pub max_concurrent_requests: usize,
    pub connection_pool_size: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_concurrent_requests: 100,
            connection_pool_size: 10,
        }
    }
}

pub struct JsonRpcClient {
    http_client: Option<HttpClient>,
    config: ClientConfig,
}

impl JsonRpcClient {
    pub async fn new_http(url: &str) -> Result<Self, Error> {
        Self::new_http_with_config(url, ClientConfig::default()).await
    }

    pub async fn new_http_with_config(url: &str, config: ClientConfig) -> Result<Self, Error> {
        let http_client = HttpClientBuilder::default()
            .request_timeout(config.timeout)
            .max_concurrent_requests(config.max_concurrent_requests)
            .build(url)?;

        Ok(Self {
            http_client: Some(http_client),
            config,
        })
    }

    pub async fn request<R, T>(&self, method: &str, params: R) -> Result<T, Error>
    where
        R: Serialize + Send + Sync,
        T: DeserializeOwned,
    {
        match &self.http_client {
            Some(http_client) => {
                let params = rpc_params![params];
                let response: T = http_client.request(method, params).await?;
                Ok(response)
            }
            _ => Err(Error::ClientError("No transport configured".to_string())),
        }
    }

    pub async fn request_no_params<T>(&self, method: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match &self.http_client {
            Some(http_client) => {
                let response: T = http_client.request(method, rpc_params![]).await?;
                Ok(response)
            }

            _ => Err(Error::ClientError("No transport configured".to_string())),
        }
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn is_http(&self) -> bool {
        self.http_client.is_some()
    }

    // Will implement ws as well, especially so solvers can use the SDK with medusa's ws solver API.
    pub fn is_websocket(&self) -> bool {
        false
    }

    pub fn transport_type(&self) -> &'static str {
        if self.is_http() {
            "HTTP"
        } else if self.is_websocket() {
            "WebSocket"
        } else {
            "None"
        }
    }
}

pub struct MedusaClient<S: Signer + Send + Sync> {
    rpc_client: JsonRpcClient,
    signer: S,
}

impl<S: Signer + Send + Sync> MedusaClient<S> {
    pub async fn new(url: &str, signer: S) -> Result<Self, Error> {
        let rpc_client = JsonRpcClient::new_http(url).await?;
        Ok(Self { rpc_client, signer })
    }

    pub fn rpc_client(&self) -> &JsonRpcClient {
        &self.rpc_client
    }

    pub fn rpc_client_mut(&mut self) -> &mut JsonRpcClient {
        &mut self.rpc_client
    }

    pub fn propose_intent(&self, signed_intent: SignedIntent) -> Result<(), Error> {
        todo!()
    }

    pub async fn sign_payload<T: Serialize>(&self, payload: &T) -> Result<Signature, Error> {
        let payload_json = serde_json::to_string(payload).map_err(Error::SerdeJsonError)?;
        let signature = self
            .signer
            .sign_message(payload_json.as_bytes())
            .await
            .map_err(Error::SignerError)?;
        Ok(signature)
    }

    // TODO: Probably doesn't work; needs to convert to sol type first.
    // Also, needs to utilize eip712 signing.
    pub async fn sign_intent(&self, intent: Intent) -> Result<SignedIntent, Error> {
        let signature = self.sign_payload(&intent).await?;
        Ok(SignedIntent { intent, signature })
    }
}
