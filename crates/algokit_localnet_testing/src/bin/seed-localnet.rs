//! Seeds a running localnet with the shared state the algod endpoint tests query, then writes the
//! manifest. Run once before the ignored endpoint tests:
//!
//! ```sh
//! algokit localnet start
//! cargo run -p algokit_localnet_testing --bin seed-localnet
//! ```

#[tokio::main]
async fn main() {
    let manifest = algokit_localnet_testing::seed_localnet().await;
    println!(
        "seeded localnet: asset_id={} app_id={} box_app_id={} round={} txid={}",
        manifest.asset_id,
        manifest.app_id,
        manifest.box_app_id,
        manifest.confirmed_round,
        manifest.txid
    );
}
