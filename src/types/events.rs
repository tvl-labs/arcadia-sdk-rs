use super::common::*;
use super::solidity::{
    AssetReserveDeposit as SolAssetReserveDeposit, MTokenWithdrawal as SolMTokenWithdrawal,
    XChainEvent as SolXChainEvent,
};
use super::{FromSol, ToSol};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct XChainEvent {
    pub(crate) publisher: Address,
    pub event_hash: B256,
    pub origin_chain_id: ChainId,
    pub event_nonce: U256,
    pub event_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetReserveDeposit {
    pub token: Address,
    pub amount: U256,
    pub depositor: Address,
}

impl ToSol for XChainEvent {
    type Sol = crate::types::solidity::XChainEvent;
    fn to_sol(&self) -> Self::Sol {
        SolXChainEvent {
            publisher: self.publisher,
            eventHash: self.event_hash,
            originChainId: U256::from(self.origin_chain_id),
            eventNonce: self.event_nonce,
            eventData: self.event_data.clone().into(),
        }
    }
}

impl FromSol for XChainEvent {
    type Sol = crate::types::solidity::XChainEvent;
    fn from_sol(sol: Self::Sol) -> Self {
        XChainEvent {
            publisher: sol.publisher,
            event_hash: sol.eventHash,
            origin_chain_id: sol.originChainId.to::<u64>(),
            event_nonce: sol.eventNonce,
            event_data: sol.eventData.to_vec(),
        }
    }
}

impl ToSol for AssetReserveDeposit {
    type Sol = crate::types::solidity::AssetReserveDeposit;
    fn to_sol(&self) -> Self::Sol {
        SolAssetReserveDeposit {
            token: self.token,
            amount: self.amount,
            depositor: self.depositor,
        }
    }
}

impl FromSol for AssetReserveDeposit {
    type Sol = crate::types::solidity::AssetReserveDeposit;
    fn from_sol(sol: Self::Sol) -> Self {
        AssetReserveDeposit {
            token: sol.token,
            amount: sol.amount,
            depositor: sol.depositor,
        }
    }
}
