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

pub struct IntentBuilder {
    pub author: Option<Address>,
    pub ttl: Option<U256>,
    pub nonce: Option<U256>,
    pub src_m_token: Option<Address>,
    pub src_amount: Option<U256>,
    pub outcome: Option<Outcome>,
}

impl IntentBuilder {
    pub fn new() -> Self {
        Self {
            author: None,
            ttl: None,
            nonce: None,
            src_m_token: None,
            src_amount: None,
            outcome: None,
        }
    }

    pub fn author(mut self, author: Address) -> Self {
        self.author = Some(author);
        self
    }

    pub fn ttl(mut self, ttl: U256) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn nonce(mut self, nonce: U256) -> Self {
        self.nonce = Some(nonce);
        self
    }

    pub fn src_m_token(mut self, src_m_token: Address) -> Self {
        self.src_m_token = Some(src_m_token);
        self
    }

    pub fn src_amount(mut self, src_amount: U256) -> Self {
        self.src_amount = Some(src_amount);
        self
    }

    pub fn outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    pub fn outcome_with_token(mut self, token: Address, amount: U256) -> Self {
        let prevOutcome = &mut self.outcome;
        if let Some(outcome) = prevOutcome {
            outcome.m_tokens.push(token);
            outcome.m_amounts.push(amount);
        } else {
            self.outcome = Some(Outcome {
                m_tokens: vec![token],
                m_amounts: vec![amount],
                outcome_asset_structure: OutcomeAssetStructure::AnySingle,
                fill_structure: FillStructure::Exact,
            });
        }
        self
    }

    pub fn build_with_defaults(self) -> Intent {
        Intent {
            author: self.author.unwrap_or(Address::ZERO),
            ttl: self.ttl.unwrap_or(U256::from(0_u64)),
            nonce: self.nonce.unwrap_or(U256::from(0)),
            src_m_token: self.src_m_token.unwrap_or(Address::ZERO),
            src_amount: self.src_amount.unwrap_or(U256::from(0)),
            outcome: self.outcome.unwrap_or(Outcome {
                m_tokens: vec![],
                m_amounts: vec![],
                outcome_asset_structure: OutcomeAssetStructure::AnySingle,
                fill_structure: FillStructure::Exact,
            }),
        }
    }

    pub fn build(self) -> Intent {
        Intent {
            author: self.author.unwrap_or(Address::ZERO),
            ttl: self.ttl.unwrap_or(U256::from(0_u64)),
            nonce: self.nonce.unwrap_or(U256::from(1_u64)),
            src_m_token: self.src_m_token.unwrap_or(Address::ZERO),
            src_amount: self.src_amount.unwrap_or(U256::from(0)),
            outcome: self.outcome.unwrap_or(Outcome {
                m_tokens: vec![],
                m_amounts: vec![],
                outcome_asset_structure: OutcomeAssetStructure::AnySingle,
                fill_structure: FillStructure::Exact,
            }),
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

#[cfg(test)]
mod tests {
    use alloy::hex::FromHex;

    use super::*;

    #[test]
    fn test_intent_builder() {
        let intent = IntentBuilder::new().build_with_defaults();
        assert_eq!(intent.author, Address::ZERO);
        assert_eq!(intent.ttl, U256::from(0_u64));
        assert_eq!(intent.nonce, U256::from(0));
        assert_eq!(intent.src_m_token, Address::ZERO);
        assert_eq!(intent.src_amount, U256::from(0));
        assert_eq!(
            intent.outcome,
            Outcome {
                m_tokens: vec![],
                m_amounts: vec![],
                outcome_asset_structure: OutcomeAssetStructure::AnySingle,
                fill_structure: FillStructure::Exact,
            }
        );

        let default_intent_hash =
            B256::from_hex("0x7ce4b31dc44f50cef069989d3be949a53208daa87e0f18097e960b2c8984b632")
                .unwrap();
        assert_eq!(intent.intent_hash(), default_intent_hash);
    }
}
