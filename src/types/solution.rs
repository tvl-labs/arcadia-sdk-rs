use alloy::primitives::{Address, Signature, U256, keccak256};
use alloy::signers::Signer;
use alloy::sol_types::SolValue;
use serde::{Deserialize, Serialize};

use super::common::*;
use super::conversion::{RpcToSol, RpcType};
use super::intents::Intent;
use super::receipt::Receipt;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    pub intent_ids: Vec<B256>,
    pub intent_outputs: Vec<Intent>,
    pub receipt_outputs: Vec<Receipt>,
    pub spend_graph: Vec<MoveRecord>,
    pub fill_graph: Vec<FillRecord>,
}

impl Solution {
    pub async fn sign<S>(&self, signer: &S) -> SignedSolution
    where
        S: Signer,
    {
        let signature = signer
            .sign_hash(&keccak256(self.convert_to_sol_type().abi_encode()))
            .await
            .unwrap();
        SignedSolution {
            solution: self.clone(),
            signature,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedSolution {
    pub solution: Solution,
    pub signature: Signature,
}

impl SignedSolution {
    pub fn hash(&self) -> B256 {
        keccak256(self.solution.convert_to_sol_type().abi_encode())
    }

    pub fn recover_address(&self) -> Address {
        let hash = self.hash();
        self.signature.recover_address_from_prehash(&hash).unwrap()
    }
}

impl RpcType for Solution {}
impl RpcType for SignedSolution {}
impl RpcType for OutType {}
impl RpcType for OutputIdx {}
impl RpcType for MoveRecord {}
impl RpcType for FillRecord {}
