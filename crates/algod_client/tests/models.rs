//! Unit tests for the generated algod client models.
//!
//! These assert that the vendor-extension-driven types the generator now emits
//! (`algokit_transact::Address`, fixed `[u8; N]` byte arrays, and the
//! box/holding/locals reference structs) deserialize and round-trip correctly.
//! No network access — pure serde.
//!
//! Lives under `tests/` (outside the generated `src/` tree) so it survives
//! `cargo api generate-algod`.

use algod_client::models::{
    AccountParticipation, ApplicationLocalReference, AssetHoldingReference, BoxReference,
};
use algokit_transact::Address;

/// `[u8; 32]` participation keys decode from base64 strings and round-trip.
#[test]
fn account_participation_fixed_bytes_roundtrip() {
    let b64 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
    let json = format!(
        r#"{{"selection-participation-key":"{b64}","vote-first-valid":1,"vote-key-dilution":2,"vote-last-valid":3,"vote-participation-key":"{b64}"}}"#
    );

    let parsed: AccountParticipation = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.selection_participation_key.len(), 32);
    assert_eq!(parsed.vote_participation_key.len(), 32);
    assert_eq!(parsed.vote_first_valid, 1);

    let reparsed: AccountParticipation =
        serde_json::from_str(&serde_json::to_string(&parsed).unwrap()).unwrap();
    assert_eq!(parsed, reparsed);
}

/// The optional state-proof key is a fixed `[u8; 64]`.
#[test]
fn account_participation_optional_fixed_64() {
    assert!(AccountParticipation::default().state_proof_key.is_none());

    let with_key = AccountParticipation {
        state_proof_key: Some([9u8; 64]),
        ..Default::default()
    };
    let reparsed: AccountParticipation =
        serde_json::from_str(&serde_json::to_string(&with_key).unwrap()).unwrap();
    assert_eq!(reparsed.state_proof_key, Some([9u8; 64]));
}

/// `x-algorand-format: "Address"` maps to `algokit_transact::Address`.
#[test]
fn application_local_reference_uses_address() {
    let account = Address::new([0u8; 32]);
    let value = ApplicationLocalReference {
        account: account.clone(),
        app: 42,
    };

    let parsed: ApplicationLocalReference =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed.account, account);
    assert_eq!(parsed.app, 42);
}

/// Holding reference also carries a typed `Address` account.
#[test]
fn asset_holding_reference_uses_address() {
    let value = AssetHoldingReference {
        account: Address::new([1u8; 32]),
        asset: 7,
    };
    let parsed: AssetHoldingReference =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed, value);
}

/// Box reference: app id + raw byte name.
#[test]
fn box_reference_roundtrip() {
    let value = BoxReference {
        app: 5,
        name: vec![1, 2, 3, 4],
    };
    let parsed: BoxReference =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed, value);
    assert_eq!(parsed.name, vec![1, 2, 3, 4]);
}

/// A logic signature inside a block-embedded transaction must reach the flattened
/// `algokit_transact::SignedTransaction`, not a competing field on the outer struct.
#[test]
fn signed_txn_in_block_carries_a_logic_signature() {
    use algod_client::models::SignedTxnInBlock;

    let mut stxn = SignedTxnInBlock::new();
    stxn.signed_transaction.logic_signature =
        Some(algokit_transact::LogicSignature::new(vec![1, 32, 1, 1, 34]));

    let encoded = rmp_serde::to_vec_named(&stxn).unwrap();
    let decoded: SignedTxnInBlock = rmp_serde::from_slice(&encoded).unwrap();

    assert_eq!(
        decoded.signed_transaction.logic_signature,
        stxn.signed_transaction.logic_signature
    );
}
