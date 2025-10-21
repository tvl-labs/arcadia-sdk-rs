use std::sync::Arc;

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, WalletProvider as _};
use alloy::signers::local::PrivateKeySigner;
use arcadia_sdk::client::arcadia::ArcadiaClient;
use arcadia_sdk::error::Error;
use std::env;

#[tokio::test]
#[ignore]
async fn test_get_intents_for_author() {
    // A key derived from the mnemonic "test test test test test test test test test test test junk"
    let signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"
        .parse::<PrivateKeySigner>()
        .unwrap();
    let rpc_url = env::var("RPC_URL").unwrap();
    let intent_book = env::var("INTENT_BOOK").unwrap();
    let m_token_manager = env::var("M_TOKEN_MANAGER").unwrap();
    let intent_book = intent_book.parse::<Address>().unwrap();
    let intent_author = env::var("INTENT_AUTHOR").unwrap();
    let intent_author = intent_author.parse::<Address>().unwrap();
    let m_token_manager = m_token_manager.parse::<Address>().unwrap();
    let wallet = EthereumWallet::new(signer.clone());
    let provider = ProviderBuilder::default()
        .with_recommended_fillers()
        .wallet(wallet)
        .connect(&rpc_url)
        .await
        .map_err(|e| Error::ProviderError(e.to_string()))
        .unwrap();
    let arcadia_client = ArcadiaClient::new(Arc::new(provider), intent_book, m_token_manager)
        .await
        .unwrap();

    let intent_ids = arcadia_client
        .get_intents_for_author(intent_author)
        .await
        .unwrap();
    println!("intent_ids: {intent_ids:?}");
    let has_intent_ids = !intent_ids.is_empty();

    let fake_addr = arcadia_client.provider.signer_addresses().next().unwrap();
    eprintln!("fake addr: {fake_addr:#?}");
    assert!(has_intent_ids);
    let new_intent_ids = arcadia_client
        .get_intents_for_author(fake_addr)
        .await
        .unwrap();
    assert_eq!(new_intent_ids.len(), 0);
}
