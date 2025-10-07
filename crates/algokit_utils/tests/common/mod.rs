#![allow(dead_code)]
#![allow(unused_imports)]
pub mod app_fixture;
pub mod fixture;
pub mod indexer_helpers;
pub mod local_net_dispenser;
pub mod logging;
pub mod mnemonic;
pub mod test_account;

use algokit_abi::Arc56Contract;
use algokit_utils::AppCreateParams;
use algokit_utils::applications::app_factory;
use algokit_utils::clients::app_manager::{
    AppManager, DeploymentMetadata, TealTemplateParams, TealTemplateValue,
};
use algokit_utils::config::{AppCompiledEventData, Config, EventData, EventType};
use base64::prelude::*;

pub use app_fixture::{
    AppFixture, AppFixtureOptions, AppFixtureResult, boxmap_app_fixture, boxmap_spec,
    build_app_fixture, default_teal_params, hello_world_app_fixture, hello_world_spec,
    nested_contract_fixture, sandbox_app_fixture, sandbox_spec, testing_app_fixture,
    testing_app_puya_fixture, testing_app_puya_spec, testing_app_spec,
};
pub use fixture::{AlgorandFixture, AlgorandFixtureResult, algorand_fixture};
pub use indexer_helpers::{
    IndexerWaitConfig, IndexerWaitError, wait_for_indexer, wait_for_indexer_transaction,
};
pub use local_net_dispenser::LocalNetDispenser;
pub use test_account::{NetworkType, TestAccount, TestAccountConfig};

pub type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn deploy_arc56_contract(
    fixture: &AlgorandFixture,
    sender: &algokit_transact::Address,
    arc56_contract: &Arc56Contract,
    template_params: Option<TealTemplateParams>,
    deploy_metadata: Option<DeploymentMetadata>,
    args: Option<Vec<Vec<u8>>>,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let teal_source = arc56_contract
        .source
        .clone()
        .expect("No source found in app spec");

    // Decode TEAL source (templates)
    let approval_src_bytes = BASE64_STANDARD.decode(teal_source.approval)?;
    let clear_src_bytes = BASE64_STANDARD.decode(teal_source.clear)?;
    let approval_teal = String::from_utf8(approval_src_bytes)?;
    let clear_teal = String::from_utf8(clear_src_bytes)?;

    // Compile via AppManager with substitution and source-map support
    let app_manager = AppManager::new(fixture.algod.clone());
    let approval_compile = app_manager
        .compile_teal_template(
            &approval_teal,
            template_params.as_ref(),
            deploy_metadata.as_ref(),
        )
        .await?;
    let clear_compile = app_manager
        .compile_teal_template(
            &clear_teal,
            template_params.as_ref(),
            deploy_metadata.as_ref(),
        )
        .await?;

    let app_create_params = AppCreateParams {
        sender: sender.clone(),
        args,
        approval_program: approval_compile.compiled_base64_to_bytes,
        clear_state_program: clear_compile.compiled_base64_to_bytes,
        global_state_schema: Some(algokit_transact::StateSchema {
            num_uints: arc56_contract.state.schema.global_state.ints,
            num_byte_slices: arc56_contract.state.schema.global_state.bytes,
        }),
        local_state_schema: Some(algokit_transact::StateSchema {
            num_uints: arc56_contract.state.schema.local_state.ints,
            num_byte_slices: arc56_contract.state.schema.local_state.bytes,
        }),
        ..Default::default()
    };

    let mut composer = fixture.algorand_client.new_composer(None);
    composer.add_app_create(app_create_params)?;

    let result = composer.send(None).await?;

    let app_id = result.results[0]
        .confirmation
        .app_id
        .expect("No app ID returned");

    Ok(app_id)
}
