use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

use crate::types::medusa::intents::Intent;
use crate::types::medusa::intents::IntentId;
use crate::types::medusa::intents::IntentState;
use crate::types::medusa::refinement::RefinementStatus;
use crate::types::medusa::solution::SignedSolution;
use crate::types::medusa::solution::Solution;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsPayload {
    IntentRefinement(IntentId, RefinementStatus),
    GetSolutionsForIntent(IntentId),
    GetSolutionsForSolver(Address),
    AddSolver(Address),
    ProposeSolution(SignedSolution),
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
