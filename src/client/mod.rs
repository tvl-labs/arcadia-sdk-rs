use alloy::primitives::{Address, B256, U256};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::rpc_params;
use serde::de::DeserializeOwned;

use super::types::intents::SignedIntent;
use super::types::rpc_payloads::{SignedPayloadIntentId, SignedVaultWithdrawalPayload};

use anyhow::Result;

use tracing::{error, info};

pub struct MedusaClient {
    http_client: HttpClient,
}

impl MedusaClient {
    pub async fn new(url: String) -> Self {
        Self {
            http_client: HttpClient::builder()
                .build(url)
                .expect("Failed to create http client"),
        }
    }

    async fn call_rpc<T, P>(&self, method: &str, params: P) -> Result<T>
    where
        T: DeserializeOwned,
        P: Send + Sync + ToRpcParams,
    {
        let res = self
            .http_client
            .request::<T, _>(method, params)
            .await
            .map_err(|e| {
                error!("{} failed with error: {}", method, e);
                anyhow::anyhow!("{} failed", method)
            })?;

        Ok(res)
    }

    pub async fn publish_intent(&self, intent: &SignedIntent) -> Result<(B256, B256)> {
        let params = rpc_params![intent];
        let res: (B256, B256) = self.call_rpc("proposeIntent", params).await?;
        info!(
            "Intent {:?} published. intent id: {}, tx hash: {}",
            intent.intent, res.1, res.0
        );
        Ok(res)
    }

    pub async fn cancel_intent(&self, intent_id: SignedPayloadIntentId) -> Result<B256> {
        let raw_intent_id = intent_id.payload.intent_id;
        let params = rpc_params![intent_id];
        let tx_hash: B256 = self.call_rpc("cancelIntent", params).await?;
        info!(
            "Intent {:?} is cancelled. tx hash: {}",
            raw_intent_id, tx_hash
        );
        Ok(tx_hash)
    }

    pub async fn get_mtoken_balance(&self, user: Address, mtoken: Address) -> Result<U256> {
        let params = rpc_params![user, mtoken];
        let res: U256 = self.call_rpc("getMtokenBalanceByAuthor", params).await?;
        Ok(res)
    }

    pub async fn preview_withdraw(
        &self,
        teller: Address,
        shares_to_burn: U256,
        mtoken: Address,
        fee_basis_points: u16,
    ) -> Result<U256> {
        let params = rpc_params![teller, mtoken, shares_to_burn, fee_basis_points];
        let res: U256 = self
            .call_rpc("previewMaximumWithdrawFromVault", params)
            .await?;
        Ok(res)
    }

    pub async fn request_withdraw(&self, payload: SignedVaultWithdrawalPayload) -> Result<U256> {
        let params = rpc_params![payload];
        let res: U256 = self.call_rpc("requestWithdrawFromVault", params).await?;
        Ok(res)
    }

    pub async fn get_vault_total_asset_value(&self, teller: Address) -> Result<U256> {
        let params = rpc_params![teller];
        let res: U256 = self.call_rpc("getVaultTotalAssetValue", params).await?;
        Ok(res)
    }

    pub async fn get_vault_total_shares(&self, teller: Address) -> Result<U256> {
        let params = rpc_params![teller];
        let res: U256 = self.call_rpc("getVaultTotalShares", params).await?;
        Ok(res)
    }
}
