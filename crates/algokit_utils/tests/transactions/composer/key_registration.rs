use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};
use algokit_utils::{
    CommonParams, NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use base64::{Engine, engine::general_purpose};
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_offline_key_registration_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_addr = algorand_fixture.test_account.account().address();

    let offline_key_reg_params = OfflineKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        non_participation: Some(false),
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_offline_key_registration(offline_key_reg_params)?;

    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];
    // Assert transaction was confirmed
    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::KeyRegistration(key_reg_fields) => {
            assert!(
                key_reg_fields.vote_key.is_none(),
                "Vote key should be None for offline registration"
            );
            assert!(
                key_reg_fields.selection_key.is_none(),
                "Selection key should be None for offline registration"
            );
            assert!(
                key_reg_fields.non_participation.is_none(),
                "Non participation should be None for offline registration"
            );
        }
        _ => return Err("Transaction should be a key registration transaction".into()),
    }

    // Verify account participation status
    let account_info = algorand_fixture
        .algod
        .account_information(&sender_addr.to_string(), None, None)
        .await?;

    // For offline registration, participation should be empty/none
    assert!(
        account_info.participation.is_none()
            || account_info
                .participation
                .as_ref()
                .is_none_or(|p| p.vote_participation_key.is_empty()),
        "Account should not have participation keys after going offline"
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_non_participation_key_registration_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_addr = algorand_fixture.test_account.account().address();

    // Use real participation keys for initial online registration
    let vote_key = general_purpose::STANDARD
        .decode("G/lqTV6MKspW6J8wH2d8ZliZ5XZVZsruqSBJMwLwlmo=")?
        .try_into()
        .map_err(|_| "Vote key should be 32 bytes")?;

    let selection_key = general_purpose::STANDARD
        .decode("LrpLhvzr+QpN/bivh6IPpOaKGbGzTTB5lJtVfixmmgk=")?
        .try_into()
        .map_err(|_| "Selection key should be 32 bytes")?;

    let state_proof_key = general_purpose::STANDARD.decode(
        "RpUpNWfZMjZ1zOOjv3MF2tjO714jsBt0GKnNsw0ihJ4HSZwci+d9zvUi3i67LwFUJgjQ5Dz4zZgHgGduElnmSA==",
    )?
    .try_into()
    .map_err(|_| "State proof key should be 64 bytes")?;

    // Step 1: First make the account online to demonstrate the permanent nature of non-participation
    let params1 = algorand_fixture.algod.transaction_params().await?;

    let vote_first = params1.last_round;
    let vote_last = vote_first + 10_000_000;

    let online_key_reg_params = OnlineKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        vote_key,
        selection_key,
        vote_first,
        vote_last,
        vote_key_dilution: 100,
        state_proof_key: Some(state_proof_key),
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_online_key_registration(online_key_reg_params)?;

    let online_result = composer.send(None).await?;

    assert!(
        online_result.confirmations[0].confirmed_round.is_some(),
        "Online transaction should be confirmed"
    );

    // Verify account is now online
    let account_info = algorand_fixture
        .algod
        .account_information(&sender_addr.to_string(), None, None)
        .await?;

    assert!(
        account_info.participation.is_some(),
        "Account should have participation information after going online"
    );

    // Step 2: Mark account as permanently non-participating
    let non_participation_params = NonParticipationKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
    };

    let mut composer2 = algorand_fixture.algorand_client.new_group();
    composer2.add_non_participation_key_registration(non_participation_params)?;

    let result = composer2.send(None).await?;
    let confirmation = &result.confirmations[0];

    // Assert transaction was confirmed
    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::KeyRegistration(key_reg_fields) => {
            assert!(
                key_reg_fields.vote_key.is_none(),
                "Vote key should be None for non participation"
            );
            assert!(
                key_reg_fields.selection_key.is_none(),
                "Selection key should be None for non participation"
            );
            assert_eq!(
                key_reg_fields.non_participation,
                Some(true),
                "Non participation should be true"
            );
        }
        _ => return Err("Transaction should be a key registration transaction".into()),
    }

    // Verify account participation status
    let account_info = algorand_fixture
        .algod
        .account_information(&sender_addr.to_string(), None, None)
        .await?;

    // For non-participation, participation should be empty/none
    assert!(
        account_info.participation.is_none()
            || account_info
                .participation
                .as_ref()
                .is_none_or(|p| p.vote_participation_key.is_empty()),
        "Account should not have participation keys after non-participation registration"
    );

    // Step 3: Verify that once marked as non-participating, account cannot be brought back online
    let params3 = algorand_fixture.algod.transaction_params().await?;

    let vote_first_3 = params3.last_round;
    let vote_last_3 = vote_first_3 + 10_000_000;

    let try_online_again_params = OnlineKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        vote_key,
        selection_key,
        vote_first: vote_first_3,
        vote_last: vote_last_3,
        vote_key_dilution: 100,
        state_proof_key: Some(state_proof_key),
    };

    let mut composer3 = algorand_fixture.algorand_client.new_group();
    composer3.add_online_key_registration(try_online_again_params)?;

    // This should fail because the account is permanently marked as non-participating
    let online_again_result = composer3.send(None).await;

    assert!(
        online_again_result.is_err(),
        "Attempting to bring a non-participating account back online should fail"
    );

    // Verify the error is related to the account being marked as non-participating
    let error_message = online_again_result.unwrap_err().to_string();
    assert!(
        error_message.contains("nonparticipatory")
            || error_message.contains("non-participating")
            || error_message.contains("nonpart")
            || error_message.contains("pool error")
            || error_message.contains("rejected"),
        "Error should indicate the account cannot participate: {}",
        error_message
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_online_key_registration_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_addr = algorand_fixture.test_account.account().address();

    let vote_key = general_purpose::STANDARD
        .decode("G/lqTV6MKspW6J8wH2d8ZliZ5XZVZsruqSBJMwLwlmo=")?
        .try_into()
        .map_err(|_| "Vote key should be 32 bytes")?;

    let selection_key = general_purpose::STANDARD
        .decode("LrpLhvzr+QpN/bivh6IPpOaKGbGzTTB5lJtVfixmmgk=")?
        .try_into()
        .map_err(|_| "Selection key should be 32 bytes")?;

    let state_proof_key = general_purpose::STANDARD.decode(
        "RpUpNWfZMjZ1zOOjv3MF2tjO714jsBt0GKnNsw0ihJ4HSZwci+d9zvUi3i67LwFUJgjQ5Dz4zZgHgGduElnmSA==",
    )?
    .try_into()
    .map_err(|_| "State proof key should be 64 bytes")?;

    // Get fresh suggested params to use proper voting rounds
    let params = algorand_fixture.algod.transaction_params().await?;

    let vote_first = params.last_round;
    let vote_last = vote_first + 10_000_000;

    let online_key_reg_params = OnlineKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        vote_key,
        selection_key,
        vote_first,
        vote_last,
        vote_key_dilution: 100,
        state_proof_key: Some(state_proof_key),
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_online_key_registration(online_key_reg_params)?;

    // Submit the transaction - should succeed with proper keys and voting rounds
    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];

    // Assert transaction was confirmed
    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::KeyRegistration(key_reg_fields) => {
            assert_eq!(
                key_reg_fields.vote_key,
                Some(vote_key),
                "Vote key should match"
            );
            assert_eq!(
                key_reg_fields.selection_key,
                Some(selection_key),
                "Selection key should match"
            );
            assert_eq!(
                key_reg_fields.vote_first,
                Some(vote_first),
                "Vote first should match"
            );
            assert_eq!(
                key_reg_fields.vote_last,
                Some(vote_last),
                "Vote last should match"
            );
            assert_eq!(
                key_reg_fields.vote_key_dilution,
                Some(100),
                "Vote key dilution should match"
            );
            assert_eq!(
                key_reg_fields.state_proof_key,
                Some(state_proof_key),
                "State proof key should match"
            );
            assert!(
                key_reg_fields.non_participation.is_none(),
                "Non participation should be None for online registration"
            );
        }
        _ => return Err("Transaction should be a key registration transaction".into()),
    }

    // Verify account participation status
    let account_info = algorand_fixture
        .algod
        .account_information(&sender_addr.to_string(), None, None)
        .await?;

    // For online registration, participation should contain the keys
    if let Some(participation) = account_info.participation {
        assert!(
            !participation.vote_participation_key.is_empty(),
            "Account should have participation keys after going online"
        );

        // Verify the participation keys match what we submitted
        assert_eq!(
            participation.vote_participation_key,
            vote_key.to_vec(),
            "Vote participation key should match submitted key"
        );
    } else {
        return Err(
            "Account should have participation information after online key registration".into(),
        );
    }

    Ok(())
}
