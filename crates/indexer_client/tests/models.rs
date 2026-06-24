//! Unit tests for the generated indexer client models.
//!
//! Assert that the vendor-extension-driven types the generator now emits
//! (`algokit_transact::Address`, fixed `[u8; N]` byte arrays) deserialize and
//! round-trip correctly. No network access — pure serde.
//!
//! Lives under `tests/` (outside the generated `src/` tree) so it survives
//! `cargo api generate-indexer`.

use algokit_transact::Address;
use indexer_client::models::{HoldingRef, StateProofVerifier};

/// `x-algokit-byte-length: 64` produces a fixed `Option<[u8; 64]>` that decodes
/// from a base64 string and round-trips.
#[test]
fn state_proof_verifier_fixed_64_roundtrip() {
    let value = StateProofVerifier {
        commitment: Some([3u8; 64]),
        key_lifetime: Some(256),
    };
    let parsed: StateProofVerifier =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed.commitment, Some([3u8; 64]));
    assert_eq!(parsed.key_lifetime, Some(256));
}

/// Omitted optional fixed-byte field stays `None`.
#[test]
fn state_proof_verifier_optional_absent() {
    let parsed: StateProofVerifier = serde_json::from_str("{}").unwrap();
    assert!(parsed.commitment.is_none());
    assert!(parsed.key_lifetime.is_none());
}

/// `x-algorand-format: "Address"` maps to `algokit_transact::Address`.
#[test]
fn holding_ref_uses_address() {
    let value = HoldingRef {
        address: Address::new([5u8; 32]),
        asset: 99,
    };
    let parsed: HoldingRef = serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed, value);
    assert_eq!(parsed.address, Address::new([5u8; 32]));
}
