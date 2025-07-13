use crate::{
    error::Error,
    types::{
        config::registry::arcadia_registry::{ArcadiaChainRegistry, MTokenRegistryEntry},
        intents::Intent,
    },
};
use alloy::primitives::{Address, U256};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MTokensUseRecord {
    pub consumed_mtoken: MTokenRegistryEntry,
    pub outcome_mtokens: Vec<(U256, MTokenRegistryEntry)>,
}
pub fn get_intent_mtokens_use_record(
    intent: &Intent,
    registry: &ArcadiaChainRegistry,
) -> MTokensUseRecord {
    let mut outcome_mtokens = Vec::new();
    let mut consumed_mtoken = None;

    consumed_mtoken = registry
        .get_mtoken_entry_by_address(intent.src_m_token)
        .cloned();

    for (i, mtoken) in intent.outcome.m_tokens.iter().enumerate() {
        let m_amount = intent.outcome.m_amounts[i];
        let mtoken_entry = registry
            .get_mtoken_entry_by_address(*mtoken)
            .cloned()
            .unwrap();
        outcome_mtokens.push((m_amount, mtoken_entry));
    }

    MTokensUseRecord {
        consumed_mtoken: consumed_mtoken.unwrap(),
        outcome_mtokens,
    }
}
