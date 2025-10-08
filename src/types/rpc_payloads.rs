use alloy::primitives::{Address, B256, Signature, U256};
use serde::{Deserialize, Serialize};

macro_rules! impl_signable {
    ($type:ty) => {
        impl $type {
            pub async fn sign(
                self,
                signer: &(impl alloy::signers::Signer + Send + Sync),
            ) -> Result<SignedPayload<Self>, alloy::signers::Error> {
                let bytes = bcs::to_bytes(&self).map_err(|e| {
                    alloy::signers::Error::other(format!("BCS serialization failed: {}", e))
                })?;
                let signature = signer.sign_message(&bytes).await?;
                Ok(SignedPayload {
                    payload: self,
                    signature,
                })
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayloadIntentId {
    pub intent_id: B256,
    pub nonce: U256,
    pub chain_id: u64,
}

impl_signable!(PayloadIntentId);

pub type SignedPayloadIntentId = SignedPayload<PayloadIntentId>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayloadAddress {
    pub address: Address,
    pub nonce: U256,
    pub chain_id: u64,
}

impl_signable!(PayloadAddress);

pub type SignedPayloadAddress = SignedPayload<PayloadAddress>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalPayload {
    pub address: Address,
    pub amount: U256,
    pub mtoken: Address,
    pub nonce: U256,
    pub chain_id: u64,
}

impl_signable!(WithdrawalPayload);

pub type SignedWithdrawalPayload = SignedPayload<WithdrawalPayload>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultWithdrawalPayload {
    pub depositor_address: Address,
    pub teller_address: Address,
    pub asset: Address,
    pub shares: U256,
    pub min_amount: U256,
    pub fee_percentage: u16,
    pub nonce: U256,
    pub chain_id: u64,
}

impl_signable!(VaultWithdrawalPayload);

pub type SignedVaultWithdrawalPayload = SignedPayload<VaultWithdrawalPayload>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPayload<T> {
    pub payload: T,
    pub signature: Signature,
}
