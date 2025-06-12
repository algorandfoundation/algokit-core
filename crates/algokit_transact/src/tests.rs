use crate::{
    constants::{
        ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, MAX_TX_GROUP_SIZE,
    },
    test_utils::{
        AddressMother, TransactionGroupMother, TransactionHeaderMother, TransactionMother,
    },
    transactions::FeeParams,
    Address, AlgorandMsgpack, EstimateTransactionSize, SignedTransaction, Transaction,
    TransactionId, Transactions,
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
