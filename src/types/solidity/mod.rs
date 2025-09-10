pub mod contract;

use alloy::sol;

sol! {
    struct XChainEvent {
        address publisher;
        bytes32 eventHash;
        uint256 chainId;
    }

    struct AssetReserveDeposit {
        address token;
        uint256 amount;
        address depositor;
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
}
