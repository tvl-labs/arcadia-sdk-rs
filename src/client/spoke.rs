use alloy::network::{EthereumWallet, TxSigner};
use alloy::primitives::{Address, B256, Bytes, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider};
use alloy::providers::{Provider, WalletProvider};
use alloy::signers::Signature;

use anyhow::Result;

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

    pub async fn deposit_to_asset_reserves(
        &self,
        token: Address,
        amount: U256,
    ) -> Result<B256, Error> {
        let allowance_contract = ERC20Instance::new(token, self.provider.clone());
        let owner = self.provider.wallet().default_signer().address();
        let allowance_amount = allowance_contract
            .allowance(owner, self.asset_reserves_address)
            .call()
            .await?;

        if allowance_amount < amount {
            match self
                .erc20_approve(token, self.asset_reserves_address, amount)
                .await
            {
                Ok(_) => {}
                Err(_) => return Err(Error::InsufficientAllowance(allowance_amount, amount)),
            }
        }

        let asset_reserves_contract =
            AssetReservesInstance::new(self.asset_reserves_address, self.provider.clone());
        let receipt = asset_reserves_contract
            .deposit(token, amount)
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

    pub async fn get_erc20_balance(&self, owner: Address, token: Address) -> Result<U256> {
        let erc20_contract = ERC20Instance::new(token, self.provider.clone());
        let balance = erc20_contract.balanceOf(owner).call().await?;
        Ok(balance)
    }

    pub async fn erc20_approve(
        &self,
        token: Address,
        spender: Address,
        amount: U256,
    ) -> Result<B256> {
        let erc20_contract = ERC20Instance::new(token, self.provider.clone());
        let receipt = erc20_contract
            .approve(spender, amount)
            .send()
            .await?
            .get_receipt()
            .await?;
        Ok(receipt.transaction_hash)
    }

    pub async fn get_native_token_balance(&self, owner: Address) -> Result<U256> {
        let balance = self.provider.get_balance(owner).await?;
        Ok(balance)
    }
}
