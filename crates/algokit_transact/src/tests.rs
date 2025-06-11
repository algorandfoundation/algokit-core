use crate::constants::{ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR};
use crate::FeeParams;
use crate::{
    test_utils::{AddressMother, TransactionMother},
    Address, AlgorandMsgpack, EstimateTransactionSize, SignedTransaction, Transaction,
    TransactionId,
};
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
