use alloy::network::{EthereumWallet, TxSigner};
use alloy::primitives::{Address, B256, Bytes, ChainId, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider};
use alloy::signers::Signature;

use crate::error::Error;
use crate::types::sol_types::AssetReserves::AssetReservesInstance;
use crate::types::sol_types::ERC20::ERC20Instance;
use crate::types::sol_types::FastWithdrawalPermit;

pub type EthereumProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

pub struct SpokeClient {
    provider: EthereumProvider,
    asset_reserves_address: Address,
}

impl SpokeClient {
    pub async fn new<S>(signer: S, url: String, asset_reserves_address: Address) -> Self
    where
        S: TxSigner<Signature> + Send + Sync + 'static,
    {
        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(url.as_str())
            .await
            .unwrap();

        Self {
            provider,
            asset_reserves_address,
        }
    }

    pub async fn deposit(
        &self,
        owner: Address,
        token: Address,
        amount: U256,
        dest_chain: ChainId,
    ) -> Result<B256, Error> {
        let allowance_contract = ERC20Instance::new(token, self.provider.clone());
        let allowance_amount = allowance_contract
            .allowance(owner, self.asset_reserves_address)
            .call()
            .await?;

        if allowance_amount < amount {
            return Err(Error::InsufficientAllowance(allowance_amount, amount));
        }

        let asset_reserves_contract =
            AssetReservesInstance::new(self.asset_reserves_address, self.provider.clone());
        let receipt = asset_reserves_contract
            .deposit(token, amount, dest_chain as u32)
            .send()
            .await?
            .get_receipt()
            .await?;

        Ok(receipt.transaction_hash)
    }

    pub async fn withdraw_with_permit(
        &self,
        permit: FastWithdrawalPermit,
        receiver: Address,
        user_signature: Bytes,
        operator_signature: Bytes,
    ) -> Result<B256, Error> {
        let asset_reserves_contract =
            AssetReservesInstance::new(self.asset_reserves_address, self.provider.clone());
        let receipt = asset_reserves_contract
            .withdrawWithPermit(permit, receiver, user_signature, operator_signature)
            .send()
            .await?
            .get_receipt()
            .await?;
        Ok(receipt.transaction_hash)
    }
}
