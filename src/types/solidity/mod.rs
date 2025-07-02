use alloy::primitives::B256;
use alloy::sol_types::eip712_domain;
use alloy::{dyn_abi::Eip712Domain, primitives::Address, sol};

use crate::types::SolidityType;

sol! {


    struct XChainEvent {
        address publisher;
        uint256 originChainId;
        bytes32 eventHash;
        uint256 eventNonce;
        bytes eventData;
    }

    struct AssetReserveDeposit {
        address token;
        uint256 amount;
        address depositor;
    }
    struct MTokenWithdrawal {
        address token;
        uint256 amount;
        address withdrawer;
    }



    enum OutcomeAssetStructure {
        AnySingle,
        Any,
        All
    }

    enum FillStructure {
        Exactly,
        Minimum,
        PctFilled,
        ConcreteRange
    }

    struct Outcome {
        address[] mTokens;
        uint256[] mAmounts;
        OutcomeAssetStructure outcomeAssetStructure;
        FillStructure fillStructure;
    }

    struct Intent {
        address author;
        uint256 ttl;
        uint256 nonce;
        address srcMToken;
        uint256 srcAmount;
        Outcome outcome;
    }

    struct SignedIntent {
        Intent intent;
        bytes signature;
    }

    enum IntentState {
        NonExistent,
        Locked,
        Settled,
        Open,
        Solved,
        Expired,
        Cancelled,
        Error,
    }

    struct Receipt {
        address mToken;
        uint256 mTokenAmount;
        address owner;
        bytes32 intentHash;
    }


    enum OutType {
        Intent,
        Receipt
    }

    struct OutputIdx {
        OutType outType;
        uint64 outIdx;
    }

    struct MoveRecord {
        uint64 srcIdx;
        OutputIdx outputIdx;
        uint256 qty;
    }

    struct FillRecord {
        uint64 inIdx;
        uint64 outIdx;
        OutType outType;
    }

    struct Solution {
        bytes32[] intentIds;
        Intent[] intentOutputs;
        Receipt[] receiptOutputs;
        MoveRecord[] spendGraph;
        FillRecord[] fillGraph;
    }

    struct SignedSolution {
        Solution solution;
        bytes signature;
    }


    bytes32 constant ASSET_RESERVE_DEPOSIT_STRUCT_TYPE_HASH = keccak256(
        "AssetReserveDeposit(address token, uint256 amount, address depositor)",
    );
    bytes32 constant MTOKEN_WITHDRAWAL_STRUCT_TYPE_HASH = keccak256(
        "MTokenWithdrawal(address token, uint256 amount, address withdrawer)",
    );

    bytes32 constant DEPOSIT_EVENT = keccak256("AssetReserveDeposit");


}

impl SolidityType for Intent {}
impl SolidityType for FillStructure {}
impl SolidityType for OutcomeAssetStructure {}
impl SolidityType for Outcome {}
impl SolidityType for Receipt {}
impl SolidityType for Solution {}
impl SolidityType for SignedSolution {}
impl SolidityType for MoveRecord {}
impl SolidityType for FillRecord {}
impl SolidityType for OutputIdx {}
impl SolidityType for OutType {}
