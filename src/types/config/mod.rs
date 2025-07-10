use alloy::primitives::ChainId;
use serde::{Deserialize, Serialize};

pub mod registry;
#[derive(Debug, Serialize, Deserialize)]
pub struct AipSupportedChain {
    pub name: String,
    pub chain_id: ChainId,
}
