use crate::common::TestResult;
use crate::common::app_fixture::testing_app_fixture;
use algokit_utils::applications::app_client::CompilationParams;
use algokit_utils::clients::app_manager::TealTemplateValue;
use algokit_utils::config::{AppCompiledEventData, EventData, EventType};
use rstest::*;

#[rstest]
#[tokio::test]
async fn compile_applies_template_params_and_emits_event(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    // Use an app name to assert AppCompiled event has a name
    let f = testing_app_fixture.await?;
    algokit_utils::config::Config::configure(Some(true), None);
    let mut events = algokit_utils::config::Config::events().subscribe();
    let client = f.client;

    let compilation_params = CompilationParams {
        deploy_time_params: Some(
            [("VALUE", 1), ("UPDATABLE", 0), ("DELETABLE", 0)]
                .into_iter()
                .map(|(k, v)| (k.to_string(), TealTemplateValue::Int(v)))
                .collect(),
        ),
        updatable: Some(false),
        deletable: Some(false),
    };
    client.compile(&compilation_params).await?;

    if let Ok((event_type, data)) =
        tokio::time::timeout(std::time::Duration::from_millis(5000), events.recv()).await?
    {
        assert_eq!(event_type, EventType::AppCompiled);
        match data {
            EventData::AppCompiled(AppCompiledEventData {
                app_name,
                approval_source_map,
                clear_source_map,
            }) => {
                assert!(app_name.is_none() || app_name.as_deref() == Some("TestingApp"));
                assert!(approval_source_map.is_some());
                assert!(clear_source_map.is_some());
            }
            _ => return Err("unexpected event data".into()),
        }
    } else {
        return Err("expected AppCompiled event".into());
    }

    Ok(())
}
