use crate::types::common::*;
use alloy::primitives::Signature;

pub struct Solution {}

pub struct SignedSolution {
    solution: Solution,
    signature: Signature,
}
