pub mod client;
pub mod error;
pub mod types;

use alloy::primitives::{Address, B256, U256};

use crate::{
    client::{MedusaRpcClient, SpokeClient},
    types::sol_types::FastWithdrawalPermit,
};

pub async fn fast_withdraw_mtoken(
    signer: &(impl alloy::signers::Signer + Send + Sync),
    medusa_client: &impl MedusaRpcClient,
    spoke_client: &SpokeClient,
    arcadia_chain_id: u64,
    mtoken_manager: Address,
    permit: FastWithdrawalPermit,
    receiver: Address,
) -> Result<(B256, B256), anyhow::Error> {
    let user_signature = permit
        .clone()
        .sign(signer, arcadia_chain_id, mtoken_manager)
        .await?;
    let (arcadia_hash, operator_signature) = medusa_client
        .fast_withdraw_mtoken(permit.clone(), user_signature.clone())
        .await?;
    let spoke_hash = spoke_client
        .withdraw_with_permit(permit, receiver, user_signature, operator_signature)
        .await?;
    Ok((arcadia_hash, spoke_hash))
}

pub fn build_fast_withdrawal_permit(
    spoke_chain_id: u32,
    spoke_token: Address,
    amount: U256,
    user: Address,
    caller: Address,
) -> FastWithdrawalPermit {
    FastWithdrawalPermit {
        nonce: U256::random(),
        spokeChainId: spoke_chain_id,
        token: spoke_token,
        amount,
        user,
        caller,
    }
}
