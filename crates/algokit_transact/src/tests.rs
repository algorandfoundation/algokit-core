use crate::{
    constants::{
        ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, MAX_TX_GROUP_SIZE,
    },
    test_utils::{
        AddressMother, TransactionGroupMother, TransactionHeaderMother, TransactionMother,
    },
    transactions::{NetworkFeeParams, TransactionFeeParams},
    Address, AlgorandMsgpack, EstimateTransactionSize, SignedTransaction, SignedTransactions,
    Transaction, TransactionId, Transactions,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use pretty_assertions::assert_eq;

#[test]
fn test_payment_transaction_encoding() {
    let tx_builder = TransactionMother::simple_payment();
    let payment_tx_fields = tx_builder.build_fields().unwrap();
    let payment_tx = tx_builder.build().unwrap();

    let encoded = payment_tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, payment_tx);
    assert_eq!(decoded, Transaction::Payment(payment_tx_fields));

    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, payment_tx);

    let raw_encoded = payment_tx.encode_raw().unwrap();
    assert_eq!(encoded[0], b'T');
    assert_eq!(encoded[1], b'X');
    assert_eq!(encoded.len(), raw_encoded.len() + 2);
    assert_eq!(encoded[2..], raw_encoded);
    assert_eq!(encoded.len(), 174);
}

#[test]
fn test_asset_transfer_transaction_encoding() {
    let tx_builder = TransactionMother::opt_in_asset_transfer();
    let asset_transfer_tx_fields = tx_builder.build_fields().unwrap();
    let asset_transfer_tx = tx_builder.build().unwrap();

    let encoded = asset_transfer_tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, asset_transfer_tx);
    assert_eq!(
        decoded,
        Transaction::AssetTransfer(asset_transfer_tx_fields)
    );

    let signed_tx = SignedTransaction {
        transaction: asset_transfer_tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, asset_transfer_tx);

    let raw_encoded = asset_transfer_tx.encode_raw().unwrap();
    assert_eq!(encoded[0], b'T');
    assert_eq!(encoded[1], b'X');
    assert_eq!(encoded.len(), raw_encoded.len() + 2);
    assert_eq!(encoded[2..], raw_encoded);
    assert_eq!(encoded.len(), 178);
}

#[test]
fn test_signed_transaction_encoding() {
    let tx_builder = TransactionMother::simple_payment();
    let payment_tx = tx_builder.build().unwrap();

    // The sender is signing the transaction
    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), 247);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);

    // The sender is not signing the transaction (rekeyed sender account)
    let auth_address = AddressMother::address();
    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: Some([1; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: Some(auth_address.clone()),
    };
    let encoded_stx: Vec<u8> = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), 286);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
}

#[test]
fn test_zero_address() {
    let addr = AddressMother::zero_address();
    assert_eq!(
        addr.to_string(),
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ"
    );

    let addr_from_str = addr.to_string().parse::<Address>().unwrap();
    assert_eq!(addr, addr_from_str);
}

#[test]
fn test_address() {
    let addr = AddressMother::address();
    assert_eq!(
        addr.to_string(),
        "RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q"
    );

    let addr_from_str = addr.to_string().parse::<Address>().unwrap();
    assert_eq!(addr, addr_from_str);
}

#[test]
fn test_pay_transaction_id() {
    let expected_tx_id_raw = [
        35, 93, 0, 170, 96, 221, 1, 74, 119, 147, 131, 116, 7, 31, 225, 40, 215, 47, 44, 120, 128,
        245, 41, 65, 116, 255, 147, 64, 90, 80, 147, 223,
    ];
    let expected_tx_id = "ENOQBKTA3UAUU54TQN2AOH7BFDLS6LDYQD2SSQLU76JUAWSQSPPQ";

    let tx_builder = TransactionMother::payment_with_note();
    let payment_tx = tx_builder.build().unwrap();
    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };

    assert_eq!(payment_tx.id().unwrap(), expected_tx_id);
    assert_eq!(payment_tx.id_raw().unwrap(), expected_tx_id_raw);
    assert_eq!(signed_tx.id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.id_raw().unwrap(), expected_tx_id_raw);
}

#[test]
fn test_estimate_transaction_size() {
    let tx_builder = TransactionMother::simple_payment();
    let payment_tx = tx_builder.build().unwrap();
    let encoding_length = payment_tx.encode_raw().unwrap().len();
    let estimation = payment_tx.estimate_size().unwrap();

    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };
    let actual_size = signed_tx.encode().unwrap().len();

    assert_eq!(
        estimation,
        encoding_length + ALGORAND_SIGNATURE_ENCODING_INCR
    );
    assert_eq!(estimation, actual_size);
}

#[test]
fn test_min_fee() {
    let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

    let network_params = NetworkFeeParams {
        fee_per_byte: 0,
        min_fee: 1000,
    };
    let transaction_params = TransactionFeeParams {
        extra_fee: None,
        max_fee: None,
    };

    let updated_transaction = txn.assign_fee(network_params, transaction_params).unwrap();
    assert_eq!(updated_transaction.header().fee, Some(1000));
}

#[test]
fn test_extra_fee() {
    let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

    let network_params = NetworkFeeParams {
        fee_per_byte: 1,
        min_fee: 1000,
    };
    let transaction_params = TransactionFeeParams {
        extra_fee: Some(500),
        max_fee: None,
    };

    let updated_transaction = txn.assign_fee(network_params, transaction_params).unwrap();
    assert_eq!(updated_transaction.header().fee, Some(1500));
}

#[test]
fn test_max_fee() {
    let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

    let network_params = NetworkFeeParams {
        fee_per_byte: 10,
        min_fee: 500,
    };
    let transaction_params = TransactionFeeParams {
        extra_fee: None,
        max_fee: Some(1000),
    };

    let result = txn.assign_fee(network_params, transaction_params);

    assert!(result.is_err());
    let err: crate::AlgoKitTransactError = result.unwrap_err();
    let msg = format!("{}", err);
    assert!(
        msg == "Transaction fee 2470 µALGO is greater than max fee 1000 µALGO",
        "Unexpected error message: {}",
        msg
    );
}

#[test]
fn test_calculate_fee() {
    let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

    let network_params = NetworkFeeParams {
        fee_per_byte: 5,
        min_fee: 1000,
    };
    let transaction_params = TransactionFeeParams {
        extra_fee: None,
        max_fee: None,
    };

    let updated_transaction = txn.assign_fee(network_params, transaction_params).unwrap();

    assert_eq!(updated_transaction.header().fee, Some(1235));
}

#[test]
fn test_multi_transaction_group() {
    let expected_group: [u8; 32] = BASE64_STANDARD
        .decode(String::from("uJA6BWzZ5g7Ve0FersqCLWsrEstt6p0+F3bNGEKH3I4="))
        .unwrap()
        .try_into()
        .unwrap();
    let txs = TransactionGroupMother::testnet_payment_group();

    let grouped_txs = txs.assign_group().unwrap();

    assert_eq!(grouped_txs.len(), txs.len());
    for grouped_tx in grouped_txs.iter() {
        assert_eq!(grouped_tx.header().group.unwrap(), expected_group);
    }
    assert_eq!(
        &grouped_txs[0].id().unwrap(),
        "6SIXGV2TELA2M5RHZ72CVKLBSJ2OPUAKYFTUUE27O23RN6TFMGHQ"
    );
    assert_eq!(
        &grouped_txs[1].id().unwrap(),
        "7OY3VQXJCDSKPMGEFJMNJL2L3XIOMRM2U7DM2L54CC7QM5YBFQEA"
    );
}

#[test]
fn test_single_transaction_group() {
    let expected_group: [u8; 32] = BASE64_STANDARD
        .decode(String::from("LLW3AwgyXbwoMMBNfLSAGHtqoKtj/c7MjNMR0MGW6sg="))
        .unwrap()
        .try_into()
        .unwrap();
    let txs: Vec<Transaction> = TransactionGroupMother::group_of(1);

    let grouped_txs = txs.assign_group().unwrap();

    assert_eq!(grouped_txs.len(), txs.len());
    for grouped_tx in grouped_txs.iter() {
        assert_eq!(grouped_tx.header().group.unwrap(), expected_group);
    }
}

#[test]
fn test_transaction_group_too_big() {
    let txs: Vec<Transaction> = TransactionGroupMother::group_of(MAX_TX_GROUP_SIZE + 1);

    let result = txs.assign_group();

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .starts_with("Transaction group size exceeds the max limit"));
}

#[test]
fn test_transaction_group_too_small() {
    let txs: Vec<Transaction> = TransactionGroupMother::group_of(0);

    let result = txs.assign_group();

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .starts_with("Transaction group size cannot be 0"));
}

#[test]
fn test_transaction_group_already_set() {
    let tx: Transaction = TransactionMother::simple_payment()
        .header(
            TransactionHeaderMother::simple_testnet()
                .group(
                    BASE64_STANDARD
                        .decode(String::from("y1Hz6KZhHJI4TZLwZqXO3TFgXVQdD/1+c6BLk3wTW6Q="))
                        .unwrap()
                        .try_into()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .to_owned()
        .build()
        .unwrap();

    let result = vec![tx].assign_group();

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .starts_with("Transactions must not already be grouped"));
}

#[test]
fn test_transaction_group_encoding() {
    let grouped_txs = TransactionGroupMother::testnet_payment_group()
        .assign_group()
        .unwrap();

    let encoded_grouped_txs = grouped_txs.encode().unwrap();
    let decoded_grouped_txs = <&[Transaction]>::decode(&encoded_grouped_txs).unwrap();

    for ((grouped_tx, encoded_tx), decoded_tx) in grouped_txs
        .iter()
        .zip(encoded_grouped_txs.into_iter())
        .zip(decoded_grouped_txs.iter())
    {
        assert_eq!(encoded_tx, grouped_tx.encode().unwrap());
        assert_eq!(decoded_tx, grouped_tx);
    }
}

#[test]
fn test_signed_transaction_group_encoding() {
    let signed_grouped_txs = TransactionGroupMother::testnet_payment_group()
        .assign_group()
        .unwrap()
        .iter()
        .map(|tx| SignedTransaction {
            transaction: tx.clone(),
            signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
            auth_address: None,
        })
        .collect::<Vec<SignedTransaction>>();

    let encoded_signed_group = signed_grouped_txs.encode().unwrap();
    let decoded_signed_group = <&[SignedTransaction]>::decode(&encoded_signed_group).unwrap();

    for ((signed_grouped_tx, encoded_signed_tx), decoded_signed_tx) in signed_grouped_txs
        .iter()
        .zip(encoded_signed_group.into_iter())
        .zip(decoded_signed_group.iter())
    {
        assert_eq!(encoded_signed_tx, signed_grouped_tx.encode().unwrap());
        assert_eq!(decoded_signed_tx, signed_grouped_tx);
    }
}

#[test]
fn test_assign_fees_success() {
    let txs: Vec<Transaction> = TransactionGroupMother::testnet_payment_group();
    let network_params = NetworkFeeParams {
        fee_per_byte: 1,
        min_fee: 1000,
    };
    let transaction_params = vec![
        TransactionFeeParams {
            extra_fee: None,
            max_fee: None,
        },
        TransactionFeeParams {
            extra_fee: Some(500),
            max_fee: None,
        },
    ];

    let txs_with_fees = txs.assign_fees(network_params, transaction_params).unwrap();

    assert_eq!(txs_with_fees.len(), txs.len());

    // First transaction: fee_per_byte=1, min_fee=1000, no extra_fee
    // Expected fee should be max(calculated_fee, min_fee) = max(247, 1000) = 1000
    assert_eq!(txs_with_fees[0].header().fee, Some(1000));

    // Second transaction: fee_per_byte=1, min_fee=1000, extra_fee=500
    // Expected fee should be max(calculated_fee, min_fee) + extra_fee = max(247, 1000) + 500 = 1500
    assert_eq!(txs_with_fees[1].header().fee, Some(1500));
}

#[test]
fn test_assign_fees_empty_group() {
    let txs: Vec<Transaction> = vec![];
    let network_params = NetworkFeeParams {
        fee_per_byte: 1,
        min_fee: 1000,
    };
    let transaction_params: Vec<TransactionFeeParams> = vec![];

    let result = txs.assign_fees(network_params, transaction_params);

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .starts_with("Transaction group size cannot be 0"));
}

#[test]
fn test_assign_fees_mismatched_size() {
    let txs: Vec<Transaction> = TransactionGroupMother::testnet_payment_group();
    let network_params = NetworkFeeParams {
        fee_per_byte: 1,
        min_fee: 1000,
    };
    let transaction_params = vec![TransactionFeeParams {
        extra_fee: None,
        max_fee: None,
    }]; // Only one transaction param for two transactions

    let result = txs.assign_fees(network_params, transaction_params);

    let error = result.unwrap_err();
    assert_eq!(
        error.to_string(),
        "Number of transaction fee parameters (1) must match number of transactions (2)"
    );
}

#[test]
fn test_assign_fees_with_max_fee_violation() {
    let txs: Vec<Transaction> = vec![TransactionMother::simple_payment().build().unwrap()];
    let network_params = NetworkFeeParams {
        fee_per_byte: 10,
        min_fee: 500,
    };
    let transaction_params = vec![TransactionFeeParams {
        extra_fee: None,
        max_fee: Some(1000),
    }];

    let result = txs.assign_fees(network_params, transaction_params);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = format!("{}", err);
    assert!(
        msg.contains("Transaction fee") && msg.contains("is greater than max fee"),
        "Unexpected error message: {}",
        msg
    );
}

#[test]
fn test_assign_fees_single_transaction() {
    let txs: Vec<Transaction> = vec![TransactionMother::simple_payment().build().unwrap()];
    let network_params = NetworkFeeParams {
        fee_per_byte: 5,
        min_fee: 1000,
    };
    let transaction_params = vec![TransactionFeeParams {
        extra_fee: Some(200),
        max_fee: Some(5000),
    }];

    let txs_with_fees = txs.assign_fees(network_params, transaction_params).unwrap();

    assert_eq!(txs_with_fees.len(), 1);
    // Expected fee: max(5 * estimated_size, 1000) + 200
    // Since estimated_size is around 247, calculated_fee = 5 * 247 = 1235
    // Final fee = max(1235, 1000) + 200 = 1235 + 200 = 1435
    assert_eq!(txs_with_fees[0].header().fee, Some(1435));
}

#[test]
fn test_assign_fees_different_transaction_types() {
    let payment_tx = TransactionMother::simple_payment().build().unwrap();
    let asset_transfer_tx = TransactionMother::opt_in_asset_transfer().build().unwrap();
    let txs: Vec<Transaction> = vec![payment_tx, asset_transfer_tx];

    let network_params = NetworkFeeParams {
        fee_per_byte: 1,
        min_fee: 1000,
    };
    let transaction_params = vec![
        TransactionFeeParams {
            extra_fee: None,
            max_fee: None,
        },
        TransactionFeeParams {
            extra_fee: Some(500),
            max_fee: None,
        },
    ];

    let txs_with_fees = txs.assign_fees(network_params, transaction_params).unwrap();

    assert_eq!(txs_with_fees.len(), 2);
    // Both transactions should have fees assigned according to their respective parameters
    assert_eq!(txs_with_fees[0].header().fee, Some(1000));
    assert_eq!(txs_with_fees[1].header().fee, Some(1500));
}
