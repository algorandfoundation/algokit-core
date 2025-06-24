use crate::{
    constants::{
        ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, MAX_TX_GROUP_SIZE,
    },
    test_utils::{
        AddressMother, TransactionGroupMother, TransactionHeaderMother, TransactionMother,
    },
    transactions::FeeParams,
    Address, AlgorandMsgpack, ApplicationCallTransactionBuilder, BoxReference,
    EstimateTransactionSize, OnApplicationComplete, SignedTransaction, Transaction, TransactionId,
    Transactions,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use pretty_assertions::assert_eq;

fn check_transaction_encoding(tx: &Transaction, expected_encoded_len: usize) {
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, *tx);

    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, *tx);

    let raw_encoded = tx.encode_raw().unwrap();
    assert_eq!(encoded[0], b'T');
    assert_eq!(encoded[1], b'X');
    assert_eq!(encoded.len(), raw_encoded.len() + 2);
    assert_eq!(encoded[2..], raw_encoded);
    assert_eq!(encoded.len(), expected_encoded_len);
}

fn check_signed_transaction_encoding(
    tx: &Transaction,
    expected_encoded_len: usize,
    auth_address: Option<Address>,
) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: auth_address.clone(),
    };
    let encoded_stx = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), expected_encoded_len);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
}

fn check_transaction_id(tx: &Transaction, expected_tx_id: &str) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
    };

    assert_eq!(tx.id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.id().unwrap(), expected_tx_id);
}

#[test]
fn test_payment_transaction_encoding() {
    let payment_tx = TransactionMother::observed_payment().build().unwrap();

    check_transaction_id(
        &payment_tx,
        "VTADY3NGJGE4DVZ4CKLX43NTEE3C2J4JJANZ5TPBR4OYJ2D4F2CA",
    );
    check_transaction_encoding(&payment_tx, 212);
}

#[test]
fn test_asset_transfer_transaction_encoding() {
    let asset_transfer_tx = TransactionMother::simple_asset_transfer().build().unwrap();

    check_transaction_id(
        &asset_transfer_tx,
        "VAHP4FRJH4GRV6ID2BZRK5VYID376EV3VE6T2TKKDFJBBDOXWCCA",
    );
    check_transaction_encoding(&asset_transfer_tx, 186);
}

#[test]
fn test_asset_opt_in_transaction_encoding() {
    let asset_opt_in_tx = TransactionMother::opt_in_asset_transfer().build().unwrap();

    check_transaction_id(
        &asset_opt_in_tx,
        "JIDBHDPLBASULQZFI4EY5FJWR6VQRMPPFSGYBKE2XKW65N3UQJXA",
    );
    check_transaction_encoding(&asset_opt_in_tx, 178);
}

#[test]
fn test_payment_signed_transaction_encoding() {
    let payment_tx = TransactionMother::simple_payment().build().unwrap();
    check_signed_transaction_encoding(&payment_tx, 247, None);
    check_signed_transaction_encoding(&payment_tx, 286, Some(AddressMother::address().clone()));
}

#[test]
fn test_asset_transfer_signed_transaction_encoding() {
    let asset_transfer_tx = TransactionMother::simple_asset_transfer().build().unwrap();
    check_signed_transaction_encoding(&asset_transfer_tx, 259, None);
    check_signed_transaction_encoding(
        &asset_transfer_tx,
        298,
        Some(AddressMother::address().clone()),
    );
}

#[test]
fn test_asset_opt_in_signed_transaction_encoding() {
    let asset_opt_in_tx = TransactionMother::opt_in_asset_transfer().build().unwrap();
    check_signed_transaction_encoding(&asset_opt_in_tx, 251, None);
    check_signed_transaction_encoding(
        &asset_opt_in_tx,
        290,
        Some(AddressMother::address().clone()),
    );
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
fn test_pay_estimate_transaction_size() {
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
fn test_axfer_estimate_transaction_size() {
    let tx_builder = TransactionMother::simple_asset_transfer();
    let asset_transfer_tx = tx_builder.build().unwrap();
    let encoding_length = asset_transfer_tx.encode_raw().unwrap().len();
    let estimation = asset_transfer_tx.estimate_size().unwrap();

    let signed_tx = SignedTransaction {
        transaction: asset_transfer_tx.clone(),
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

    let updated_transaction = txn
        .assign_fee(FeeParams {
            fee_per_byte: 0,
            min_fee: 1000,
            extra_fee: None,
            max_fee: None,
        })
        .unwrap();
    assert_eq!(updated_transaction.header().fee, Some(1000));
}

#[test]
fn test_extra_fee() {
    let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

    let updated_transaction = txn
        .assign_fee(FeeParams {
            fee_per_byte: 1,
            min_fee: 1000,
            extra_fee: Some(500),
            max_fee: None,
        })
        .unwrap();
    assert_eq!(updated_transaction.header().fee, Some(1500));
}

#[test]
fn test_max_fee() {
    let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

    let result = txn.assign_fee(FeeParams {
        fee_per_byte: 10,
        min_fee: 500,
        extra_fee: None,
        max_fee: Some(1000),
    });

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

    let updated_transaction = txn
        .assign_fee(FeeParams {
            fee_per_byte: 5,
            min_fee: 1000,
            extra_fee: None,
            max_fee: None,
        })
        .unwrap();

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

    let encoded_grouped_txs = grouped_txs
        .iter()
        .map(|tx| tx.encode())
        .collect::<Result<Vec<Vec<u8>>, _>>()
        .unwrap();
    let decoded_grouped_txs = encoded_grouped_txs
        .iter()
        .map(|tx| Transaction::decode(tx))
        .collect::<Result<Vec<Transaction>, _>>()
        .unwrap();

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

    let encoded_signed_group = signed_grouped_txs
        .iter()
        .map(|tx| tx.encode())
        .collect::<Result<Vec<Vec<u8>>, _>>()
        .unwrap();
    let decoded_signed_group = encoded_signed_group
        .iter()
        .map(|tx| SignedTransaction::decode(tx))
        .collect::<Result<Vec<SignedTransaction>, _>>()
        .unwrap();

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
fn test_application_create_transaction_encoding() {
    let application_create_tx = TransactionMother::application_create().build().unwrap();

    check_transaction_id(
        &application_create_tx,
        "L6B56N2BAXE43PUI7IDBXCJN5DEB6NLCH4AAN3ON64CXPSCTJNTA",
    );
    check_transaction_encoding(&application_create_tx, 1386);
}

#[test]
fn test_application_call_encoding() {
    let application_call_tx = TransactionMother::application_call().build().unwrap();

    check_transaction_id(
        &application_call_tx,
        "6Y644M5SGTKNBH7ZX6D7QAAHDF6YL6FDJPRAGSUHNZLR4IKGVSPQ",
    );
    check_transaction_encoding(&application_call_tx, 377);
}

#[test]
fn test_application_update_encoding() {
    let application_update_tx = TransactionMother::application_update().build().unwrap();

    check_transaction_id(
        &application_update_tx,
        "NQVNJ5VWEDX42DMJQIQET4QPNUOW27EYIPKZ4SDWKOOEFJQB7PZA",
    );
    check_transaction_encoding(&application_update_tx, 7069);
}

#[test]
fn test_application_delete_transaction_encoding() {
    let application_delete_tx = TransactionMother::application_delete().build().unwrap();

    check_transaction_id(
        &application_delete_tx,
        "XVVC7UDLCPI622KCJZLWK3SEAWWVUEPEXUM5CO3DFLWOBH7NOPDQ",
    );
    check_transaction_encoding(&application_delete_tx, 263);
}

#[test]
fn test_application_opt_in_transaction_encoding() {
    let application_opt_in_tx = TransactionMother::application_opt_in().build().unwrap();

    check_transaction_id(
        &application_opt_in_tx,
        "BNASGY47TXXUTFUZPDAGGPQKK54B4QPEEPDTJIZFDXC64WQH4GOQ",
    );
    check_transaction_encoding(&application_opt_in_tx, 247);
}

#[test]
fn test_application_close_out_transaction_encoding() {
    let application_close_out_tx = TransactionMother::application_close_out().build().unwrap();

    check_transaction_id(
        &application_close_out_tx,
        "R4LXOUN4KPRIILRLIYKMA2DJ4HKCXWCD5TYWGH76635KUHGFNTUQ",
    );
    check_transaction_encoding(&application_close_out_tx, 131);
}

#[test]
fn test_application_clear_state_transaction_encoding() {
    let application_clear_state_tx = TransactionMother::application_clear_state()
        .build()
        .unwrap();

    check_transaction_id(
        &application_clear_state_tx,
        "XQE2YKONC62QXSXDIRJ7CL6YDWP45JXCQO6N7DAAFQH7DJM6BEKA",
    );
    check_transaction_encoding(&application_clear_state_tx, 131);
}

#[test]
fn test_0_box_ref_application_call_transaction_encoding() {
    let application_call_tx = TransactionMother::application_call_example()
        .box_references(vec![BoxReference {
            app_id: 0,
            name: "b1".as_bytes().to_vec(),
        }])
        .build()
        .unwrap();

    check_transaction_id(
        &application_call_tx,
        "LXUGSM4264PQ2YSSO3JW535NHGC5JESKLQS6ITONGO2S6ATEWM2A",
    );
    check_transaction_encoding(&application_call_tx, 138);
}

#[test]
fn test_app_id_box_ref_application_call_transaction_encoding() {
    let application_call_tx = TransactionMother::application_call_example()
        .box_references(vec![BoxReference {
            app_id: 12345,
            name: "b1".as_bytes().to_vec(),
        }])
        .build()
        .unwrap();

    check_transaction_id(
        &application_call_tx,
        "LXUGSM4264PQ2YSSO3JW535NHGC5JESKLQS6ITONGO2S6ATEWM2A",
    );

    let encoded = application_call_tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();

    if let Transaction::ApplicationCall(decoded_app_call) = decoded {
        assert_eq!(
            decoded_app_call.box_references.as_ref().unwrap()[0].app_id,
            0
        );
    } else {
        panic!("Expected ApplicationCall transaction type");
    }
}

#[test]
fn test_external_box_refs_application_call_transaction_encoding() {
    let application_call_tx = TransactionMother::application_call_example()
        .app_references(vec![54321, 11111, 55555, 22222])
        .box_references(vec![
            BoxReference {
                app_id: 55555,
                name: "b1".as_bytes().to_vec(),
            },
            BoxReference {
                app_id: 54321,
                name: "b2".as_bytes().to_vec(),
            },
        ])
        .build()
        .unwrap();

    check_transaction_id(
        &application_call_tx,
        "GB4AYDJEHVBLOVSLCBOXG3KASTS3V6QV6GPB6F2BILG7L6J3P4OQ",
    );
    check_transaction_encoding(&application_call_tx, 169);
}

#[test]
fn test_box_ref_missing_app_reference_encode() {
    let application_call_tx: Transaction = TransactionMother::application_call_example()
        .app_references(vec![54321])
        .box_references(vec![
            BoxReference {
                app_id: 55555,
                name: "b1".as_bytes().to_vec(),
            },
            BoxReference {
                app_id: 54321,
                name: "b2".as_bytes().to_vec(),
            },
        ])
        .build()
        .unwrap();

    let result = application_call_tx.encode();

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("Box reference with app id 55555 not found in app references"),
        "Expected missing app reference error, got: {}",
        error_message
    );
}

#[test]
fn test_box_ref_missing_app_reference_decode() {
    let encoded_tx_missing_app_ref = [
        84, 88, 138, 164, 97, 112, 98, 120, 146, 130, 161, 105, 1, 161, 110, 196, 2, 98, 49, 130,
        161, 105, 2, 161, 110, 196, 2, 98, 50, 164, 97, 112, 102, 97, 145, 205, 212, 49, 164, 97,
        112, 105, 100, 205, 48, 57, 163, 102, 101, 101, 205, 3, 232, 162, 102, 118, 1, 163, 103,
        101, 110, 167, 101, 120, 97, 109, 112, 108, 101, 162, 103, 104, 196, 32, 222, 189, 190,
        157, 28, 11, 247, 214, 147, 68, 228, 226, 58, 211, 196, 121, 68, 26, 174, 253, 159, 1, 57,
        38, 54, 88, 135, 169, 241, 177, 52, 144, 162, 108, 118, 205, 3, 231, 163, 115, 110, 100,
        196, 32, 2, 204, 225, 113, 58, 8, 179, 189, 204, 74, 148, 128, 202, 244, 192, 188, 2, 202,
        236, 227, 17, 198, 25, 62, 33, 204, 91, 40, 252, 44, 209, 74, 164, 116, 121, 112, 101, 164,
        97, 112, 112, 108,
    ];

    let result = Transaction::decode(&encoded_tx_missing_app_ref);

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("Cannot find app reference index 1"),
        "Expected missing app reference error, got: {}",
        error_message
    );
}

#[test]
fn test_application_call_empty_value_encoding() {
    let builder = &ApplicationCallTransactionBuilder::default()
        .header(TransactionHeaderMother::example().build().unwrap())
        .app_id(1234)
        .on_complete(OnApplicationComplete::NoOp)
        .to_owned();

    let tx = builder.clone().build().unwrap();
    let tx_with_empties = builder
        .clone()
        .approval_program(vec![])
        .clear_state_program(vec![])
        .args(vec![])
        .account_references(vec![])
        .asset_references(vec![])
        .account_references(vec![])
        .box_references(vec![])
        .build()
        .unwrap();

    let expected_id = "AEAVEJUTYW5MFUWTDX6YPQS57FILUMVGDNYVB6ZGNNWL5Z4D43OA";

    assert_ne!(tx, tx_with_empties);

    // Because id's are a hash of the encoded bytes, we can be sure the encoded bytes are the same
    check_transaction_id(&tx, expected_id);
    check_transaction_id(&tx_with_empties, expected_id);
}
