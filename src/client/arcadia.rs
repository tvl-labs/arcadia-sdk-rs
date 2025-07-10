use crate::{
    error::Error,
    types::solidity::{Intent, IntentBook, MTokenInfo, MTokenRegistry},
};
use alloy::providers::{Provider, WalletProvider};
use alloy::signers::Signer;
use alloy::{
    primitives::{Address, B256},
    providers::ProviderBuilder,
};
use reqwest::Url;
use std::sync::Arc;

pub struct ArcadiaClient<P> {
    pub provider: Arc<P>,
    pub intent_book: Address,
    pub m_token_manager: Address,
}

impl<P: Provider + 'static + WalletProvider> ArcadiaClient<P> {
    pub async fn new(
        provider: Arc<P>,
        intent_book: Address,
        m_token_manager: Address,
    ) -> Result<Self, Error> {
        Ok(Self {
            provider,
            intent_book,
            m_token_manager,
        })
    }

    pub async fn get_intents_for_author(&self, author: Address) -> Result<Vec<B256>, Error> {
        let intent_book = IntentBook::new(self.intent_book, self.provider.clone());
        let intent_ids = intent_book
            .getIntentIdsByAuthor(author)
            .call()
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?;
        Ok(intent_ids)
    }
}
