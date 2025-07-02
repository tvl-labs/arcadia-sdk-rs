use crate::types::{FromSol, ToSol, solidity::Receipt as SolReceipt};
use alloy::primitives::{Address, B256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub m_token: Address,
    pub m_token_amount: U256,
    pub owner: Address,
    pub intent_hash: B256,
}

impl ToSol for Receipt {
    type Sol = crate::types::solidity::Receipt;
    fn to_sol(&self) -> Self::Sol {
        SolReceipt {
            mToken: self.m_token,
            mTokenAmount: self.m_token_amount,
            owner: self.owner,
            intentHash: self.intent_hash,
        }
    }
}

impl FromSol for Receipt {
    type Sol = crate::types::solidity::Receipt;
    fn from_sol(sol: Self::Sol) -> Self {
        Receipt {
            m_token: sol.mToken,
            m_token_amount: sol.mTokenAmount,
            owner: sol.owner,
            intent_hash: sol.intentHash,
        }
    }
}
