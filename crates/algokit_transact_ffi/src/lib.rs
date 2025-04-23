use algokit_transact::{AlgorandMsgpack, Byte32, TransactionId};
use ffi_macros::{ffi_func, ffi_record};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

// thiserror is used to easily create errors than can be propagated to the language bindings
// UniFFI will create classes for errors (i.e. `MsgPackError.EncodingError` in Python)
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitTransactError {
    #[error("EncodingError: {0}")]
    EncodingError(String),
    #[error("DecodingError: {0}")]
    DecodingError(String),
}

// For now, in WASM we just throw the string, hence the error
// type being included in the error string above
// Perhaps in the future we could use a class like in UniFFI
#[cfg(feature = "ffi_wasm")]
impl From<AlgoKitTransactError> for JsValue {
    fn from(e: AlgoKitTransactError) -> Self {
        JsValue::from(e.to_string())
    }
}

// Convert errors from the Rust crate into the FFI-specific errors
impl From<algokit_transact::AlgoKitTransactError> for AlgoKitTransactError {
    fn from(e: algokit_transact::AlgoKitTransactError) -> Self {
        match e {
            algokit_transact::AlgoKitTransactError::DecodingError(_) => {
                AlgoKitTransactError::DecodingError(e.to_string())
            }
            algokit_transact::AlgoKitTransactError::EncodingError(_) => {
                AlgoKitTransactError::EncodingError(e.to_string())
            }
            algokit_transact::AlgoKitTransactError::MsgpackDecodingError(_) => {
                AlgoKitTransactError::DecodingError(e.to_string())
            }
            algokit_transact::AlgoKitTransactError::MsgpackEncodingError(_) => {
                AlgoKitTransactError::EncodingError(e.to_string())
            }
            algokit_transact::AlgoKitTransactError::UnknownTransactionType(_) => {
                AlgoKitTransactError::DecodingError(e.to_string())
            }
            algokit_transact::AlgoKitTransactError::InputError(e) => {
                AlgoKitTransactError::DecodingError(e.to_string())
            }
            algokit_transact::AlgoKitTransactError::InvalidAddress(_) => {
                AlgoKitTransactError::DecodingError(e.to_string())
            }
        }
    }
}

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "ffi_wasm")]
use tsify_next::Tsify;
#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

// We need to use ByteBuf directly in the structs to get Uint8Array in TSify
// custom_type! and this impl is used to convert the ByteBuf to a Vec<u8> for the UniFFI bindings
#[cfg(feature = "ffi_uniffi")]
impl UniffiCustomTypeConverter for ByteBuf {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(ByteBuf::from(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec()
    }
}

#[cfg(feature = "ffi_uniffi")]
uniffi::custom_type!(ByteBuf, Vec<u8>);

// This becomes an enum in UniFFI language bindings and a
// string literal union in TS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum TransactionType {
    Payment,
    AssetTransfer,
    AssetFreeze,
    AssetConfig,
    KeyRegistration,
    ApplicationCall,
}

#[ffi_record]
pub struct Address {
    address: String,
    pub_key: ByteBuf,
}

impl From<algokit_transact::Address> for Address {
    fn from(value: algokit_transact::Address) -> Self {
        return Self {
            address: value.address(),
            pub_key: value.pub_key.to_vec().into(),
        };
    }
}

impl TryFrom<Address> for algokit_transact::Address {
    type Error = AlgoKitTransactError;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        let pub_key: Byte32 = value.pub_key.to_vec().try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError("public key should be 32 bytes".to_string())
        })?;

        Ok(algokit_transact::Address::from_pubkey(&pub_key))
    }
}

// Even though these structs are all defined in the crate, we need to re-define them
// because we have to use different serde attributes for the struct fields.
// In the crate, we need to use the msgpack names for the fields, but in the FFI
// we need to use the camelCase names for the fields for TSify.

#[ffi_record]
pub struct PaymentTransactionFields {
    receiver: Address,

    amount: u64,

    close_remainder_to: Option<Address>,
}

#[ffi_record]
pub struct AssetTransferTransactionFields {
    asset_id: u64,

    amount: u64,

    receiver: Address,

    asset_sender: Option<Address>,

    close_remainder_to: Option<Address>,
}

#[ffi_record]
pub struct Transaction {
    /// The type of transaction
    transaction_type: TransactionType,

    /// The sender of the transaction
    sender: Address,

    fee: u64,

    first_valid: u64,

    last_valid: u64,

    genesis_hash: Option<ByteBuf>,

    genesis_id: Option<String>,

    note: Option<ByteBuf>,

    rekey_to: Option<Address>,

    lease: Option<ByteBuf>,

    group: Option<ByteBuf>,

    payment: Option<PaymentTransactionFields>,

    asset_transfer: Option<AssetTransferTransactionFields>,
}

impl TryFrom<Transaction> for algokit_transact::Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        // Ensure there is never more than 1 transaction type specific field set
        if [tx.payment.is_some(), tx.asset_transfer.is_some()]
            .iter()
            .filter(|&&x| x)
            .count()
            > 1
        {
            return Err(Self::Error::DecodingError(
                "Multiple transaction type specific fields set".to_string(),
            ));
        }

        // Ensure that we have the correct fields set for the transaction type
        match tx.transaction_type {
            TransactionType::Payment => {
                if tx.payment.is_none() {
                    return Err(Self::Error::DecodingError(
                        "Payment data missing".to_string(),
                    ));
                }
            }
            TransactionType::AssetTransfer => {
                if tx.asset_transfer.is_none() {
                    return Err(Self::Error::DecodingError(
                        "Asset Transfer data missing".to_string(),
                    ));
                }
            }
            _ => {
                return Err(Self::Error::DecodingError(
                    "Transaction type is not implemented".to_string(),
                ));
            }
        }

        Ok(Self {
            transaction_type: tx.transaction_type.into(),
            sender: tx.sender.try_into()?,
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_hash: tx.genesis_hash.map(bytebuf_to_byte32).transpose()?,
            genesis_id: tx.genesis_id,
            note: tx.note.map(ByteBuf::into_vec),
            rekey_to: tx.rekey_to.map(TryInto::try_into).transpose()?,
            lease: tx.lease.map(bytebuf_to_byte32).transpose()?,
            group: tx.group.map(bytebuf_to_byte32).transpose()?,
            payment: tx.payment.map(TryInto::try_into).transpose()?,
            asset_transfer: tx.asset_transfer.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<TransactionType> for algokit_transact::TransactionType {
    fn from(tx: TransactionType) -> Self {
        match tx {
            TransactionType::Payment => algokit_transact::TransactionType::Payment,
            TransactionType::AssetTransfer => algokit_transact::TransactionType::AssetTransfer,
            TransactionType::AssetFreeze => algokit_transact::TransactionType::AssetFreeze,
            TransactionType::AssetConfig => algokit_transact::TransactionType::AssetConfig,
            TransactionType::KeyRegistration => algokit_transact::TransactionType::KeyRegistration,
            TransactionType::ApplicationCall => algokit_transact::TransactionType::ApplicationCall,
        }
    }
}

impl From<algokit_transact::PaymentTransactionFields> for PaymentTransactionFields {
    fn from(tx: algokit_transact::PaymentTransactionFields) -> Self {
        Self {
            receiver: tx.receiver.into(),
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to.map(Into::into),
        }
    }
}

impl TryFrom<PaymentTransactionFields> for algokit_transact::PaymentTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: PaymentTransactionFields) -> Result<Self, Self::Error> {
        Ok(Self {
            receiver: tx.receiver.try_into()?,
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<algokit_transact::AssetTransferTransactionFields> for AssetTransferTransactionFields {
    fn from(tx: algokit_transact::AssetTransferTransactionFields) -> Self {
        Self {
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver.into(),
            asset_sender: tx.asset_sender.map(Into::into),
            close_remainder_to: tx.close_remainder_to.map(Into::into),
        }
    }
}

impl TryFrom<AssetTransferTransactionFields> for algokit_transact::AssetTransferTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: AssetTransferTransactionFields) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver.try_into()?,
            asset_sender: tx.asset_sender.map(TryInto::try_into).transpose()?,
            close_remainder_to: tx.close_remainder_to.map(TryInto::try_into).transpose()?,
        })
    }
}

impl TryFrom<algokit_transact::Transaction> for Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(tx: algokit_transact::Transaction) -> Result<Self, Self::Error> {
        // Ensure there is never more than 1 transaction type specific field set
        if [tx.payment.is_some(), tx.asset_transfer.is_some()]
            .iter()
            .filter(|&&x| x)
            .count()
            > 1
        {
            return Err(Self::Error::DecodingError(
                "Multiple transaction type specific fields set".to_string(),
            ));
        }

        // Ensure that we have the correct fields set for the transaction type
        match tx.transaction_type {
            algokit_transact::TransactionType::Payment => {
                if tx.payment.is_none() {
                    return Err(Self::Error::DecodingError(
                        "Payment data missing".to_string(),
                    ));
                }
            }
            algokit_transact::TransactionType::AssetTransfer => {
                if tx.asset_transfer.is_none() {
                    return Err(Self::Error::DecodingError(
                        "Asset Transfer data missing".to_string(),
                    ));
                }
            }
            _ => {
                return Err(Self::Error::DecodingError(
                    "Transaction type is not implemented".to_string(),
                ));
            }
        }

        Ok(Self {
            transaction_type: tx.transaction_type.into(),
            sender: tx.sender.into(),
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_hash: tx.genesis_hash.map(byte32_to_bytebuf),
            genesis_id: tx.genesis_id,
            note: tx.note.map(Into::into),
            rekey_to: tx.rekey_to.map(Into::into),
            lease: tx.lease.map(byte32_to_bytebuf),
            group: tx.group.map(byte32_to_bytebuf),
            payment: tx.payment.map(Into::into),
            asset_transfer: tx.asset_transfer.map(Into::into),
        })
    }
}

impl From<algokit_transact::TransactionType> for TransactionType {
    fn from(tx: algokit_transact::TransactionType) -> Self {
        match tx {
            algokit_transact::TransactionType::Payment => TransactionType::Payment,
            algokit_transact::TransactionType::AssetTransfer => TransactionType::AssetTransfer,
            algokit_transact::TransactionType::AssetFreeze => TransactionType::AssetFreeze,
            algokit_transact::TransactionType::AssetConfig => TransactionType::AssetConfig,
            algokit_transact::TransactionType::KeyRegistration => TransactionType::KeyRegistration,
            algokit_transact::TransactionType::ApplicationCall => TransactionType::ApplicationCall,
        }
    }
}

fn bytebuf_to_byte32(buf: ByteBuf) -> Result<Byte32, AlgoKitTransactError> {
    let vec = buf.to_vec();
    vec.try_into().map_err(|_| {
        AlgoKitTransactError::DecodingError(
            "Expected 32 bytes but got a different length".to_string(),
        )
    })
}

fn byte32_to_bytebuf(b32: Byte32) -> ByteBuf {
    ByteBuf::from(b32.to_vec())
}

// Each function need to be explicitly renamed for WASM
// and exported for UniFFI

/// Get the transaction type from the encoded transaction.
/// This is particularly useful when decoding a transaction that has an unknown type
#[ffi_func]
pub fn get_encoded_transaction_type(bytes: &[u8]) -> Result<TransactionType, AlgoKitTransactError> {
    let decoded = algokit_transact::Transaction::decode(bytes)?;
    Ok(decoded.transaction_type.into())
}

#[ffi_func] 
/// Encode the transaction with the domain separation (e.g. "TX") prefix
pub fn encode_transaction(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = tx.try_into()?;
    Ok(ctx.encode()?)
}

#[ffi_func] 
/// Encode the transaction without the domain separation (e.g. "TX") prefix
/// This is useful for encoding the transaction for signing with tools that automatically add "TX" prefix to the transaction bytes. 
pub fn encode_transaction_raw(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = tx.try_into()?;
    Ok(ctx.encode_raw()?)
}

#[ffi_func]
pub fn decode_transaction(bytes: &[u8]) -> Result<Transaction, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = algokit_transact::Transaction::decode(bytes)?;
    Ok(ctx.try_into()?)
}

#[ffi_func]
pub fn attach_signature(
    encoded_tx: &[u8],
    signature: &[u8],
) -> Result<Vec<u8>, AlgoKitTransactError> {
    let encoded_tx = algokit_transact::Transaction::decode(encoded_tx)?;
    let signed_tx = algokit_transact::SignedTransaction {
        transaction: encoded_tx,
        signature: signature.try_into().expect("signature should be 64 bytes"),
    };
    Ok(signed_tx.encode()?)
}

#[ffi_func]
pub fn address_from_pub_key(pub_key: &[u8]) -> Result<Address, AlgoKitTransactError> {
    Ok(
        algokit_transact::Address::from_pubkey(pub_key.try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError("public key should be 32 bytes".to_string())
        })?)
        .into(),
    )
}

#[ffi_func]
pub fn address_from_string(address: &str) -> Result<Address, AlgoKitTransactError> {
    algokit_transact::Address::from_string(address)
        .map(Into::into)
        .map_err(|e| AlgoKitTransactError::EncodingError(e.to_string()))
}

/// Get the raw 32-byte transaction ID for a transaction.
#[ffi_func]
pub fn get_transaction_raw_id(tx: &Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let tx_internal: algokit_transact::Transaction = tx.clone().try_into()?;
    let raw_id = tx_internal.raw_id()?;
    Ok(raw_id.to_vec())
}

/// Get the base32 transaction ID string for a transaction.
#[ffi_func]
pub fn get_transaction_id(tx: &Transaction) -> Result<String, AlgoKitTransactError> {
    let tx_internal: algokit_transact::Transaction = tx.clone().try_into()?;
    Ok(tx_internal.id()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{prelude::BASE64_STANDARD, Engine};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_encoded_transaction_type() {
        let addr = algokit_transact::Address::from_pubkey(&[0; 32]);

        // Create a minimal payment transaction
        let payment_fields = PaymentTransactionFields {
            receiver: address_from_pub_key(&addr.pub_key).unwrap(),
            amount: 1000000,
            close_remainder_to: None,
        };

        let tx = Transaction {
            transaction_type: TransactionType::Payment,
            sender: address_from_string(&addr.address()).unwrap(),
            fee: 1000,
            first_valid: 1000,
            last_valid: 2000,
            genesis_hash: None,
            genesis_id: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
            payment: Some(payment_fields),
            asset_transfer: None,
        };

        // Encode the transaction
        let encoded = encode_transaction(tx).unwrap();

        // Test the get_encoded_transaction_type function
        let tx_type = get_encoded_transaction_type(&encoded).unwrap();
        assert_eq!(tx_type, TransactionType::Payment);
    }

    // Example transaction modeled from algokit_transact::tests
    fn example_pay_transaction_ffi() -> Transaction {
        let sender_addr =
            address_from_string("RIMARGKZU46OZ77OLPDHHPUJ7YBSHRTCYMQUC64KZCCMESQAFQMYU6SL2Q")
                .unwrap();
        let receiver_addr =
            address_from_string("VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ")
                .unwrap();
        let genesis_hash_bytes = BASE64_STANDARD
            .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
            .unwrap();
        let note_bytes = BASE64_STANDARD
            .decode("MGFhNTBkMjctYjhmNy00ZDc3LWExZmItNTUxZmQ1NWRmMmJj")
            .unwrap();

        let payment_fields = PaymentTransactionFields {
            receiver: receiver_addr,
            amount: 101000,
            close_remainder_to: None,
        };

        Transaction {
            transaction_type: TransactionType::Payment,
            sender: sender_addr,
            fee: 1000,
            first_valid: 50659540,
            last_valid: 50660540,
            genesis_hash: Some(ByteBuf::from(genesis_hash_bytes)),
            genesis_id: Some(String::from("testnet-v1.0")),
            note: Some(ByteBuf::from(note_bytes)),
            rekey_to: None,
            lease: None,
            group: None,
            payment: Some(payment_fields),
            asset_transfer: None,
        }
    }

    #[test]
    fn test_transaction_id_ffi() {
        let tx_ffi = example_pay_transaction_ffi();

        // Expected values from algokit_transact::tests
        //TODO: Update from outcome of AK-244
        let expected_id = "ENOQBKTA3UAUU54TQN2AOH7BFDLS6LDYQD2SSQLU76JUAWSQSPPQ";
        let expected_raw_id = [
            35, 93, 0, 170, 96, 221, 1, 74, 119, 147, 131, 116, 7, 31, 225, 40, 215, 47, 44, 120,
            128, 245, 41, 65, 116, 255, 147, 64, 90, 80, 147, 223,
        ];

        let actual_id = get_transaction_id(&tx_ffi).unwrap();
        let actual_raw_id = get_transaction_raw_id(&tx_ffi).unwrap();

        assert_eq!(actual_id, expected_id);
        assert_eq!(actual_raw_id, expected_raw_id);
    }
}
