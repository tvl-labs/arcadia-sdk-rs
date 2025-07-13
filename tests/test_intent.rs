use alloy::primitives::{Address, U256, address};
use arcadia_sdk::types::config::registry::arcadia_registry::ArcadiaChainRegistry;
use arcadia_sdk::types::config::registry::{get_intent_mtokens_use_record, load_registry};
use arcadia_sdk::types::intents::Intent;
use arcadia_sdk::types::{DEFAULT_PRECISION, base_precision};
use std::ops::Mul;

#[test]
fn test_intent_from_json() {
    let intent_string = std::fs::read_to_string("tests/intent.json").unwrap();
    let mut intent: Intent = serde_json::from_str(&intent_string).unwrap();
    let arcadia_registry: ArcadiaChainRegistry =
        load_registry("tests/config/arcadia.json").unwrap();

    let mtoken_entry = arcadia_registry
        .get_mtoken_entry_by_address(intent.src_m_token)
        .unwrap();
    assert_eq!(mtoken_entry.name, "EthSepUSDC");
    assert_eq!(mtoken_entry.symbol, "EthSepUSDC");

    let src_amount = intent.src_amount;
    let test_amt = U256::from(2000000_u128);
    assert_eq!(src_amount, test_amt);

    let correct_mtoken_amount = U256::from(2_u64).mul(base_precision());
    intent.normalize_src_amount(6);
    assert_eq!(intent.src_amount, correct_mtoken_amount);

    let curr_outcome_amount = intent.outcome.m_amounts[0];
    assert_eq!(curr_outcome_amount, test_amt);
    intent.normalize_outcome_amounts([6]);
    assert_eq!(intent.outcome.m_amounts[0], correct_mtoken_amount);

    let mtokens_use_record = get_intent_mtokens_use_record(&intent, &arcadia_registry);
    assert_eq!(mtokens_use_record.consumed_mtoken.name, "EthSepUSDC");
    assert_eq!(mtokens_use_record.outcome_mtokens.len(), 1);
}
