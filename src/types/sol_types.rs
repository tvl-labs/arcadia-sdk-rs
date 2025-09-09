use alloy::primitives::{Address, B256};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolStruct, eip712_domain};

use super::conversion::{RpcToSol, SolidityType};
use super::intents::Intent as RpcIntent;
use super::{intents, receipt, solution};

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
        uint256 validBefore;
        uint256 validAfter;
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

    // We put all errors here so that we can decode them all with AllErrors.
    #[derive(Debug)]
    contract All {
        error IntentValidator__PublishError__ValidAfterLargerThanValidBefore();
        error IntentValidator__PublishError__InvalidIntentNonce();
        error IntentValidator__PublishError__IntentExpired();
        error IntentValidator__PublishError__MissingField(bytes32 intentId);
        error IntentValidator__PublishError__MAmountsMustYieldManageableOutcomes();
        error IntentValidator__PublishError__IntentAlreadyExists(bytes32 intentId);
        error IntentValidator__PublishError__OutcomeMTokenAndMAmountLengthMismatch(bytes32 intentId);
        error IntentValidator__PublishError__UnsupportedIntentType(bytes32 intentId);
        error IntentValidator__PublishError__ZeroOutcomeToken(bytes32 intentId, uint256 index);
        error IntentValidator__PublishError__ZeroOutcomeAmount(bytes32 intentId, uint256 index);
        error IntentValidator__CancelError__IntentNotOpen(bytes32 intentId);
        error SolutionValidator__IntentNotOpen(bytes32 intentId);
        error SolutionValidator__IntentExpired(bytes32 intentId);
        error SolutionValidator__IntentValidAfterNotReached(bytes32 intentId);
        error SolutionValidator__EmptySolution();
        error SolutionValidator__SolutionMustHaveReceiptOutputs();
        error SolutionValidator__ChildIntentAuthorMismatch(uint256 intentIdx);
        error SolutionValidator__ChildIntentMTokenMismatch(uint256 intentIdx);
        error SolutionValidator__ChildIntentValidBeforeMismatch(uint256 intentIdx);
        error SolutionValidator__ChildIntentValidAfterMismatch(uint256 intentIdx);
        error SolutionValidator__ChildIntentInvalidNonce(uint256 intentIdx);
        error SolutionValidator__ChildIntentOutcomeMismatch(uint256 intentIdx);
        error SolutionValidator__IntentSpentAmountMismatch(uint256 intentIdx, uint256 moveIdx);
        error SolutionValidator__ReceiptMTokenMismatch(uint256 intentIdx);
        error SolutionValidator__InvalidMoveRecord(uint256 intentIdx, uint256 moveIdx);
        error SolutionValidator__ReceiptMTokenNotFoundInIntentOutcome(uint256 intentIdx);
        error SolutionValidator__IntentTokenBurnt(uint256 intentIdx);
        error SolutionValidator__IntentTokenDoubleSpent(uint256 intentIdx);
        error SolutionValidator__IntentFillError__MultipleMTokensFilledForAnySingleIntent(uint256 intentIdx);
        error SolutionValidator__IntentFillError__NoMTokensFilledForAnySingleIntent(uint256 intentIdx);
        error SolutionValidator__MoveRecordOutputIdxOutOfBounds(uint256 moveIdx);
        error SolutionValidator__FillRecordOutputIdxOutOfBounds(uint256 fillIdx);
        error SolutionValidator__IntentMustSpendMToken(uint256 intentIdx);
        error SolutionValidator__PercentageIntentNotSatisfied(uint256 intentIdx);
        error SolutionValidator__ExactIntentMustSpendAllMTokens(uint256 intentIdx);
        error SolutionValidator__ExactIntentNotSatisfied(uint256 intentIdx);
        error SolutionValidator__UnsupportedFillStructure(uint256 intentIdx);
        error IntentBook__ValidAfterLargerThanValidBefore();
        error IntentBook__InvalidIntentNonce();
        error IntentBook__IntentExpired();
        error IntentBook__IntentNonactivated();
        error IntentBook__IntentAlreadyExists(bytes32 _intentId);
        error IntentBook__UnauthorizedIntentPublisher();
        error IntentBook__CannotLockIntentThatIsNotOpen(bytes32 intentId);
        error IntentBook__CannotCancelNonOpenIntent();
        error IntentBook__UnauthorizedCancellationAttempt();
        error IntentBook__InvalidSignature();
        error IntentBook__InvalidIntentAuthor();
        error IntentBook__IntentNotSpendable(bytes32 intentId);
        error IntentBook__IntentNotFound(bytes32 intentId);
        error IntentBook__CannotSpendIntentThatIsNotOpen(bytes32 intentId);
        error IntentBook__SpendingPartiallyFillableIntentMustMakeProgress();
        error IntentBook__IntentVersionsCannotChangeValidBeforeWhenSpent();
        error IntentBook__IntentVersionsCannotChangeValidAfterWhenSpent();
        error IntentBook__FillGraphCannotBeEmpty();
        error IntentBook__IntentPredecessorRootDoesNotMatch();
        error IntentBook__InvalidTimestamp();
        error IntentBook__IntentIdAuthorMismatch();
        error IntentBook__PercentageCannotBeZero();

        // MTokenManager errors.
        error UnauthorizedCaller();
        error InsufficientAllowance();
        error InsufficientIntentBalance(uint256 intentBalance, uint256 amount);
        error UnsupportedMToken();
        error MTokenPaused();
        error MTokenDestroyed();
        error CallerNotIntentBook();
        error InvalidSignature();
        error AuthorMismatch();
        error IntentIDMismatch();
        error InsufficientMTokens();
        error MissmatchedMToken();

        // ReceiptManager errors.
        error ReceiptNotFound();
        error UnauthorizedRedemption();
        error UnauthorizedReceiptIssuance();
        error ReceiptAlreadyLocked();
        error ReceiptNotLocked();
        error UnauthorizedLockOperation();
    }

    #[sol(rpc)]
    contract IntentBook {
        event IntentPublisherAdded(address indexed publisher);
        event IntentPublisherRevoked(address indexed publisher);
        event IntentCreated(
            bytes32 indexed intentId,
            address indexed author,
            address indexed srcMToken,
            uint256 srcAmount,
            address[] mTokens,
            uint256[] mAmounts,
            OutcomeAssetStructure outcomeAssetStructure,
            FillStructure fillStructure
        );
        event IntentLocked(bytes32 indexed intentId);
        event IntentCancelled(bytes32 indexed intentId);
        event IntentSolved(bytes32 indexed intentId);

        function cancelIntent(bytes32 intentId) external;
        function publishIntent(SignedIntent memory signedIntent) public returns (bytes32);
        function solve(Solution memory solution) public returns (bytes32);
        function setTimestamp(uint64 timestamp) public;

        uint64 public timestamp;
        function getNonce(address user) public view returns (uint256);
        function getIntent(bytes32 intentId) public view returns (Intent memory);
    }

    #[sol(rpc)]
    contract MTokenManager {
        function withdrawMToken(address from, address mToken, uint256 amount) external payable;
    }
}

impl From<solution::OutType> for OutType {
    fn from(ot: solution::OutType) -> Self {
        match ot {
            solution::OutType::Intent => OutType::Intent,
            solution::OutType::Receipt => OutType::Receipt,
        }
    }
}

impl From<solution::OutputIdx> for OutputIdx {
    fn from(oi: solution::OutputIdx) -> Self {
        OutputIdx {
            outType: oi.out_type.into(),
            outIdx: oi.out_idx,
        }
    }
}

impl From<solution::MoveRecord> for MoveRecord {
    fn from(mr: solution::MoveRecord) -> Self {
        MoveRecord {
            srcIdx: mr.src_idx,
            outputIdx: mr.output_idx.into(),
            qty: mr.qty,
        }
    }
}

impl From<solution::FillRecord> for FillRecord {
    fn from(fr: solution::FillRecord) -> Self {
        FillRecord {
            inIdx: fr.in_idx,
            outIdx: fr.out_idx,
            outType: fr.out_type.into(),
        }
    }
}

impl From<receipt::Receipt> for Receipt {
    fn from(r: receipt::Receipt) -> Self {
        Receipt {
            mToken: r.m_token,
            mTokenAmount: r.m_token_amount,
            owner: r.owner,
            intentHash: r.intent_hash,
        }
    }
}

impl From<intents::OutcomeAssetStructure> for OutcomeAssetStructure {
    fn from(oas: intents::OutcomeAssetStructure) -> Self {
        match oas {
            intents::OutcomeAssetStructure::AnySingle => OutcomeAssetStructure::AnySingle,
            intents::OutcomeAssetStructure::Any => OutcomeAssetStructure::Any,
            intents::OutcomeAssetStructure::All => OutcomeAssetStructure::All,
        }
    }
}

impl From<intents::FillStructure> for FillStructure {
    fn from(fs: intents::FillStructure) -> Self {
        match fs {
            intents::FillStructure::Exact => FillStructure::Exactly,
            intents::FillStructure::Minimum => FillStructure::Minimum,
            intents::FillStructure::PercentageFilled => FillStructure::PctFilled,
            intents::FillStructure::ConcreteRange => FillStructure::ConcreteRange,
        }
    }
}

impl From<intents::Outcome> for Outcome {
    fn from(o: intents::Outcome) -> Self {
        Outcome {
            mTokens: o.m_tokens,
            mAmounts: o.m_amounts,
            outcomeAssetStructure: o.outcome_asset_structure.into(),
            fillStructure: o.fill_structure.into(),
        }
    }
}

impl From<intents::Intent> for Intent {
    fn from(i: intents::Intent) -> Self {
        Intent {
            author: i.author,
            validBefore: i.valid_before,
            validAfter: i.valid_after,
            nonce: i.nonce,
            srcMToken: i.src_m_token,
            srcAmount: i.src_amount,
            outcome: i.outcome.into(),
        }
    }
}

pub fn eip712_domain(verifying_contract: Address) -> Eip712Domain {
    eip712_domain! {
        name: "KhalaniIntent".to_string(),
        version: "1.0.0".to_string(),
        verifying_contract: verifying_contract,
    }
}

pub fn eip712_intent_hash(intent: &RpcIntent, intent_book: Address) -> B256 {
    let domain = eip712_domain(intent_book);
    intent.convert_to_sol_type().eip712_signing_hash(&domain)
}

impl SolidityType for XChainEvent {}
impl SolidityType for AssetReserveDeposit {}
impl SolidityType for OutcomeAssetStructure {}
impl SolidityType for FillStructure {}
impl SolidityType for Outcome {}
impl SolidityType for Intent {}
impl SolidityType for SignedIntent {}
impl SolidityType for IntentState {}
impl SolidityType for Receipt {}
impl SolidityType for OutType {}
impl SolidityType for OutputIdx {}
impl SolidityType for MoveRecord {}
impl SolidityType for FillRecord {}
impl SolidityType for Solution {}
impl SolidityType for SignedSolution {}
