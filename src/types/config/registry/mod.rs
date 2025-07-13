use crate::error::Error;
use alloy::primitives::Address;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub mod arcadia_registry;
pub mod spoke_registry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CrossChainSystem {
    #[serde(rename = "hyperlane")]
    Hyperlane,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CrossChainSystemContracts {
    pub hyperlane: HyperlaneContracts,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HyperlaneContracts {
    pub mailbox: Address,
    pub igp: Address,
    pub gas_amount_oracle: Address,
}

pub fn load_registry<C: DeserializeOwned>(file_path: &str) -> Result<C, Error> {
    let file = std::fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&file)?)
}
