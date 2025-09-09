// Simulate transaction tests
// These tests demonstrate the integration test structure and API communication

use algod_client::{
    apis::Format,
    models::{SimulateRequest, SimulateRequestTransactionGroup, SimulateTraceConfig},
};
use algokit_transact::{SignedTransaction, test_utils::TransactionMother};
use algokit_utils::ClientManager;

use crate::common::logging::init_test_logging;

#[tokio::test]
async fn test_simulate_transactions() {
    init_test_logging();

    // Create algod client using ClientManager
    let config = ClientManager::get_config_from_environment_or_localnet();
    let algod_client = ClientManager::get_algod_client(&config.algod_config).unwrap();

    // Create multiple transactions for group simulation using TransactionMother from algokit_transact
    let transaction1 = TransactionMother::simple_payment().build().unwrap();
    let transaction2 = TransactionMother::payment_with_note().build().unwrap();

    let signed_transactions = vec![
        SignedTransaction {
            transaction: transaction1,
            signature: None,
            auth_address: None,
            multisignature: None,
        },
        SignedTransaction {
            transaction: transaction2,
            signature: None,
            auth_address: None,
            multisignature: None,
        },
    ];

    let txn_group = SimulateRequestTransactionGroup {
        txns: signed_transactions.clone(),
    };

    let exec_trace_config = SimulateTraceConfig {
        enable: Some(true),
        stack_change: Some(true),
        scratch_change: Some(true),
        state_change: Some(true),
    };

    let simulate_request = SimulateRequest {
        txn_groups: vec![txn_group],
        allow_empty_signatures: Some(true),
        allow_more_logging: Some(true),
        allow_unnamed_resources: Some(true),
        round: None,
        extra_opcode_budget: Some(1000),
        exec_trace_config: Some(exec_trace_config),
        fix_signers: Some(true),
    };

    // Call the simulate transaction endpoint
    let result = algod_client
        .simulate_transaction(simulate_request, Some(Format::Msgpack))
        .await;

    assert!(
        result.is_ok(),
        "Multi-transaction simulation should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(
        response.txn_groups.len(),
        1,
        "Should have one transaction group"
    );
    assert_eq!(
        response.txn_groups[0].txn_results.len(),
        2,
        "Should have two transaction results"
    );
}
