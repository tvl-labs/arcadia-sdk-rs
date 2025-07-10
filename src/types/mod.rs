use alloy::primitives::{B256, Signature};
pub mod common;

pub mod config;
pub mod events;
pub mod intents;
pub mod receipt;
pub mod solidity;

pub mod solution;
use intents::{Intent, SignedIntent};
use solution::Solution;

pub trait SolidityType {}

pub trait ToSol {
    type Sol: SolidityType;
    fn to_sol(&self) -> Self::Sol;
}

pub trait FromSol {
    type Sol: SolidityType;
    fn from_sol(sol: Self::Sol) -> Self;
}

pub trait Intentful {
    type Error: std::error::Error;
    fn to_intent(&self) -> Intent;
    fn calc_intent_id(&self) -> common::IntentId;
    fn validate(&self) -> bool;
    fn sign(&self) -> Result<SignedIntent, Self::Error>;
    fn check_solution(&self, solution: &Solution) -> Result<bool, Self::Error>;
}

#[cfg(test)]
mod tests {
    use alloy::{
        hex::FromHex,
        primitives::{B256, address},
    };

    use crate::types::intents::IntentBuilder;

    use super::{
        FromSol, SolidityType, ToSol,
        intents::{Intent, SignedIntent},
        solidity::eip712_intent_hash,
    };

    #[test]
    fn test_hash_intent() {
        let intent = IntentBuilder::new().build_with_defaults();
        let intent_hash = intent.intent_hash();
        let expected_hash =
            B256::from_hex("0x7ce4b31dc44f50cef069989d3be949a53208daa87e0f18097e960b2c8984b632")
                .unwrap();
        assert_eq!(intent_hash, expected_hash);

        let expected_eip712_hash =
            B256::from_hex("0xbad1abd0a5c686fffc08eab627fbe44c855afa94a04dbf516f1bde4c4c8e8705")
                .unwrap();
        let eip712_hash = eip712_intent_hash(
            &intent,
            address!("0x0000000000000000000000000000000000000000"),
        );
        assert_eq!(eip712_hash, expected_eip712_hash);
    }
}
