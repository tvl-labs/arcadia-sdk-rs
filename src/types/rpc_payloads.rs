use super::sol_types::FastWithdrawalPermit;
use alloy::dyn_abi::TypedData;
use alloy::primitives::{Address, B256, Bytes, Signature};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolStruct};
use anyhow::{Context, ensure};
use serde::{Deserialize, Serialize};

macro_rules! impl_signable {
    ($type:ty) => {
        impl $type {
            pub async fn sign(
                self,
                signer: &(impl alloy::signers::Signer + Send + Sync),
                domain: &Eip712Domain,
            ) -> Result<SignedPayload<Self>, alloy::signers::Error> {
                let hash = self.eip712_signing_hash(domain);
                let signature = signer.sign_hash(&hash).await?;
                Ok(SignedPayload {
                    payload: self,
                    signature: signature.as_bytes().to_vec().into(),
                })
            }
        }
    };
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct CancelIntent {
        uint256 nonce;
        bytes32 intentId;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AddSolver {
        address address;
        uint256 nonce;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Withdraw {
        address address;
        uint256 amount;
        address mtoken;
        uint256 nonce;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct VaultDeposit {
        address depositorAddress;
        address tellerAddress;
        address asset;
        uint256 amount;
        uint256 minShares;
        uint256 nonce;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct VaultWithdraw {
        address depositorAddress;
        address tellerAddress;
        address asset;
        uint256 shares;
        uint256 minAmount;
        uint16 feePercentage;
        uint256 nonce;
    }
}

impl_signable!(CancelIntent);
impl_signable!(AddSolver);
impl_signable!(Withdraw);
impl_signable!(VaultDeposit);
impl_signable!(VaultWithdraw);

pub type SignedCancelIntent = SignedPayload<CancelIntent>;
pub type SignedAddSolver = SignedPayload<AddSolver>;
pub type SignedWithdraw = SignedPayload<Withdraw>;
pub type SignedVaultDeposit = SignedPayload<VaultDeposit>;
pub type SignedVaultWithdraw = SignedPayload<VaultWithdraw>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPayload<T> {
    pub payload: T,
    pub signature: Bytes,
}

impl<T> SignedPayload<T>
where
    T: SolStruct,
{
    pub fn recover_signer_address(&self, domain: &Eip712Domain) -> anyhow::Result<Address> {
        let hash = self.payload.eip712_signing_hash(&domain);
        let addr = Signature::from_raw(&self.signature)?.recover_address_from_prehash(&hash)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;
    use alloy::sol_types::eip712_domain;

    #[tokio::test]
    async fn test_signing_and_recovery() {
        let signer = alloy::signers::local::PrivateKeySigner::random();
        let domain = eip712_domain!(
            name: "Arcadia",
            version: "1",
            chain_id: 1,
            verifying_contract: "0x1234567890123456789012345678901234567890".parse().unwrap(),
        );

        let payload = CancelIntent {
            intentId: B256::random(),
            nonce: U256::from(1),
        };
        let signed_payload = payload.sign(&signer, &domain).await.unwrap();
        let recovered_address = signed_payload.recover_signer_address(&domain).unwrap();
        assert_eq!(recovered_address, signer.address());
    }
}
