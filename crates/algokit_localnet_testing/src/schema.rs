//! Validates raw response bytes against the algod OpenAPI response schemas.
//!
//! This is the half of the harness that is independent of the generated client's serde decode: it
//! checks that the untouched JSON a node returned actually matches the schema declared in
//! `api/specs/algod.oas3.json`, catching drift between the spec, the generated client, and what a
//! real node emits.
//!
//! The whole spec document is registered once so internal `#/components/schemas/...` references
//! resolve, and one validator per schema is compiled eagerly the first time the harness is used.
//! After that the validator map is read-only and shared lock-free across parallel tests.

use std::collections::HashMap;
use std::sync::OnceLock;

use jsonschema::{Resource, Validator};
use serde_json::{Value, json};

/// Base URI the spec document is registered under so `$ref`s can point back into it.
const SPEC_BASE_URI: &str = "urn:algod-oas3";

/// Every response schema the endpoint tests validate against, by `components.schemas` name.
/// Add an entry here when a new endpoint test needs a schema that isn't compiled yet.
const RESPONSE_SCHEMAS: &[&str] = &[
    "NodeStatusResponse",
    "Genesis",
    "Version",
    "SupplyResponse",
    "TransactionParametersResponse",
    "Account",
    "AccountAssetResponse",
    "AccountApplicationResponse",
    "Asset",
    "Application",
    "BoxesResponse",
    "Box",
];

/// Lazily built, then read-only: one compiled validator per schema name.
static VALIDATORS: OnceLock<HashMap<String, Validator>> = OnceLock::new();

/// Absolute path to the algod OAS3 spec, resolved from this crate's manifest dir.
fn spec_path() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR = crates/algokit_localnet_testing; spec lives at <workspace>/api/specs.
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../api/specs/algod.oas3.json")
}

/// Parse the spec and compile a validator for each name in [`RESPONSE_SCHEMAS`].
fn build_validators() -> HashMap<String, Validator> {
    let spec_text = std::fs::read_to_string(spec_path()).expect("failed to read algod.oas3.json");
    let spec: Value = serde_json::from_str(&spec_text).expect("algod.oas3.json is not valid JSON");
    let resource = Resource::from_contents(spec).expect("spec is not a valid schema resource");

    let mut validators = HashMap::with_capacity(RESPONSE_SCHEMAS.len());
    for name in RESPONSE_SCHEMAS {
        let schema = json!({ "$ref": format!("{SPEC_BASE_URI}#/components/schemas/{name}") });
        let validator = jsonschema::options()
            .with_resource(SPEC_BASE_URI, resource.clone())
            .build(&schema)
            .unwrap_or_else(|e| panic!("failed to compile schema `{name}`: {e}"));
        validators.insert((*name).to_string(), validator);
    }
    validators
}

/// Validate `body` against the response schema named `schema_name` (a `components.schemas` key).
///
/// Returns `Err` with a readable message if the bytes aren't valid JSON, the schema isn't known, or
/// the JSON doesn't match the schema.
pub fn validate_response(schema_name: &str, body: &[u8]) -> Result<(), String> {
    let validators = VALIDATORS.get_or_init(build_validators);
    let validator = validators.get(schema_name).ok_or_else(|| {
        format!("unknown response schema `{schema_name}` (add it to RESPONSE_SCHEMAS)")
    })?;

    let instance: Value = serde_json::from_slice(body)
        .map_err(|e| format!("response body is not valid JSON: {e}"))?;

    validator
        .validate(&instance)
        .map_err(|e| format!("response does not match schema `{schema_name}`: {e}"))
}
