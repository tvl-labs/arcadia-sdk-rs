use alloy::signers::Signature;

pub struct Intent {}

pub struct SignedIntent {
    intent: Intent,
    signature: Signature,
}
