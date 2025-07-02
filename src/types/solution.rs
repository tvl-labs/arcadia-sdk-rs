use crate::types::common::*;
use crate::types::{FromSol, SolidityType, ToSol, intents::Intent, receipt::Receipt};

use alloy::primitives::{Address, Signature, U256, keccak256};
use alloy::sol_types::SolValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    pub intent_ids: Vec<B256>,
    pub intent_outputs: Vec<Intent>,
    pub receipt_outputs: Vec<Receipt>,
    pub spend_graph: Vec<MoveRecord>,
    pub fill_graph: Vec<FillRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedSolution {
    pub solution: Solution,
    pub signature: Signature,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum OutType {
    Intent,
    Receipt,
}

impl From<OutType> for u8 {
    fn from(out_type: OutType) -> u8 {
        match out_type {
            OutType::Intent => 0,
            OutType::Receipt => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutputIdx {
    pub out_type: OutType,
    pub out_idx: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoveRecord {
    pub src_idx: u64,
    pub output_idx: OutputIdx,
    pub qty: U256,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillRecord {
    pub in_idx: u64,
    pub out_idx: u64,
    pub out_type: OutType,
}

impl ToSol for Solution {
    type Sol = crate::types::solidity::Solution;
    fn to_sol(&self) -> Self::Sol {
        crate::types::solidity::Solution {
            intentIds: self.intent_ids.clone(),
            intentOutputs: self.intent_outputs.iter().map(|i| i.to_sol()).collect(),
            receiptOutputs: self.receipt_outputs.iter().map(|r| r.to_sol()).collect(),
            spendGraph: self.spend_graph.iter().map(|m| m.to_sol()).collect(),
            fillGraph: self.fill_graph.iter().map(|f| f.to_sol()).collect(),
        }
    }
}

impl ToSol for MoveRecord {
    type Sol = crate::types::solidity::MoveRecord;
    fn to_sol(&self) -> Self::Sol {
        crate::types::solidity::MoveRecord {
            srcIdx: self.src_idx,
            outputIdx: self.output_idx.to_sol(),
            qty: self.qty,
        }
    }
}

impl ToSol for FillRecord {
    type Sol = crate::types::solidity::FillRecord;
    fn to_sol(&self) -> Self::Sol {
        crate::types::solidity::FillRecord {
            inIdx: self.in_idx,
            outIdx: self.out_idx,
            outType: self.out_type.to_sol(),
        }
    }
}

impl ToSol for OutputIdx {
    type Sol = crate::types::solidity::OutputIdx;
    fn to_sol(&self) -> Self::Sol {
        crate::types::solidity::OutputIdx {
            outType: self.out_type.to_sol(),
            outIdx: self.out_idx,
        }
    }
}

impl ToSol for OutType {
    type Sol = crate::types::solidity::OutType;
    fn to_sol(&self) -> Self::Sol {
        match self {
            OutType::Intent => crate::types::solidity::OutType::Intent,
            OutType::Receipt => crate::types::solidity::OutType::Receipt,
        }
    }
}

impl SignedSolution {
    pub fn hash(&self) -> B256 {
        keccak256(self.solution.to_sol().abi_encode())
    }

    pub fn recover_address(&self) -> Address {
        let hash = self.hash();
        self.signature.recover_address_from_prehash(&hash).unwrap()
    }
}
