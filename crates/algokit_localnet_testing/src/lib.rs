//! Live localnet test support shared across the generated API client crates.
//!
//! Kept as a standalone crate (not the low-level [`algokit_test_artifacts`] static-artifacts crate)
//! so it can depend on the client crates without dragging them into foundational crates' test builds.

/// Localnet fixtures (KMD accounts, transaction seeding).
pub mod fixtures;
/// An HTTP client that records raw response bytes.
pub mod http_capture;
/// Validates response bytes against the algod OpenAPI response schemas.
pub mod schema;
/// Seeds localnet with shared state and reads/writes the test manifest.
pub mod seed;

pub use fixtures::LocalnetFixture;
pub use fixtures::kmd_account::KmdAccount;
pub use http_capture::CapturingHttpClient;
pub use schema::validate_response;
pub use seed::{Manifest, load_manifest, seed_localnet};

/// Serializes tests that mutate shared node state. Hold the guard for the whole test.
pub fn state_lock() -> std::sync::MutexGuard<'static, ()> {
    static STATE_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
    STATE_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod schema_selftest {
    use super::validate_response;

    /// A minimal object matching `NodeStatusResponse`'s required integer fields validates.
    #[test]
    fn valid_status_response_passes() {
        let body = br#"{
            "catchup-time": 0,
            "last-round": 42,
            "last-version": "v1",
            "next-version": "v2",
            "next-version-round": 43,
            "next-version-supported": true,
            "stopped-at-unsupported-round": false,
            "time-since-last-round": 100
        }"#;
        assert!(
            validate_response("NodeStatusResponse", body).is_ok(),
            "a well-formed status response should validate"
        );
    }

    /// Wrong types for required fields are rejected.
    #[test]
    fn malformed_status_response_fails() {
        let body = br#"{ "last-round": "not-a-number" }"#;
        assert!(
            validate_response("NodeStatusResponse", body).is_err(),
            "a malformed status response should not validate"
        );
    }

    /// An unknown schema name is a clear error, not a silent pass.
    #[test]
    fn unknown_schema_is_an_error() {
        assert!(validate_response("NoSuchSchema", b"{}").is_err());
    }
}
