//! Unit tests for the generated kmd client models.
//!
//! Assert that `x-algorand-format: "Address"` fields map to
//! `algokit_transact::Address` and round-trip. No network access — pure serde.
//!
//! Lives under `tests/` (outside the generated `src/` tree) so it survives
//! `cargo api generate-kmd`.

use algokit_transact::Address;
use kmd_client::models::{ExportKeyRequest, GenerateKeyResponse};

/// `GenerateKeyResponse.address` is a typed `Address`.
#[test]
fn generate_key_response_uses_address() {
    let value = GenerateKeyResponse {
        address: Address::new([2u8; 32]),
    };
    let parsed: GenerateKeyResponse =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed.address, Address::new([2u8; 32]));
}

/// `ExportKeyRequest.address` is a typed `Address`; optional password stays optional.
#[test]
fn export_key_request_uses_address() {
    let value = ExportKeyRequest {
        address: Address::new([4u8; 32]),
        wallet_handle_token: "handle".to_string(),
        wallet_password: None,
    };
    let parsed: ExportKeyRequest =
        serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
    assert_eq!(parsed.address, Address::new([4u8; 32]));
    assert_eq!(parsed.wallet_handle_token, "handle");
    assert!(parsed.wallet_password.is_none());
}
