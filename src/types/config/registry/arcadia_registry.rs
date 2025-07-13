use crate::types::config::registry::{CrossChainSystem, CrossChainSystemContracts};
use alloy::primitives::{Address, ChainId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArcadiaChainRegistry {
    pub name: String,
    pub chain_id: ChainId,
    pub short_name: String,
    pub native_currency: NativeCurrency,
    #[serde(default)]
    pub rpc: Vec<String>,
    #[serde(default)]
    pub faucets: Vec<String>,
    #[serde(default)]
    pub explorers: Vec<String>,
    #[serde(default)]
    pub core_contracts: CoreContracts,
    #[serde(default)]
    pub cross_chain_systems: Vec<CrossChainSystem>,
    #[serde(default)]
    pub cross_chain_system_contracts: CrossChainSystemContracts,
    #[serde(default)]
    pub is_testnet: bool,
    pub mtokens: HashMap<String, MTokenRegistryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CoreContracts {
    pub intent_book: String,
    pub m_token_manager: String,
    pub event_publisher: String,
    pub event_verifier: String,
    pub event_handler: String,
    pub event_provers: Vec<EventProverRegistryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventProverRegistryEntry {
    pub to: ChainId,
    pub prover: Address,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MTokenRegistryEntry {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub spoke_chain: MTokenSpokeChainDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MTokenSpokeChainDetails {
    pub name: String,
    pub registry_name: String,
    pub spoke_token_address: Address,
    pub spoke_token_name: String,
    pub spoke_token_symbol: String,
    pub spoke_token_decimals: u8,
    pub chain_id: ChainId,
}

impl ArcadiaChainRegistry {
    pub fn get_mtoken_entry_by_address(&self, address: Address) -> Option<&MTokenRegistryEntry> {
        self.mtokens.values().find(|entry| entry.address == address)
    }
}
