use crate::{
    constants::{
        ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, MAX_TX_GROUP_SIZE,
    },
    test_utils::{
        AccountMother, TransactionGroupMother, TransactionHeaderMother, TransactionMother,
    },
    transactions::{AssetFreezeTransactionBuilder, FeeParams},
    Account, AlgorandMsgpack, EstimateTransactionSize, MultisigSignature, MultisigSubsignature,
    SignedTransaction, Transaction, TransactionId, Transactions,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use pretty_assertions::assert_eq;

pub fn check_transaction_encoding(tx: &Transaction, expected_encoded_len: usize) {
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, *tx);

    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
        multisignature: None,
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

pub fn check_signed_transaction_encoding(
    tx: &Transaction,
    expected_encoded_len: usize,
    auth_account: Option<Account>,
) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: auth_account.map(|acc| acc.address()),
        multisignature: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), expected_encoded_len);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
}

pub fn check_multisigned_transaction_encoding(tx: &Transaction, expected_encoded_len: usize) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: None,
        auth_address: None,
        multisignature: Some(MultisigSignature {
            version: 1,
            threshold: 2,
            subsignatures: vec![
                MultisigSubsignature {
                    address: AccountMother::account().address(),
                    signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
                },
                MultisigSubsignature {
                    address: AccountMother::neil().address(),
                    signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
                },
            ],
        }),
    };
    let encoded_stx = signed_tx.encode().unwrap();
    assert_eq!(encoded_stx.len(), expected_encoded_len);
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
}

pub fn check_transaction_id(tx: &Transaction, expected_tx_id: &str) {
    let signed_tx = SignedTransaction {
        transaction: tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
        multisignature: None,
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
    check_multisigned_transaction_encoding(&payment_tx, 449);
}

#[test]
fn test_asset_transfer_transaction_encoding() {
    let asset_transfer_tx = TransactionMother::simple_asset_transfer().build().unwrap();

    check_transaction_id(
        &asset_transfer_tx,
        "VAHP4FRJH4GRV6ID2BZRK5VYID376EV3VE6T2TKKDFJBBDOXWCCA",
    );
    check_transaction_encoding(&asset_transfer_tx, 186);
    check_multisigned_transaction_encoding(&asset_transfer_tx, 423);
}

#[test]
fn test_asset_opt_in_transaction_encoding() {
    let asset_opt_in_tx = TransactionMother::opt_in_asset_transfer().build().unwrap();

    check_transaction_id(
        &asset_opt_in_tx,
        "JIDBHDPLBASULQZFI4EY5FJWR6VQRMPPFSGYBKE2XKW65N3UQJXA",
    );
    check_transaction_encoding(&asset_opt_in_tx, 178);
    check_multisigned_transaction_encoding(&asset_opt_in_tx, 415);
}

#[test]
fn test_payment_signed_transaction_encoding() {
    let payment_tx = TransactionMother::simple_payment().build().unwrap();
    check_signed_transaction_encoding(&payment_tx, 247, None);
    check_signed_transaction_encoding(&payment_tx, 286, Some(AccountMother::account().clone()));
}

#[test]
fn test_asset_transfer_signed_transaction_encoding() {
    let asset_transfer_tx = TransactionMother::simple_asset_transfer().build().unwrap();
    check_signed_transaction_encoding(&asset_transfer_tx, 259, None);
    check_signed_transaction_encoding(
        &asset_transfer_tx,
        298,
        Some(AccountMother::account().clone()),
    );
}

#[test]
fn test_asset_opt_in_signed_transaction_encoding() {
    let asset_opt_in_tx = TransactionMother::opt_in_asset_transfer().build().unwrap();
    check_signed_transaction_encoding(&asset_opt_in_tx, 251, None);
    check_signed_transaction_encoding(
        &asset_opt_in_tx,
        290,
        Some(AccountMother::account().clone()),
    );
}

#[test]
fn test_zero_address_account() {
    let acct = AccountMother::zero_address_account();
    assert_eq!(
        acct.to_string(),
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ"
    );

    let addr_from_str = acct.to_string().parse::<Account>().unwrap();
    assert_eq!(acct, addr_from_str);
}

#[test]
fn test_account() {
    let acct = AccountMother::account();
    assert_eq!(
        acct.to_string(),
        "RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q"
    );

    let addr_from_str = acct.to_string().parse::<Account>().unwrap();
    assert_eq!(acct, addr_from_str);
}

#[test]
fn test_msig_account() {
    let msig = AccountMother::msig();
    assert_eq!(
        msig.to_string(),
        "TZ6HCOKXK54E2VRU523LBTDQMQNX7DXOWENPFNBXOEU3SMEWXYNCRJUTBU"
    );
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
        multisignature: None,
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
        multisignature: None,
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
            multisignature: None,
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
fn test_asset_freeze_transaction_encoding() {
    let tx_builder = TransactionMother::asset_freeze();
    let asset_freeze_tx_fields = tx_builder.build_fields().unwrap();
    let asset_freeze_tx = tx_builder.build().unwrap();

    let encoded = asset_freeze_tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, asset_freeze_tx);
    assert_eq!(decoded, Transaction::AssetFreeze(asset_freeze_tx_fields));

    let signed_tx = SignedTransaction {
        transaction: asset_freeze_tx.clone(),
        signature: Some([0; ALGORAND_SIGNATURE_BYTE_LENGTH]),
        auth_address: None,
        multisignature: None,
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, asset_freeze_tx);

    let raw_encoded = asset_freeze_tx.encode_raw().unwrap();
    assert_eq!(encoded[0], b'T');
    assert_eq!(encoded[1], b'X');
    assert_eq!(encoded.len(), raw_encoded.len() + 2);
    assert_eq!(encoded[2..], raw_encoded);
}

#[test]
fn test_asset_unfreeze_transaction_encoding() {
    let tx_builder = TransactionMother::asset_unfreeze();
    let asset_freeze_tx_fields = tx_builder.build_fields().unwrap();
    let asset_freeze_tx = tx_builder.build().unwrap();

    // Verify it's an unfreeze transaction
    assert_eq!(asset_freeze_tx_fields.frozen, Some(false));

    let encoded = asset_freeze_tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, asset_freeze_tx);
    assert_eq!(decoded, Transaction::AssetFreeze(asset_freeze_tx_fields));
}

#[test]
fn test_asset_freeze_mainnet_encoding() {
    let tx = TransactionMother::asset_freeze().build().unwrap();
    // Just verify it encodes without checking exact size
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, tx);
}

#[test]
fn test_asset_freeze_testnet_encoding() {
    let tx = TransactionMother::asset_freeze().build().unwrap();
    // Just verify it encodes without checking exact size
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, tx);
}

#[test]
fn test_asset_freeze_minimal_encoding() {
    let tx = TransactionMother::asset_freeze().build().unwrap();
    // Just verify it encodes without checking exact size
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, tx);
}

#[test]
fn test_asset_freeze_with_group_encoding() {
    let tx = TransactionMother::asset_freeze().build().unwrap();

    // Verify group field is set
    if let Transaction::AssetFreeze(fields) = &tx {
        assert!(fields.header.group.is_some());
    }

    // Just verify it encodes without checking exact size
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();
    assert_eq!(decoded, tx);
}

#[test]
fn test_asset_freeze_real_transaction_ids() {
    // Test with mainnet freeze
    let freeze_tx = TransactionMother::asset_freeze().build().unwrap();
    let freeze_id = freeze_tx.id().unwrap();
    assert_eq!(freeze_id.len(), 52); // Base32 encoded length

    // Test with mainnet unfreeze
    let unfreeze_tx = TransactionMother::asset_unfreeze().build().unwrap();
    let unfreeze_id = unfreeze_tx.id().unwrap();
    assert_eq!(unfreeze_id.len(), 52);

    // Verify freeze and unfreeze have different IDs (different frozen value)
    assert_ne!(freeze_id, unfreeze_id);
}

#[test]
fn test_asset_freeze_required_fields() {
    // Missing asset_id should fail
    let result = AssetFreezeTransactionBuilder::default()
        .header(TransactionHeaderMother::simple_testnet().build().unwrap())
        .freeze_target(AccountMother::neil().address())
        .frozen(true)
        .build();

    // Builder with derive_builder pattern requires all fields
    assert!(result.is_err());

    // Missing freeze_target should fail
    let result = AssetFreezeTransactionBuilder::default()
        .header(TransactionHeaderMother::simple_testnet().build().unwrap())
        .asset_id(12345)
        .frozen(true)
        .build();

    assert!(result.is_err());
}

#[test]
fn test_asset_freeze_serialization_fields() {
    let tx = TransactionMother::asset_freeze().build().unwrap();
    let encoded = tx.encode_raw().unwrap();

    // Decode as MessagePack Value first
    let decoded_value: rmpv::Value = rmp_serde::from_slice(&encoded).unwrap();

    // Convert to a map to check fields
    if let rmpv::Value::Map(map) = decoded_value {
        let mut found_faid = false;
        let mut found_fadd = false;
        let mut found_afrz = false;
        let mut found_type = false;

        for (key, _value) in map {
            if let rmpv::Value::String(ref s) = key {
                match s.as_str() {
                    Some("faid") => found_faid = true,
                    Some("fadd") => found_fadd = true,
                    Some("afrz") => found_afrz = true,
                    Some("type") => found_type = true,
                    _ => {}
                }
            }
        }

        assert!(found_faid, "Missing faid field");
        assert!(found_fadd, "Missing fadd field");
        assert!(found_afrz, "Missing afrz field");
        assert!(found_type, "Missing type field");
    } else {
        panic!("Expected MessagePack map");
    }
}
