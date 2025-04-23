use crate::{
    transactions::{SignedTransaction, Transaction},
    Address, AlgorandMsgpack, AssetTransferTransactionFields, PaymentTransactionFields,
    TransactionId, TransactionType,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use pretty_assertions::assert_eq;

#[test]
fn test_payment_transaction_encoding() {
    let tx_struct = Transaction {
        genesis_id: None,
        transaction_type: TransactionType::Payment,
        sender: Address::from_pubkey(&[1; 32]),
        fee: 0,
        first_valid: 1000,
        last_valid: 1000,
        genesis_hash: None,
        note: None,
        rekey_to: None,
        lease: None,
        group: None,
        payment: Some(PaymentTransactionFields {
            receiver: Address::from_pubkey(&[1; 32]),
            amount: 1000,
            close_remainder_to: None,
        }),
        asset_transfer: None,
    };

    let encoded_struct = tx_struct.encode().unwrap();
    let decoded_struct = Transaction::decode(&encoded_struct).unwrap();
    assert_eq!(decoded_struct, tx_struct);

    let signed_tx = SignedTransaction {
        transaction: tx_struct.clone(),
        signature: [0; 64],
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, tx_struct);

    let raw_encoding = tx_struct.encode_raw().unwrap();
    assert_eq!(encoded_struct[0], b'T');
    assert_eq!(encoded_struct[1], b'X');
    assert_eq!(encoded_struct.len(), raw_encoding.len() + 2);
    assert_eq!(encoded_struct[2..], raw_encoding);
    assert_eq!(encoded_struct.len(), 107);
}

#[test]
fn test_asset_transfer_transaction() {
    let tx_struct = Transaction {
        genesis_id: None,
        transaction_type: TransactionType::AssetTransfer,
        sender: Address::from_pubkey(&[1; 32]),
        fee: 0,
        first_valid: 1000,
        last_valid: 1000,
        genesis_hash: None,
        note: None,
        rekey_to: None,
        lease: None,
        group: None,
        payment: None,
        asset_transfer: Some(AssetTransferTransactionFields {
            asset_id: 1,
            amount: 1000,
            receiver: Address::from_pubkey(&[1; 32]),
            asset_sender: None,
            close_remainder_to: None,
        }),
    };

    let encoded_struct = tx_struct.encode().unwrap();
    let decoded_struct = Transaction::decode(&encoded_struct).unwrap();
    assert_eq!(decoded_struct, tx_struct);

    let signed_tx = SignedTransaction {
        transaction: tx_struct.clone(),
        signature: [0; 64],
    };
    let encoded_stx = signed_tx.encode().unwrap();
    let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
    assert_eq!(decoded_stx, signed_tx);
    assert_eq!(decoded_stx.transaction, tx_struct);

    let raw_encoding = tx_struct.encode_raw().unwrap();
    assert_eq!(encoded_struct[0], b'T');
    assert_eq!(encoded_struct[1], b'X');
    assert_eq!(encoded_struct.len(), raw_encoding.len() + 2);
    assert_eq!(encoded_struct[2..], raw_encoding);
    assert_eq!(encoded_struct.len(), 117);
}

#[test]
fn test_address() {
    let addr = Address::from_pubkey(&[0; 32]);
    assert_eq!(
        addr.address(),
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ"
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

    let unsigned_tx = example_pay_transaction();
    let signed_tx = SignedTransaction {
        transaction: unsigned_tx.clone(),
        signature: [0; 64],
    };

    assert_eq!(unsigned_tx.raw_id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.raw_id().unwrap(), expected_tx_id);
}

#[test]
fn test_pay_transaction_id() {
    let expected_tx_id = "ENOQBKTA3UAUU54TQN2AOH7BFDLS6LDYQD2SSQLU76JUAWSQSPPQ";

    let payment = example_pay_transaction();

    let signed_tx = SignedTransaction {
        transaction: payment.clone(),
        signature: [0; 64],
    };

    assert_eq!(payment.id().unwrap(), expected_tx_id);
    assert_eq!(signed_tx.id().unwrap(), expected_tx_id);
}

fn example_pay_transaction() -> Transaction {
    Transaction {
        genesis_id: Some(String::from("testnet-v1.0")),
        transaction_type: TransactionType::Payment,
        sender: Address::from_string("RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q")
            .unwrap(),
        fee: 1000,
        first_valid: 50659540,
        last_valid: 50660540,
        genesis_hash: Some(
            BASE64_STANDARD
                .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        note: Some(
            BASE64_STANDARD
                .decode("MGFhNTBkMjctYjhmNy00ZDc3LWExZmItNTUxZmQ1NWRmMmJj")
                .unwrap(),
        ),
        rekey_to: None,
        lease: None,
        group: None,
        payment: Some(PaymentTransactionFields {
            amount: 101000,
            receiver: Address::from_string(
                "VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ",
            )
            .unwrap(),
            close_remainder_to: None,
        }),
        asset_transfer: None,
    }
}
