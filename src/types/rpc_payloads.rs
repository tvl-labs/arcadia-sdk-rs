use super::sol_types::FastWithdrawalPermit;
use alloy::dyn_abi::TypedData;
use alloy::primitives::{Address, B256, Bytes, Signature, U256};
use anyhow::{Context, ensure};
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
                    signature: signature.as_bytes().to_vec().into(),
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

pub type SignedPayloadIntentId = SignedPayload<PayloadIntentId>;
impl_signable!(PayloadIntentId);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayloadAddress {
    pub address: Address,
    pub nonce: U256,
    pub chain_id: u64,
}

pub type SignedPayloadAddress = SignedPayload<PayloadAddress>;
impl_signable!(PayloadAddress);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalPayload {
    pub address: Address,
    pub amount: U256,
    pub mtoken: Address,
    pub nonce: U256,
    pub chain_id: u64,
}

pub type SignedWithdrawalPayload = SignedPayload<WithdrawalPayload>;
impl_signable!(WithdrawalPayload);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultDepositPayload {
    pub depositor_address: Address,
    pub teller_address: Address,
    pub asset: Address,
    pub amount: U256,
    pub min_shares: U256,
    pub nonce: U256,
    pub chain_id: u64,
}

pub type SignedVaultDepositPayload = SignedPayload<VaultDepositPayload>;
impl_signable!(VaultDepositPayload);

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
pub type SignedVaultWithdrawalPayload = SignedPayload<VaultWithdrawalPayload>;
impl_signable!(VaultWithdrawalPayload);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPayload<T> {
    pub payload: T,
    pub signature: Bytes,
}

impl<T> SignedPayload<T>
where
    T: Serialize,
{
    pub fn recover_signer_address(&self) -> anyhow::Result<Address> {
        let addr = Signature::from_raw(&self.signature)?
            .recover_address_from_msg(bcs::to_bytes(&self.payload)?.as_slice())?;
        Ok(addr)
    }
}

const FAST_WITHDRAWAL_PERMIT_TYPE_STUB: &str = "FastWithdrawalPermit(uint256 nonce,uint32 spokeChainId,address token,uint256 amount,address user,address caller,";

/// Validate that the typed data is mostly valid for a fast withdrawal permit with an extra witness.
///
/// Note: the domain is not validated.
pub fn validate_and_extract_fast_withdrawal_permit_with_witness_typed_data(
    typed_data: &TypedData,
) -> anyhow::Result<(FastWithdrawalPermit, String, B256)> {
    let type_string = typed_data.encode_type()?;
    // Witness type string is the type string without the stub prefix.
    let witness_type_string = type_string
        .strip_prefix(FAST_WITHDRAWAL_PERMIT_TYPE_STUB)
        .context("type string does not start with the expected stub")?
        .to_string();

    let data = typed_data.encode_data()?;
    // Data length should be 8 * 32 for all the fields of the permit and the witness.
    ensure!(data.len() == 32 * 7);
    // The witness is the last 32 bytes of the encoded data.
    let witness = B256::from_slice(&data[data.len() - 32..]);
    // The message should have all the fields of the permit, so we should be able to deserialize the permit from the message.
    let permit = serde_json::from_value(typed_data.message.clone())?;
    Ok((permit, witness_type_string, witness))
}

#[test]
fn test_validate_and_extract_fast_withdrawal_permit_with_witness_typed_data() {
    let typed_data: TypedData = serde_json::from_str(
        r#"
        {
            "domain": {
                "name": "FastWithdrawalPermit",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0x1234567890123456789012345678901234567890"
            },
            "types": {
                "FastWithdrawalPermit": [
                    { "name": "nonce", "type": "uint256" },
                    { "name": "spokeChainId", "type": "uint32" },
                    { "name": "token", "type": "address" },
                    { "name": "amount", "type": "uint256" },
                    { "name": "user", "type": "address" },
                    { "name": "caller", "type": "address" },
                    { "name": "foo", "type": "Foo" }
                ],
                "Foo": [
                    { "name": "bar", "type": "uint32" }
                ]
            },
            "primaryType": "FastWithdrawalPermit",
            "message": {
                "nonce": "1",
                "hubChainId": 2,
                "spokeChainId": 3,
                "token": "0x1234567890123456789012345678901234567890",
                "amount": "4",
                "user": "0x1234567890123456789012345678901234567890",
                "caller": "0x1234567890123456789012345678901234567890",
                "foo": {
                    "bar": 7
                }
            }
        }
    "#,
    )
    .unwrap();
    let (_permit, witness_type_string, witness) =
        validate_and_extract_fast_withdrawal_permit_with_witness_typed_data(&typed_data).unwrap();
    assert_eq!(witness_type_string, "Foo foo)Foo(uint32 bar)");
    println!("witness: {witness}");
}
