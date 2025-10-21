pub use jsonrpsee::core::client::async_client::Client;

use alloy::dyn_abi::TypedData;
use alloy::primitives::{Address, B256, Bytes, U256};
use anyhow::Result;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::proc_macros::rpc;

use crate::types::intents::{Intent, IntentHistory, IntentId, IntentState, SignedIntent};
use crate::types::refinement::RefinementStatus;
use crate::types::rpc_payloads::{
    SignedPayloadAddress, SignedPayloadIntentId, SignedVaultDepositPayload,
    SignedVaultWithdrawalPayload, SignedWithdrawalPayload,
};
use crate::types::sol_types::FastWithdrawalPermit;
use crate::types::solution::SignedSolution;

#[rpc(client)]
pub trait MedusaRpc {
    #[method(name = "getDepositorVaultShares")]
    async fn get_depositor_vault_shares(
        &self,
        teller_address: Address,
        depositor_address: Address,
    ) -> RpcResult<U256>;

    #[method(name = "getVaultTotalAssetValue")]
    async fn get_vault_total_asset_value(&self, teller_address: Address) -> RpcResult<U256>;

    #[method(name = "getVaultTotalShares")]
    async fn get_vault_total_shares(&self, teller_address: Address) -> RpcResult<U256>;

    #[method(name = "previewDepositToVault")]
    async fn preview_deposit_to_vault(
        &self,
        teller_address: Address,
        asset: Address,
        amount: U256,
    ) -> RpcResult<U256>;

    #[method(name = "depositToVault")]
    async fn deposit_to_vault(&self, payload: SignedVaultDepositPayload) -> RpcResult<B256>;

    #[method(name = "previewMaximumWithdrawFromVault")]
    async fn preview_maximum_withdraw_from_vault(
        &self,
        teller_address: Address,
        asset: Address,
        shares: U256,
        fee_percentage: u16,
    ) -> RpcResult<U256>;

    #[method(name = "withdrawFromVault")]
    async fn withdraw_from_vault(&self, payload: SignedVaultWithdrawalPayload) -> RpcResult<B256>;

    #[method(name = "computeIntentId")]
    fn compute_intent_id(&self, intent: Intent) -> RpcResult<IntentId>;

    #[method(name = "getMtokenBalanceByAuthor")]
    async fn get_mtoken_balance_by_author(
        &self,
        user: Address,
        mtoken_address: Address,
    ) -> RpcResult<U256>;

    #[method(name = "getSolution")]
    async fn get_solution_for_intent(&self, intent_id: B256) -> RpcResult<Option<SignedSolution>>;

    #[method(name = "getConnectedSolvers")]
    async fn get_connected_solvers(&self) -> RpcResult<Vec<Address>>;

    #[method(name = "getIntentIdsByAuthor")]
    async fn get_intent_ids_by_author(&self, author: Address) -> RpcResult<Vec<IntentId>>;

    #[method(name = "getActiveIntentsByAuthor")]
    async fn get_active_intents_by_author(&self, author: Address) -> RpcResult<Vec<Intent>>;

    #[method(name = "getLiquidityIntentsByAuthor")]
    async fn get_liquidity_intents_by_author(&self, author: Address) -> RpcResult<Vec<Intent>>;

    #[method(name = "getBridgeIntentsByAuthor")]
    async fn get_bridge_intents_by_author(&self, author: Address) -> RpcResult<Vec<Intent>>;

    #[method(name = "getLatestLiquidityVersion")]
    async fn get_latest_liquidity(&self, intent_id: IntentId) -> RpcResult<IntentId>;

    #[method(name = "getSolutionsForSolver")]
    async fn get_solutions_for_solver(
        &self,
        solver_address: Address,
    ) -> RpcResult<Vec<SignedSolution>>;

    #[method(name = "getIntent")]
    async fn get_intent(&self, intent_id: B256) -> RpcResult<Option<Intent>>;

    #[method(name = "getIntentStatus")]
    async fn get_intent_status(&self, intent_id: B256) -> RpcResult<Option<IntentState>>;

    #[method(name = "proposeIntent")]
    async fn propose_intent(&self, intent: SignedIntent) -> RpcResult<(B256, IntentId)>;

    #[method(name = "createRefinement")]
    async fn create_refinement(&self, intent: Intent) -> RpcResult<IntentId>;

    #[method(name = "queryRefinement")]
    async fn query_refinement(&self, intent_id: IntentId) -> RpcResult<Option<RefinementStatus>>;

    #[method(name = "cancelIntent")]
    async fn cancel_intent(&self, signed_intent_id: SignedPayloadIntentId) -> RpcResult<B256>;

    #[method(name = "getHistory")]
    async fn get_history_for_intent(&self, intent_id: B256) -> RpcResult<(IntentHistory, Intent)>;

    #[method(name = "withdrawMtokens")]
    async fn withdraw_mtokens(&self, signed_payload: SignedWithdrawalPayload) -> RpcResult<B256>;

    #[method(name = "fastWithdrawMTokenWithWitness")]
    async fn fast_withdraw_mtokens_with_witness(
        &self,
        permit_and_witness: TypedData,
        user_signature: Bytes,
    ) -> RpcResult<(B256, Bytes)>;

    #[method(name = "fastWithdrawMToken")]
    async fn fast_withdraw_mtoken(
        &self,
        permit: FastWithdrawalPermit,
        user_signature: Bytes,
    ) -> RpcResult<(B256, Bytes)>;

    #[method(name = "getFailedIntentsSince")]
    async fn get_failed_intents_since_timestamp(
        &self,
        timestamp: u64,
    ) -> RpcResult<Vec<(IntentHistory, Intent)>>;

    #[method(name = "getNonce")]
    async fn get_nonce(&self, user: Address) -> RpcResult<U256>;

    #[method(name = "requestAddSolver")]
    async fn request_add_solver(&self, signed_address: SignedPayloadAddress) -> RpcResult<()>;
}

pub fn create_medusa_rpc_client(url: String) -> Result<HttpClient> {
    HttpClientBuilder::default()
        .build(url)
        .map_err(anyhow::Error::msg)
}
