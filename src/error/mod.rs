use alloy::{primitives::U256, signers::Error as SignerError};
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
    ContractError(#[from] alloy::contract::Error),
    #[error(transparent)]
    PendingTransactionError(#[from] alloy::providers::PendingTransactionError),
    #[error("Insufficient allowance {0}, needed {1}")]
    InsufficientAllowance(U256, U256),
}
