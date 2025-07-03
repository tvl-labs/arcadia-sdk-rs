pub mod client;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::types::intents::{FillStructure, Outcome, OutcomeAssetStructure};
    use crate::types::solution::{FillRecord, MoveRecord, OutType, OutputIdx};
    use crate::types::{FromSol, ToSol};
    use crate::types::{events, intents, receipt, solidity, solution};
    use crate::types::{
        events::XChainEvent, intents::Intent, receipt::Receipt, solution::Solution,
    };
    use alloy::{
        hex::FromHex,
        primitives::{Address, B256, U256},
    };
    #[test]
    fn test_to_and_from_sol_for_xchain_event() {
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
        let new_xchain_event = XChainEvent::from_sol(sol_xchain_event.clone());
        let new_xchain_event_sol = new_xchain_event.to_sol();
        assert_eq!(sol_xchain_event, new_xchain_event_sol);
        assert_eq!(xchain_event, new_xchain_event);
    }

    #[test]
    fn test_to_and_from_sol_for_intents() {
        let intent = Intent {
            author: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
            ttl: U256::from(1_u64),
            nonce: U256::from(1_u64),
            src_m_token: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
            src_amount: U256::from(1_u64),
            outcome: Outcome {
                m_tokens: vec![
                    Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
                ],
                m_amounts: vec![U256::from(1_u64)],
                outcome_asset_structure: OutcomeAssetStructure::AnySingle,
                fill_structure: FillStructure::Exact,
            },
        };

        let sol_intent = intent.to_sol();
        let new_intent = Intent::from_sol(sol_intent.clone());
        let new_intent_sol = new_intent.to_sol();
        assert_eq!(sol_intent, new_intent_sol);
        assert_eq!(intent, new_intent);
    }

    #[test]
    fn test_to_and_from_sol_for_receipt() {
        let receipt = Receipt {
            m_token: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
            m_token_amount: U256::from(1_u64),
            owner: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
            intent_hash: B256::from_hex(
                "0x1234567890123456789012345678901234567890000000000000000000000000",
            )
            .unwrap(),
        };

        let sol_receipt = receipt.to_sol();
        let new_receipt = Receipt::from_sol(sol_receipt.clone());
        let new_receipt_sol = new_receipt.to_sol();
        assert_eq!(sol_receipt, new_receipt_sol);
        assert_eq!(receipt, new_receipt);
    }

    #[test]
    fn test_to_and_from_sol_for_solution() {
        let solution = Solution {
            intent_ids: vec![
                B256::from_hex(
                    "0x1234567890123456789012345678901234567890000000000000000000000000",
                )
                .unwrap(),
            ],
            intent_outputs: vec![Intent {
                author: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
                ttl: U256::from(1_u64),
                nonce: U256::from(1_u64),
                src_m_token: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
                    .unwrap(),
                src_amount: U256::from(1_u64),
                outcome: Outcome {
                    m_tokens: vec![
                        Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
                    ],
                    m_amounts: vec![U256::from(1_u64)],
                    outcome_asset_structure: OutcomeAssetStructure::AnySingle,
                    fill_structure: FillStructure::Exact,
                },
            }],
            receipt_outputs: vec![Receipt {
                m_token: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
                m_token_amount: U256::from(1_u64),
                owner: Address::from_hex("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
                intent_hash: B256::from_hex(
                    "0x1234567890123456789012345678901234567890000000000000000000000000",
                )
                .unwrap(),
            }],
            spend_graph: vec![MoveRecord {
                src_idx: 0,
                output_idx: OutputIdx {
                    out_type: OutType::Intent,
                    out_idx: 0,
                },
                qty: U256::from(1_u64),
            }],
            fill_graph: vec![FillRecord {
                in_idx: 0,
                out_idx: 0,
                out_type: OutType::Intent,
            }],
        };

        let sol_solution = solution.to_sol();
        let new_solution = Solution::from_sol(sol_solution.clone());
        let new_solution_sol = new_solution.to_sol();
        assert_eq!(sol_solution, new_solution_sol);
        assert_eq!(solution, new_solution);
    }
}
