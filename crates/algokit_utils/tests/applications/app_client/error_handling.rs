use crate::common::TestResult;
use crate::common::app_fixture::testing_app_fixture;
use algokit_utils::applications::app_client::{AppClientError, AppClientMethodCallParams};
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_exposing_logic_error_without_sourcemaps(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let error_response = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "error".to_string(),
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .expect_err("expected logic error");

    if let AppClientError::LogicError { logic, .. } = &error_response {
        assert!(logic.message.contains("assert failed pc=885"));
    }

    Ok(())
}

// NOTE: more comprehensive version with source maps will be added in app factory pr
