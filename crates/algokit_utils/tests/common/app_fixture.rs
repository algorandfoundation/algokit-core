use crate::common::{
    AlgorandFixture, AlgorandFixtureResult, algorand_fixture, deploy_arc56_contract,
};
use algokit_abi::Arc56Contract;
use algokit_transact::Address;
use algokit_utils::AlgorandClient;
use algokit_utils::ResourcePopulation;
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::clients::app_manager::{
    DeploymentMetadata, TealTemplateParams, TealTemplateValue,
};
use algokit_utils::transactions::TransactionComposerConfig;
use rstest::fixture;
use std::sync::Arc;

pub struct AppFixture {
    pub algorand_fixture: AlgorandFixture,
    pub sender_address: Address,
    pub app_id: u64,
    pub app_spec: Arc56Contract,
    pub client: AppClient,
}

pub type AppFixtureResult = Result<AppFixture, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Default)]
pub struct AppFixtureOptions {
    pub template_params: Option<TealTemplateParams>,
    pub deploy_metadata: Option<DeploymentMetadata>,
    pub args: Option<Vec<Vec<u8>>>,
    pub transaction_composer_config: Option<TransactionComposerConfig>,
    pub default_sender_override: Option<String>,
    pub app_name: Option<String>,
}

pub async fn build_app_fixture(
    fixture: AlgorandFixture,
    spec: Arc56Contract,
    opts: AppFixtureOptions,
) -> AppFixtureResult {
    let sender = fixture.test_account.account().address();

    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &spec,
        opts.template_params.clone(),
        opts.deploy_metadata.clone(),
        opts.args.clone(),
    )
    .await?;

    let mut algorand = AlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id,
        app_spec: spec.clone(),
        algorand: algorand.into(),
        app_name: opts.app_name.clone(),
        default_sender: Some(
            opts.default_sender_override
                .unwrap_or_else(|| sender.to_string()),
        ),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: opts.transaction_composer_config,
    });

    Ok(AppFixture {
        algorand_fixture: fixture,
        sender_address: sender,
        app_id,
        app_spec: spec,
        client,
    })
}

pub fn default_teal_params(value: u64, updatable: bool, deletable: bool) -> TealTemplateParams {
    let mut t = TealTemplateParams::default();
    t.insert("VALUE".to_string(), TealTemplateValue::Int(value));
    t.insert(
        "UPDATABLE".to_string(),
        TealTemplateValue::Int(if updatable { 1 } else { 0 }),
    );
    t.insert(
        "DELETABLE".to_string(),
        TealTemplateValue::Int(if deletable { 1 } else { 0 }),
    );
    t
}

// ARC56 contract specs for test apps
pub fn testing_app_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::testing_app::APPLICATION_ARC56).unwrap()
}

pub fn nested_contract_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::nested_contract::APPLICATION_ARC56).unwrap()
}

pub fn sandbox_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::sandbox::APPLICATION_ARC56).unwrap()
}

pub fn hello_world_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::hello_world::APPLICATION_ARC56).unwrap()
}

pub fn boxmap_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::box_map_test::APPLICATION_ARC56).unwrap()
}

pub fn testing_app_puya_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::testing_app_puya::APPLICATION_ARC56).unwrap()
}

// Common fixtures for app_client tests
#[fixture]
pub async fn testing_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = testing_app_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            template_params: Some(default_teal_params(0, false, false)),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn nested_contract_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = nested_contract_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            args: Some(vec![vec![184u8, 68u8, 123u8, 54u8]]),
            transaction_composer_config: Some(TransactionComposerConfig {
                populate_app_call_resources: ResourcePopulation::Enabled {
                    use_access_list: false,
                },
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn sandbox_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = sandbox_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            template_params: Some(default_teal_params(0, false, false)),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn hello_world_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = hello_world_spec();
    build_app_fixture(f, spec, AppFixtureOptions::default()).await
}

#[fixture]
pub async fn boxmap_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = boxmap_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            args: Some(vec![vec![184u8, 68u8, 123u8, 54u8]]),
            transaction_composer_config: Some(TransactionComposerConfig {
                populate_app_call_resources: ResourcePopulation::Enabled {
                    use_access_list: false,
                },
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn testing_app_puya_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = testing_app_puya_spec();
    build_app_fixture(f, spec, AppFixtureOptions::default()).await
}
