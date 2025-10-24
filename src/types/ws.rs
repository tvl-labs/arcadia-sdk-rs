use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

use crate::types::intents::IntentId;
use crate::types::intents::IntentState;
use crate::types::intents::{Intent, SignedIntent};
use crate::types::refinement::RefinementStatus;
use crate::types::solution::SignedSolution;
use crate::types::solution::Solution;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComplementaryWithdrawal(pub bool);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsPayload {
    IntentRefinement(IntentId, RefinementStatus),
    GetSolutionsForIntent(IntentId),
    GetSolutionsForSolver(Address),
    AddSolver(Address),
    ProposeSolution(SignedSolution, Vec<SignedIntent>, ComplementaryWithdrawal),
    RequestOpenIntents,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsBroadcastMessage {
    IntentStatusUpdated(IntentId, IntentState),
    RefinementNeededForIntent(Intent),
    NewIntent(Intent),
    IntentsSolved(Vec<IntentId>, Address),
    Solutions(u128, Vec<Solution>),
    ExistingOpenIntents(Vec<Intent>),
    SolutionRejected(SignedSolution),
}
