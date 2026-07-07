//! Seeds localnet with shared on-chain state and records the resulting ids in a manifest.
//!
//! Run once (via the `seed-localnet` binary / `cargo api seed-localnet`) before the endpoint tests:
//! it creates an asset, an app, and a confirmed payment, then writes their ids to a manifest the
//! tests read. Seeding also guarantees at least one block exists, which the block-dependent and
//! `/ready` tests rely on.

use std::path::PathBuf;

use algod_client::AlgodClient;
use algokit_transact::{
    AppCallTransactionBuilder, AssetConfigTransactionBuilder, OnApplicationComplete, StateSchema,
    TransactionHeaderBuilder,
};
use kmd_client::KmdClient;
use serde::{Deserialize, Serialize};

use crate::fixtures::kmd_account::{KmdAccount, dispenser_account};
use crate::fixtures::seeding::submit_and_confirm;

/// Ids of the state seeded into localnet, consumed by the endpoint tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// A funded account address controlled by the dispenser.
    pub address: String,
    /// Id of the created asset.
    pub asset_id: u64,
    /// Id of the created application.
    pub app_id: u64,
    /// Txid of the confirmed seed payment.
    pub txid: String,
    /// Round the seed payment confirmed in.
    pub confirmed_round: u64,
}

/// Where the manifest is written / read, relative to the workspace root.
pub fn manifest_path() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../algod_client/tests/.localnet-manifest.json")
}

/// Load the manifest, panicking with guidance if it is missing.
pub fn load_manifest() -> Manifest {
    let path = manifest_path();
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "localnet manifest not found at {}; run `cargo api seed-localnet` first",
            path.display()
        )
    });
    serde_json::from_str(&text).expect("localnet manifest is not valid JSON")
}

/// Seed localnet and write the manifest. Idempotent enough for repeated runs (each run creates fresh
/// state and overwrites the manifest).
pub async fn seed_localnet() -> Manifest {
    let algod = AlgodClient::localnet();
    let kmd = KmdClient::localnet();
    let dispenser = dispenser_account(&kmd, &algod).await;

    let asset_id = create_asset(&algod, &dispenser).await;
    let app_id = create_app(&algod, &dispenser).await;
    let (txid, confirmed_round) = seed_payment(&algod, &dispenser).await;

    let manifest = Manifest {
        address: dispenser.address.as_str(),
        asset_id,
        app_id,
        txid,
        confirmed_round,
    };

    let path = manifest_path();
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&manifest).expect("failed to serialize manifest"),
    )
    .unwrap_or_else(|e| panic!("failed to write manifest to {}: {e}", path.display()));

    manifest
}

/// Create a simple asset and return its id.
async fn create_asset(algod: &AlgodClient, dispenser: &KmdAccount) -> u64 {
    let header = header(algod, dispenser).await;
    let create = AssetConfigTransactionBuilder::default()
        .header(header)
        .asset_id(0)
        .total(1_000_000)
        .decimals(0)
        .unit_name("SEED".to_string())
        .asset_name("Seed Asset".to_string())
        .manager(dispenser.address.clone())
        .build()
        .expect("failed to build asset create");

    let txid = submit_and_confirm(algod, dispenser, create).await;
    algod
        .pending_transaction_information(&txid)
        .await
        .expect("failed to fetch asset create result")
        .asset_id
        .expect("asset create did not report an asset id")
}

/// Create a minimal always-approve app and return its id.
async fn create_app(algod: &AlgodClient, dispenser: &KmdAccount) -> u64 {
    let approval = compile_teal(algod, "#pragma version 8\nint 1\nreturn\n").await;
    let clear = compile_teal(algod, "#pragma version 8\nint 1\nreturn\n").await;

    let header = header(algod, dispenser).await;
    let create = AppCallTransactionBuilder::default()
        .header(header)
        .app_id(0)
        .on_complete(OnApplicationComplete::NoOp)
        .approval_program(approval)
        .clear_state_program(clear)
        .global_state_schema(StateSchema {
            num_uints: 0,
            num_byte_slices: 0,
        })
        .local_state_schema(StateSchema {
            num_uints: 0,
            num_byte_slices: 0,
        })
        .build()
        .expect("failed to build app create");

    let txid = submit_and_confirm(algod, dispenser, create).await;
    algod
        .pending_transaction_information(&txid)
        .await
        .expect("failed to fetch app create result")
        .app_id
        .expect("app create did not report an app id")
}

/// Submit a self-payment and return its (txid, confirmed round).
async fn seed_payment(algod: &AlgodClient, dispenser: &KmdAccount) -> (String, u64) {
    let payment =
        crate::fixtures::seeding::payment(algod, &dispenser.address, &dispenser.address, 0).await;
    let txid = submit_and_confirm(algod, dispenser, payment).await;
    let round = algod
        .pending_transaction_information(&txid)
        .await
        .expect("failed to fetch seed payment result")
        .confirmed_round
        .expect("seed payment not confirmed");
    (txid, round)
}

/// Compile TEAL source to program bytes via algod.
async fn compile_teal(algod: &AlgodClient, source: &str) -> Vec<u8> {
    algod
        .teal_compile(source.as_bytes().to_vec(), None)
        .await
        .expect("teal compile failed")
        .result
}

/// Build a transaction header from algod's current suggested params.
async fn header(
    algod: &AlgodClient,
    dispenser: &KmdAccount,
) -> algokit_transact::TransactionHeader {
    let params = algod
        .transaction_params()
        .await
        .expect("failed to fetch suggested params");
    TransactionHeaderBuilder::default()
        .sender(dispenser.address.clone())
        .fee(params.min_fee)
        .first_valid(params.last_round + 1)
        .last_valid(params.last_round + 1 + 1000)
        .genesis_hash(params.genesis_hash)
        .genesis_id(params.genesis_id)
        .build()
        .expect("failed to build transaction header")
}
