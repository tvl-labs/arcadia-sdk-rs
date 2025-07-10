use alloy::primitives::{Address, ChainId};
use serde::{Deserialize, Serialize};

use crate::types::config::registry::{
    CrossChainSystem, CrossChainSystemContracts, arcadia_registry::NativeCurrency,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpokeRegistry {
    pub name: String,
    pub chain_id: ChainId,
    pub short_name: String,
    pub native_currency: NativeCurrency,
    pub cross_chain_systems: Vec<CrossChainSystem>,
    pub arcadia_contracts: SpokeArcadiaContracts,
    pub cross_chain_system_contracts: CrossChainSystemContracts,
    pub rpc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpokeArcadiaContracts {
    pub event_prover: Address,
    pub event_publisher: Address,
    pub event_verifier: Address,
    pub event_handler: Address,
}
