use alloy::primitives::address;
use arcadia_sdk::load_registry;
use arcadia_sdk::types::config::registry::{
    CrossChainSystem, arcadia_registry::ArcadiaChainRegistry, spoke_registry::SpokeRegistry,
};

#[test]
fn test_spoke_registry() {
    let spoke_registry: SpokeRegistry = load_registry!("config/arbitrum.json").unwrap();
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

#[test]
fn test_arcadia_registry() {
    let arcadia_registry: ArcadiaChainRegistry = load_registry!("config/arcadia.json").unwrap();
    assert_eq!(arcadia_registry.name, "Arcadia Testnet 2");
    assert_eq!(arcadia_registry.chain_id, 1098411886);
    assert_eq!(arcadia_registry.short_name, "arcadia-testnet");
    assert_eq!(arcadia_registry.native_currency.name, "AIP");
    assert_eq!(arcadia_registry.native_currency.symbol, "AIP");
    assert_eq!(arcadia_registry.native_currency.decimals, 18);

    let usdc_arb = arcadia_registry.mtokens.get("arbSepUSDC").unwrap();
    assert_eq!(usdc_arb.spoke_chain.name, "Arbitrum Sepolia");
    assert_eq!(usdc_arb.spoke_chain.registry_name, "arbitrumsepolia");
    assert_eq!(
        usdc_arb.spoke_chain.spoke_token_address,
        address!("0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d")
    );
    assert_eq!(usdc_arb.spoke_chain.spoke_token_name, "USD Coin");
    assert_eq!(usdc_arb.spoke_chain.spoke_token_symbol, "USDC");
    assert_eq!(usdc_arb.spoke_chain.spoke_token_decimals, 6);
    assert_eq!(usdc_arb.spoke_chain.chain_id, 421614);
}
