use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Token {
    pub spoke_address: Address,
    pub mtoken_address: Address,
    pub spoke_chain_id: u64,
    pub symbol: String,
    pub decimals: u8,
}
