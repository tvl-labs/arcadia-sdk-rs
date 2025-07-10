use alloy::signers::Error as SignerError;
use jsonrpsee::core::client::Error as JsonRpcClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SignerError(#[from] SignerError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonRpcClientError(#[from] JsonRpcClientError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Client error: {0}")]
    ClientError(String),
    #[error("Transport error: {0}")]
    TransportError(String),
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
}
