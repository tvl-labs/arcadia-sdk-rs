use crate::types::config::registry::{CrossChainSystem, CrossChainSystemContracts};
use alloy::primitives::{Address, ChainId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArcadiaChainRegistry {
    pub name: String,
    pub chain_id: ChainId,
    pub short_name: String,
    pub native_currency: NativeCurrency,
    pub rpc: Vec<String>,
    pub faucets: Vec<String>,
    pub explorers: Vec<String>,
    pub core_contracts: CoreContracts,
    pub cross_chain_systems: Vec<CrossChainSystem>,
    pub cross_chain_system_contracts: CrossChainSystemContracts,
    pub is_testnet: bool,
    pub is_mainnet: bool,
    pub m_tokens: MTokenRegistryEntry,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MTokenRegistryEntry {
    pub m_token_address: String,
    pub m_token_name: String,
    pub m_token_symbol: String,
    pub m_token_decimals: u8,
}
