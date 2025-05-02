use crate::{
    test_utils::{AddressMother, TransactionMother},
    Address, AlgorandMsgpack, AssetTransferTransactionFields, PaymentTransactionFields,
    SignedTransaction, Transaction, TransactionId,
};
use pretty_assertions::assert_eq;

#[test]
fn test_payment_transaction_encoding() {
    let tx_builder = TransactionMother::simple_payment();
    let payment_tx_fields = tx_builder.build_fields().unwrap();
    let payment_tx = tx_builder.build().unwrap();

    let encoded_struct = payment_tx_fields.encode().unwrap();
    let decoded_struct = PaymentTransactionFields::decode(&encoded_struct).unwrap();
    assert_eq!(decoded_struct, payment_tx_fields);

    let encoded_enum = payment_tx.encode().unwrap();
    let decoded_enum = Transaction::decode(&encoded_enum).unwrap();
    assert_eq!(decoded_enum, payment_tx);
    assert_eq!(
        decoded_enum,
        Transaction::Payment(payment_tx_fields.clone())
    );

    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: [0; 64],
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, payment_tx);

    let raw_encoding = payment_tx_fields.encode_raw().unwrap();
    assert_eq!(encoded_struct[0], b'T');
    assert_eq!(encoded_struct[1], b'X');
    assert_eq!(encoded_struct.len(), raw_encoding.len() + 2);
    assert_eq!(encoded_struct[2..], raw_encoding);
    assert_eq!(encoded_struct.len(), 174);
}

#[test]
fn test_asset_transfer_transaction_encoding() {
    let tx_builder = TransactionMother::opt_in_asset_transfer();
    let asset_transfer_tx_fields = tx_builder.build_fields().unwrap();
    let asset_transfer_tx = tx_builder.build().unwrap();

    let encoded_struct = asset_transfer_tx_fields.encode().unwrap();
    let decoded_struct = AssetTransferTransactionFields::decode(&encoded_struct).unwrap();
    assert_eq!(decoded_struct, asset_transfer_tx_fields);

    let encoded_enum = asset_transfer_tx.encode().unwrap();
    let decoded_enum = Transaction::decode(&encoded_enum).unwrap();
    assert_eq!(decoded_enum, asset_transfer_tx);

    let signed_tx = SignedTransaction {
        transaction: asset_transfer_tx.clone(),
        signature: [0; 64],
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, asset_transfer_tx);

    let raw_encoding = asset_transfer_tx_fields.encode_raw().unwrap();
    assert_eq!(encoded_struct[0], b'T');
    assert_eq!(encoded_struct[1], b'X');
    assert_eq!(encoded_struct.len(), raw_encoding.len() + 2);
    assert_eq!(encoded_struct[2..], raw_encoding);
    assert_eq!(encoded_struct.len(), 178);
}

#[test]
fn test_zero_address() {
    let addr = AddressMother::zero_address();
    assert_eq!(
        addr.address(),
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ"
    );

    let addr_from_str = Address::from_string(&addr.address()).unwrap();
    assert_eq!(addr, addr_from_str);
}

#[test]
fn test_address() {
    let addr = AddressMother::address();
    assert_eq!(
        addr.address(),
        "RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q"
    );

    let addr_from_str = Address::from_string(&addr.address()).unwrap();
    assert_eq!(addr, addr_from_str);
}

#[test]
fn test_pay_transaction_raw_id() {
    let expected_tx_id = [
        35, 93, 0, 170, 96, 221, 1, 74, 119, 147, 131, 116, 7, 31, 225, 40, 215, 47, 44, 120, 128,
        245, 41, 65, 116, 255, 147, 64, 90, 80, 147, 223,
    ];

    let tx_builder = TransactionMother::payment_with_note();
    let payment_tx_fields = tx_builder.build_fields().unwrap();
    let payment_tx = tx_builder.build().unwrap();
    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: [0; 64],
    };

    assert_eq!(payment_tx_fields.raw_id().unwrap(), expected_tx_id);
    assert_eq!(payment_tx.raw_id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.raw_id().unwrap(), expected_tx_id);
}

#[test]
fn test_pay_transaction_id() {
    let expected_tx_id = "ENOQBKTA3UAUU54TQN2AOH7BFDLS6LDYQD2SSQLU76JUAWSQSPPQ";

    let tx_builder = TransactionMother::payment_with_note();
    let payment_tx_fields = tx_builder.build_fields().unwrap();
    let payment_tx = tx_builder.build().unwrap();
    let signed_tx = SignedTransaction {
        transaction: payment_tx.clone(),
        signature: [0; 64],
    };

    assert_eq!(payment_tx_fields.id().unwrap(), expected_tx_id);
    assert_eq!(payment_tx.id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.id().unwrap(), expected_tx_id);
}
