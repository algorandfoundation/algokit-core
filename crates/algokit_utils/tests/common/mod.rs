#![allow(dead_code)]
#![allow(unused_imports)]
pub mod fixture;
pub mod indexer_helpers;
pub mod local_net_dispenser;
pub mod logging;
pub mod mnemonic;
pub mod test_account;

use algokit_abi::Arc56Contract;
use algokit_transact::Address;
use algokit_utils::{AppCreateParams, CommonParams};
use base64::prelude::*;

pub use fixture::{AlgorandFixture, AlgorandFixtureResult, algorand_fixture};
pub use indexer_helpers::{
    IndexerWaitConfig, IndexerWaitError, wait_for_indexer, wait_for_indexer_transaction,
};
pub use local_net_dispenser::LocalNetDispenser;
pub use test_account::{NetworkType, TestAccount, TestAccountConfig};

pub type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn deploy_arc56_contract(
    fixture: &AlgorandFixture,
    sender: &Address,
    arc56_contract: &Arc56Contract,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let teal_source = arc56_contract
        .source
        .clone()
        .expect("No source found in app spec");

    let approval_bytes = BASE64_STANDARD.decode(teal_source.approval)?;

    let clear_state_bytes = BASE64_STANDARD.decode(teal_source.clear)?;

    let approval_compile_result = fixture.algod.teal_compile(approval_bytes, None).await?;
    let clear_state_compile_result = fixture.algod.teal_compile(clear_state_bytes, None).await?;

    let app_create_params = AppCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        approval_program: approval_compile_result.result,
        clear_state_program: clear_state_compile_result.result,
        ..Default::default()
    };

    let mut composer = fixture.algorand_client.new_group();
    composer.add_app_create(app_create_params)?;

    let result = composer.send(None).await?;

    let app_id = result.confirmations[0].app_id.expect("No app ID returned");

    Ok(app_id)
}
