use alloy::primitives::{Address, address};
use arcadia_sdk_rs::types::config::registry::{
    CrossChainSystem, CrossChainSystemContracts, load_registry, spoke_registry::SpokeRegistry,
};
use std::str::FromStr;

#[test]
fn test_spoke_registry() {
    let spoke_registry: SpokeRegistry = load_registry("tests/config/arbitrum.json").unwrap();
    assert_eq!(spoke_registry.name, "Arbitrum");
    assert_eq!(spoke_registry.chain_id, 42161);
    assert_eq!(spoke_registry.short_name, "arbitrum");
    assert_eq!(spoke_registry.native_currency.name, "Ether");
    assert_eq!(spoke_registry.native_currency.symbol, "ETH");
    assert_eq!(spoke_registry.native_currency.decimals, 18);
    assert_eq!(
        spoke_registry.cross_chain_systems,
        vec![CrossChainSystem::Hyperlane]
    );
    assert_eq!(spoke_registry.rpc, vec!["https://arb1.arbitrum.io/rpc"]);
    assert_eq!(
        spoke_registry.arcadia_contracts.event_handler,
        address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
    );
    assert_eq!(
        spoke_registry
            .cross_chain_system_contracts
            .hyperlane
            .mailbox,
        address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65")
    );
    assert_eq!(
        spoke_registry.cross_chain_system_contracts.hyperlane.igp,
        address!("9965507D1a55bcC2695C58ba16FB37d819B0A4dc")
    );
    assert_eq!(
        spoke_registry
            .cross_chain_system_contracts
            .hyperlane
            .gas_amount_oracle,
        address!("976EA74026E726554dB657fA54763abd0C3a0aa9")
    );
}
