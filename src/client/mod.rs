use alloy::primitives::{Address, B256, U256};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::rpc_params;
use serde::de::DeserializeOwned;

use super::types::intents::SignedIntent;
use super::types::rpc_payloads::SignedPayloadIntentId;

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

    async fn call_rpc<T, P>(&self, method: &str, params: P, operation: &str) -> Result<T>
    where
        T: DeserializeOwned,
        P: Send + Sync + ToRpcParams,
    {
        let res = self
            .http_client
            .request::<T, _>(method, params)
            .await
            .map_err(|e| {
                error!("{} failed with error: {}", operation, e);
                anyhow::anyhow!("{} failed", operation)
            })?;

        Ok(res)
    }

    pub async fn publish_intent(&self, intent: &SignedIntent) -> Result<(B256, B256)> {
        let params = rpc_params![intent];
        let res: (B256, B256) = self
            .call_rpc("proposeIntent", params, "Intent published")
            .await?;
        info!(
            "Intent {:?} published. intent id: {}, tx hash: {}",
            intent.intent, res.1, res.0
        );
        Ok(res)
    }

    pub async fn cancel_intent(&self, intent_id: SignedPayloadIntentId) -> Result<B256> {
        let raw_intent_id = intent_id.payload.intent_id;
        let params = rpc_params![intent_id];
        let tx_hash: B256 = self
            .call_rpc("cancelIntent", params, "Intent cancelled")
            .await?;
        info!(
            "Intent {:?} is cancelled. tx hash: {}",
            raw_intent_id, tx_hash
        );
        Ok(tx_hash)
    }

    pub async fn get_mtoken_balance(&self, user: Address, mtoken: Address) -> Result<U256> {
        let params = rpc_params![user, mtoken];
        let res: U256 = self
            .call_rpc(
                "getMtokenBalanceByAuthor",
                params,
                "Failed to get mtoken balance",
            )
            .await?;
        Ok(res)
    }
}
