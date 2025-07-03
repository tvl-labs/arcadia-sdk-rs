pub mod client;
pub mod error;
pub mod types;

#[cfg(test)]

mod tests {
    use crate::types::{FromSol, ToSol};
    use crate::types::{events, intents, receipt, solidity, solution};
    use crate::types::{
        events::XChainEvent, intents::Intent, receipt::Receipt, solidity::Solution,
    };
    use alloy::{
        hex::FromHex,
        primitives::{Address, B256, U256},
    };
    #[test]
    fn test_to_and_from_sol_for_xchain_event() {
        use crate::types::events::XChainEvent;
        let xchain_event = XChainEvent {
            publisher: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
            event_hash: B256::from_hex(
                "0x1234567890123456789012345678901234567890000000000000000000000000",
            )
            .unwrap(),
            origin_chain_id: 31337,
            event_nonce: U256::from(1_u64),
            event_data: vec![0x01, 0x02, 0x03, 0x04],
        };

        let sol_xchain_event = xchain_event.to_sol();
        let new_xchain_event = XChainEvent::from_sol(sol_xchain_event);
        assert_eq!(xchain_event, new_xchain_event);
    }
}
