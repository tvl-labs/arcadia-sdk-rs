use alloy::primitives::{B256, Signature};
pub mod common;

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
