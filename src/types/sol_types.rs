use alloy::primitives::{Address, B256};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolStruct, eip712_domain};
use serde::{Deserialize, Serialize};

use super::conversion::{RpcToSol, SolidityType};
use super::intents::Intent as RpcIntent;
use super::{intents, receipt, solution};

sol! {

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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CrossChainIntent {
        address author;
        uint256 validBefore;
        uint256 nonce;
        address srcMToken;
        uint256 srcAmount;
        uint32 destinationChainId;
        uint256 nativeOutcome;
        address outcomeToken;
        uint256 outcomeAmount;
    }

    #[derive(Debug)]
    enum IntentState {
        NonExistent,
        Open,
        Locked,
        Solved,
        Settled,
        Expired,
        Cancelled,
        Error
    }

    struct Receipt {
        address mToken;
        uint256 mTokenAmount;
        address owner;
        bytes32 intentHash;
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    struct FastWithdrawalPermit {
        uint256 nonce;
        uint32 spokeChainId;
        /// The spoke token address (not the mtoken address).
        address token;
        /// The amount in 18 decimals.
        uint256 amount;
        address user;
        address caller;
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
        error SolutionValidator__DuplicateIntentId(bytes32 intentId);
        error SolutionValidator__UnfilledInputIntent(uint256 intentIdx);
        error SolutionValidator__DuplicateOutputIntentNonce(uint256 intentIdx1, uint256 intentIdx2);
        error SolutionValidator__OrphanIntentOutput(uint256 intentIdx);
        error SolutionValidator__OrphanReceiptOutput(uint256 receiptIdx);
        error SolutionValidator__MoveRecordSrcIdxOutOfBounds(uint256 moveIdx);
        error SolutionValidator__FillRecordInIdxOutOfBounds(uint256 fillIdx);
        error SolutionValidator__IntentSpentAmountMismatch(uint256 intentIdx, uint256 moveIdx);
        error SolutionValidator__ReceiptMTokenMismatch(uint256 intentIdx);
        error SolutionValidator__InvalidMoveRecord(uint256 intentIdx, uint256 moveIdx);
        error SolutionValidator__ReceiptMTokenNotFoundInIntentOutcome(uint256 intentIdx, uint256 receiptIdx);
        error SolutionValidator__IntentTokenBurnt(uint256 intentIdx);
        error SolutionValidator__IntentTokenDoubleSpent(uint256 intentIdx);
        error SolutionValidator__IntentFillError__MultipleMTokensFilledForAnySingleIntent(uint256 intentIdx);
        error SolutionValidator__IntentFillError__NoMTokensFilledForAnySingleIntent(uint256 intentIdx);
        error SolutionValidator__MoveRecordOutputIdxOutOfBounds(uint256 moveIdx);
        error SolutionValidator__FillRecordOutputIdxOutOfBounds(uint256 fillIdx);
        error SolutionValidator__IntentMustSpendMToken(uint256 intentIdx);
        error SolutionValidator__PercentageIntentNotSatisfied(uint256 intentIdx, uint256 amtSpent, uint256 expectedSpentAmt);
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
        error IntentBook__CannotCancelNonOpenIntent(IntentState intentState);
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
        error InvalidNonce();

        // ReceiptManager errors.
        error ReceiptNotFound();
        error UnauthorizedRedemption();
        error UnauthorizedReceiptIssuance();
        error ReceiptAlreadyLocked();
        error ReceiptNotLocked();
        error UnauthorizedLockOperation();

        // SolutionLib errors.
        error SolutionLib__EmptyIntentsAndReceipts();
        error SolutionLib__InputOutputMismatch(uint256 sumInput, uint256 sumIntentOutput, uint256 sumReceiptOutput);
        error SolutionLib__SolutionMintsTokens();
        error SolutionLib__InputOutputTokenTypeMismatch(address inputMToken, address outputMToken);
        error SolutionLib__IntentAmountMismatch();
        error SolutionLib__ReceiptAmountMismatch();
        error SolutionLib__UnsupportedOutcomeStructure();
        error SolutionLib__UnsupportedFillStructure();
        error SolutionLib__InvalidFillGraphForEASIntent();
        error SolutionLib__ExactAnySingleMustBeFulfilledWithReceipts(FillRecord fillRec);
        error SolutionLib__MismatchBetweenInputAndOutputOwners();
        error SolutionLib__InsufficientReceiptsToFillPASIntent(uint256 expectedReceiptTotal, uint256 actualReceiptTotal);
        error SolutionLib__MinimumFillAmountNotSatisfied(uint256 minimumReceiptAmount, uint256 actualReceiptAmount);
        error SolutionLib__AnySingleExactlyIntentMustHaveExactlyOneOutcomeToken();
        error SolutionLib__AnySingleExactlyIntentMustHaveExactlyOneOutcomeTokenAmount();
        error Teller__InvalidInitializationParameters();
        error Teller__Paused();
        error Teller__InvalidMedusaAddress();
        error Teller__AssetNotSupported();
        error Teller__ZeroAmount();
        error Teller__CannotWithdrawAmount();
        error Teller__MinimumDepositShareAmountNotMet();
        error Teller__InsufficientSharesForWithdrawalFee();
        error Teller__CannotRemoveSupportedAssetWithNonZeroBalance();
        error Teller__DepositorDoesNotHaveEnoughShares(uint256 shares);
        error Teller__InvalidFeePercentage(uint16 feePercentage);
        error Teller__CannotWithdrawZeroShares();
        error Teller__InvalidRate();
        error MTokenVault__InsufficientBalance(address asset, uint256 balance);
        error MTokenVault__CannotEnterZeroAmount();
        error MTokenVault__CannotEnterWithoutMintingShares();
        error MTokenVault__CannotExitZeroAmount();
        error MTokenVault__CannotExitWithoutBurningShares();
    }

    #[sol(rpc)]
    contract IntentBook {
        event IntentCreated(
            bytes32 indexed intentId,
            Intent intent,
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
        function fastWithdrawMToken(FastWithdrawalPermit calldata permit, bytes calldata userSignature) external;
        function fastWithdrawMTokenWithWitness(FastWithdrawalPermit calldata permit, bytes calldata userSignature, string calldata witnessTypeString, bytes32 witness) external;
        function getBalanceOfUser(address user, address mToken) external view returns (uint256);
        function setVaultManager(address manager, bool isVaultManager) external;
        function runMTokenVault(address manager, address rateProvider, string memory name, string memory symbol) external;
        event MTokenVaultCreated(address indexed managerAddress, address indexed vaultAddress, address indexed tellerAddress);

        mapping(bytes32 => bool) public signedWithdrawalPermits;
    }

    #[sol(rpc)]
    contract Teller {
        function deposit(address depositor, address asset, uint256 amount, uint256 minShares) external returns (uint256 shares);
        function previewDeposit(address asset, uint256 amount) external view returns (uint256 shares);
        function maximumWithdraw(address asset, uint256 shares, uint16 feePercentage) external view returns (uint256 amount);
        function withdraw(uint256 shares, address depositor, address asset, uint256 minAmount, uint16 feePercentage) external;
        function pause() external;
        function unpause() external;
        function owner() external view returns (address);
        function getTotalShares() external view returns (uint256);
        function getTotalAssetValue() external view returns (uint256);
        function mtokenBalanceof(address mToken) external view returns (uint256);
        function mtokenBalanceInIntent(address mToken) public view returns (uint256);
        function getDepositorShares(address depositor) external view returns (uint256);
        function getAllSupportedAssets() public view returns (address[] memory);
        function addSupportedAsset(address asset) external;
        function removeSupportedAsset(address asset) external;
    }

    #[sol(rpc)]
    contract MToken {
        function approve(address spender, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function decimals() external view returns (uint8);

    }

    #[sol(rpc)]
    contract ERC20 {
        function allowance(address owner, address spender) external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function decimals() external view returns (uint8);
    }

    #[sol(rpc)]
    contract AssetReserves {
        function withdrawWithPermit(
            FastWithdrawalPermit calldata permit,
            address receiver,
            bytes calldata userSignature,
            bytes calldata operatorSignature
        ) external;

        function withdrawWithPermitAndWitness(
            FastWithdrawalPermit calldata permit,
            address receiver,
            bytes32 witness,
            string calldata witnessTypeString,
            bytes calldata userSignature,
            bytes calldata operatorSignature
        ) external;
        function deposit(address token, uint256 amount, uint32 destChain) external payable;
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

impl FastWithdrawalPermit {
    pub async fn sign(
        self,
        signer: &(impl alloy::signers::Signer + Send + Sync),
        chain_id: u64,
        mtoken_manager: Address,
    ) -> Result<alloy::primitives::Bytes, alloy::signers::Error> {
        let domain = eip712_domain! {
            name: "FastWithdrawalPermit".to_string(),
            version: "1".to_string(),
            chain_id: chain_id,
            verifying_contract: mtoken_manager,
        };
        let hash = self.eip712_signing_hash(&domain);
        let signature = signer.sign_hash(&hash).await?;
        Ok(signature.as_bytes().to_vec().into())
    }
}
