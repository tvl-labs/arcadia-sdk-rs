use alloy::primitives::{B256, Signature};
use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use alloy::primitives::{Address, U256, keccak256};
use alloy::sol_types::SolValue;

use crate::types::{FromSol, SolidityType, ToSol};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename = "OutcomeAssetStructure")]
pub enum OutcomeAssetStructure {
    AnySingle,
    Any,
    All,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename = "FillStructure")]
pub enum FillStructure {
    Exact,
    Minimum,
    PercentageFilled,
    ConcreteRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Outcome {
    pub m_tokens: Vec<Address>,
    pub m_amounts: Vec<U256>,
    pub outcome_asset_structure: OutcomeAssetStructure,
    pub fill_structure: FillStructure,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Intent {
    pub author: Address,
    pub ttl: U256,
    pub nonce: U256,
    pub src_m_token: Address,
    pub src_amount: U256,
    pub outcome: Outcome,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SignedIntent {
    pub intent: Intent,
    pub signature: Signature,
}

impl SignedIntent {
    pub fn intent_id(&self) -> B256 {
        self.intent.intent_id()
    }
}
impl From<OutcomeAssetStructure> for u8 {
    fn from(outcome_asset_structure: OutcomeAssetStructure) -> u8 {
        match outcome_asset_structure {
            OutcomeAssetStructure::AnySingle => 0,
            OutcomeAssetStructure::Any => 1,
            OutcomeAssetStructure::All => 2,
        }
    }
}

impl From<FillStructure> for u8 {
    fn from(fill_structure: FillStructure) -> u8 {
        match fill_structure {
            FillStructure::Exact => 0,
            FillStructure::Minimum => 1,
            FillStructure::PercentageFilled => 2,
            FillStructure::ConcreteRange => 3,
        }
    }
}

impl Intent {
    pub fn simple_swap(
        author: Address,
        ttl: U256,
        nonce: Option<U256>,
        src_m_token: Address,
        src_amount: impl Into<U256> + Copy,
        output_m_token: Address,
        output_amount: U256,
    ) -> Self {
        let nonce = nonce.unwrap_or(U256::from(1_u64));
        let outcome = Outcome {
            m_tokens: vec![output_m_token],
            m_amounts: vec![output_amount],
            outcome_asset_structure: OutcomeAssetStructure::AnySingle,
            fill_structure: FillStructure::Exact,
        };
        Intent {
            author,
            ttl,
            nonce,
            src_m_token,
            src_amount: src_amount.into(),
            outcome,
        }
    }

    pub fn intent_hash(&self) -> B256 {
        let sol_intent = self.to_sol(); //
        keccak256(sol_intent.abi_encode())
    }

    pub fn intent_id(&self) -> B256 {
        self.intent_hash()
    }
}

impl ToSol for Intent {
    type Sol = crate::types::solidity::Intent;
    fn to_sol(&self) -> Self::Sol {
        crate::types::solidity::Intent {
            author: self.author,
            ttl: self.ttl,
            nonce: self.nonce,
            srcMToken: self.src_m_token,
            srcAmount: self.src_amount,
            outcome: self.outcome.to_sol(),
        }
    }
}

impl ToSol for FillStructure {
    type Sol = crate::types::solidity::FillStructure;
    fn to_sol(&self) -> Self::Sol {
        match self {
            FillStructure::Exact => crate::types::solidity::FillStructure::Exactly,
            FillStructure::Minimum => crate::types::solidity::FillStructure::Minimum,
            FillStructure::PercentageFilled => crate::types::solidity::FillStructure::PctFilled,
            FillStructure::ConcreteRange => crate::types::solidity::FillStructure::ConcreteRange,
        }
    }
}

impl ToSol for OutcomeAssetStructure {
    type Sol = crate::types::solidity::OutcomeAssetStructure;
    fn to_sol(&self) -> Self::Sol {
        match self {
            OutcomeAssetStructure::AnySingle => {
                crate::types::solidity::OutcomeAssetStructure::AnySingle
            }
            OutcomeAssetStructure::Any => crate::types::solidity::OutcomeAssetStructure::Any,
            OutcomeAssetStructure::All => crate::types::solidity::OutcomeAssetStructure::All,
        }
    }
}

impl ToSol for Outcome {
    type Sol = crate::types::solidity::Outcome;
    fn to_sol(&self) -> Self::Sol {
        crate::types::solidity::Outcome {
            mTokens: self.m_tokens.clone(),
            mAmounts: self.m_amounts.clone(),
            outcomeAssetStructure: self.outcome_asset_structure.to_sol(),
            fillStructure: self.fill_structure.to_sol(),
        }
    }
}

impl FromSol for Intent {
    type Sol = crate::types::solidity::Intent;
    fn from_sol(sol: Self::Sol) -> Self {
        Intent {
            author: sol.author,
            ttl: sol.ttl,
            nonce: sol.nonce,
            src_m_token: sol.srcMToken,
            src_amount: sol.srcAmount,
            outcome: Outcome::from_sol(sol.outcome),
        }
    }
}

impl FromSol for Outcome {
    type Sol = crate::types::solidity::Outcome;
    fn from_sol(sol: Self::Sol) -> Self {
        Outcome {
            m_tokens: sol.mTokens,
            m_amounts: sol.mAmounts,
            outcome_asset_structure: OutcomeAssetStructure::from_sol(sol.outcomeAssetStructure),
            fill_structure: FillStructure::from_sol(sol.fillStructure),
        }
    }
}

impl FromSol for OutcomeAssetStructure {
    type Sol = crate::types::solidity::OutcomeAssetStructure;
    fn from_sol(sol: Self::Sol) -> Self {
        match sol {
            crate::types::solidity::OutcomeAssetStructure::AnySingle => {
                OutcomeAssetStructure::AnySingle
            }
            crate::types::solidity::OutcomeAssetStructure::Any => OutcomeAssetStructure::Any,
            crate::types::solidity::OutcomeAssetStructure::All => OutcomeAssetStructure::All,
            _ => panic!("Invalid OutcomeAssetStructure"),
        }
    }
}

impl FromSol for FillStructure {
    type Sol = crate::types::solidity::FillStructure;
    fn from_sol(sol: Self::Sol) -> Self {
        match sol {
            crate::types::solidity::FillStructure::Exactly => FillStructure::Exact,
            crate::types::solidity::FillStructure::Minimum => FillStructure::Minimum,
            crate::types::solidity::FillStructure::PctFilled => FillStructure::PercentageFilled,
            crate::types::solidity::FillStructure::ConcreteRange => FillStructure::ConcreteRange,
            _ => panic!("Invalid FillStructure"),
        }
    }
}
