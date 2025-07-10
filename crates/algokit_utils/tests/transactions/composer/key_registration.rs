use algokit_utils::testing::*;
use algokit_utils::{
    CommonParams, NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use base64::{Engine, engine::general_purpose};

use crate::common::init_test_logging;

#[tokio::test]
async fn test_offline_key_registration_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender address")
        .address();

    let offline_key_reg_params = OfflineKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        non_participation: Some(false),
    };

    let mut composer = context.composer.clone();
    composer
        .add_offline_key_registration(offline_key_reg_params)
        .expect("Failed to add offline key registration");

    let result = composer
        .send()
        .await
        .expect("Failed to send offline key registration");

    // Assert transaction was confirmed
    assert!(
        result.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        result.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = result.txn.transaction;

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
        _ => panic!("Transaction should be a key registration transaction"),
    }

    // Verify account participation status
    let account_info = context
        .algod
        .account_information(None, &sender_addr.to_string(), None)
        .await
        .expect("Failed to get account information");

    // For offline registration, participation should be empty/none
    assert!(
        account_info.participation.is_none()
            || account_info
                .participation
                .as_ref()
                .is_none_or(|p| p.vote_participation_key.is_empty()),
        "Account should not have participation keys after going offline"
    );
}

#[tokio::test]
async fn test_non_participation_key_registration_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender address")
        .address();

    // Use real participation keys for initial online registration
    let vote_key = general_purpose::STANDARD
        .decode("G/lqTV6MKspW6J8wH2d8ZliZ5XZVZsruqSBJMwLwlmo=")
        .expect("Failed to decode vote key")
        .try_into()
        .expect("Vote key should be 32 bytes");

    let selection_key = general_purpose::STANDARD
        .decode("LrpLhvzr+QpN/bivh6IPpOaKGbGzTTB5lJtVfixmmgk=")
        .expect("Failed to decode selection key")
        .try_into()
        .expect("Selection key should be 32 bytes");

    let state_proof_key = general_purpose::STANDARD.decode(
        "RpUpNWfZMjZ1zOOjv3MF2tjO714jsBt0GKnNsw0ihJ4HSZwci+d9zvUi3i67LwFUJgjQ5Dz4zZgHgGduElnmSA==",
    )
    .expect("Failed to decode state proof key")
    .try_into()
    .expect("State proof key should be 64 bytes");

    // Step 1: First make the account online to demonstrate the permanent nature of non-participation
    let params1 = context
        .algod
        .transaction_params()
        .await
        .expect("Failed to get transaction params");

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

    let mut composer = context.composer.clone();
    composer
        .add_online_key_registration(online_key_reg_params)
        .expect("Failed to add online key registration");

    let online_result = composer
        .send()
        .await
        .expect("Failed to send online key registration");

    assert!(
        online_result.confirmed_round.is_some(),
        "Online transaction should be confirmed"
    );

    // Verify account is now online
    let account_info = context
        .algod
        .account_information(None, &sender_addr.to_string(), None)
        .await
        .expect("Failed to get account information");

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

    let mut composer2 = context.composer.clone();
    composer2
        .add_non_participation_key_registration(non_participation_params)
        .expect("Failed to add non participation key registration");

    let result = composer2
        .send()
        .await
        .expect("Failed to send non participation key registration");

    // Assert transaction was confirmed
    assert!(
        result.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        result.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = result.txn.transaction;

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
        _ => panic!("Transaction should be a key registration transaction"),
    }

    // Verify account participation status
    let account_info = context
        .algod
        .account_information(None, &sender_addr.to_string(), None)
        .await
        .expect("Failed to get account information");

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
    let params3 = context
        .algod
        .transaction_params()
        .await
        .expect("Failed to get transaction params");

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

    let mut composer3 = context.composer.clone();
    composer3
        .add_online_key_registration(try_online_again_params)
        .expect("Failed to add second online key registration");

    // This should fail because the account is permanently marked as non-participating
    let online_again_result = composer3.send().await;

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
}

#[tokio::test]
async fn test_online_key_registration_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender address")
        .address();

    // Use real participation keys from the Python test
    let vote_key = general_purpose::STANDARD
        .decode("G/lqTV6MKspW6J8wH2d8ZliZ5XZVZsruqSBJMwLwlmo=")
        .expect("Failed to decode vote key")
        .try_into()
        .expect("Vote key should be 32 bytes");

    let selection_key = general_purpose::STANDARD
        .decode("LrpLhvzr+QpN/bivh6IPpOaKGbGzTTB5lJtVfixmmgk=")
        .expect("Failed to decode selection key")
        .try_into()
        .expect("Selection key should be 32 bytes");

    let state_proof_key = general_purpose::STANDARD.decode(
        "RpUpNWfZMjZ1zOOjv3MF2tjO714jsBt0GKnNsw0ihJ4HSZwci+d9zvUi3i67LwFUJgjQ5Dz4zZgHgGduElnmSA==",
    )
    .expect("Failed to decode state proof key")
    .try_into()
    .expect("State proof key should be 64 bytes");

    // Get fresh suggested params to use proper voting rounds
    let params = context
        .algod
        .transaction_params()
        .await
        .expect("Failed to get transaction params");

    // Use voting rounds from suggested params like in Python test
    let vote_first = params.last_round;
    let vote_last = vote_first + 10_000_000; // 10 million rounds like Python test

    let online_key_reg_params = OnlineKeyRegistrationParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        vote_key,
        selection_key,
        vote_first,
        vote_last,
        vote_key_dilution: 100, // Same as Python test
        state_proof_key: Some(state_proof_key),
    };

    let mut composer = context.composer.clone();
    composer
        .add_online_key_registration(online_key_reg_params)
        .expect("Failed to add online key registration");

    // Submit the transaction - should succeed with proper keys and voting rounds
    let result = composer
        .send()
        .await
        .expect("Failed to send online key registration");

    // Assert transaction was confirmed
    assert!(
        result.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        result.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = result.txn.transaction;

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
        _ => panic!("Transaction should be a key registration transaction"),
    }

    // Verify account participation status
    let account_info = context
        .algod
        .account_information(None, &sender_addr.to_string(), None)
        .await
        .expect("Failed to get account information");

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
        panic!("Account should have participation information after online key registration");
    }
}
