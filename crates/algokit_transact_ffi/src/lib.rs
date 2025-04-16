use algokit_transact::AlgorandMsgpack;
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
        let pub_key: [u8; 32] = value.pub_key.to_vec().try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError("public key should be 32 bytes".to_string())
        })?;

        Ok(algokit_transact::Address::from_pubkey(&pub_key))
    }
}

// Even though these structs are all defined in the crate, we need to re-define them
// because we have to use different serde attributes for the struct fields.
// In the crate, we need to use the msgpack names for the fields, but in the FFI
// we need to use the camelCase names for the fields for TSify.

/// The transaction header contains the fields that can be present in any transaction.
/// "Header" only indicates that these are common fields, NOT that they are the first fields in the transaction.
#[ffi_record]
pub struct TransactionHeader {
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
}

#[ffi_record]
pub struct PayTransactionFields {
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
    header: TransactionHeader,

    pay_fields: Option<PayTransactionFields>,

    asset_transfer_fields: Option<AssetTransferTransactionFields>,
}

impl TryFrom<Transaction> for algokit_transact::Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, AlgoKitTransactError> {
        // Ensure we only have pay fields or asset transfer fields
        let fields: [bool; 2] = [tx.pay_fields.is_some(), tx.asset_transfer_fields.is_some()];

        // If fields has more than one true value, then we have an error
        if fields.iter().filter(|&&x| x).count() > 1 {
            return Err(AlgoKitTransactError::DecodingError(
                "Multiple fields set".to_string(),
            ));
        }

        if let Some(pay) = tx.pay_fields {
            return Ok(algokit_transact::Transaction::Payment(
                algokit_transact::PayTransactionFields {
                    header: tx.header.try_into()?,
                    amount: pay.amount,
                    receiver: pay.receiver.try_into()?,
                    close_remainder_to: pay.close_remainder_to.map(|a| a.try_into()).transpose()?,
                },
            ));
        }

        if let Some(asset_transfer) = tx.asset_transfer_fields {
            return Ok(algokit_transact::Transaction::AssetTransfer(
                algokit_transact::AssetTransferTransactionFields {
                    header: tx.header.try_into()?,
                    asset_id: asset_transfer.asset_id,
                    amount: asset_transfer.amount,
                    receiver: asset_transfer.receiver.try_into()?,
                    asset_sender: asset_transfer
                        .asset_sender
                        .map(|a| a.try_into())
                        .transpose()?,
                    close_remainder_to: asset_transfer
                        .close_remainder_to
                        .map(|a| a.try_into())
                        .transpose()?,
                },
            ));
        }

        Err(AlgoKitTransactError::DecodingError(
            "No transaction fields set".to_string(),
        ))
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

impl TryFrom<TransactionHeader> for algokit_transact::TransactionHeader {
    type Error = AlgoKitTransactError;

    fn try_from(tx: TransactionHeader) -> Result<Self, AlgoKitTransactError> {
        Ok(Self {
            transaction_type: tx.transaction_type.into(),
            sender: tx.sender.try_into()?,
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx
                .genesis_hash
                .map(|b| {
                    b.to_vec().try_into().map_err(|_| {
                        AlgoKitTransactError::EncodingError(
                            "genesis_hash should be 32 byte hash".to_string(),
                        )
                    })
                })
                .transpose()?,
            note: tx.note.map(|b| b.to_vec()),
            rekey_to: tx.rekey_to.map(|a| a.try_into()).transpose()?,
            lease: tx
                .lease
                .map(|b| {
                    b.to_vec().try_into().map_err(|_| {
                        AlgoKitTransactError::EncodingError("lease should be 32 bytes".to_string())
                    })
                })
                .transpose()?,
            group: tx
                .group
                .map(|b| {
                    b.to_vec().try_into().map_err(|_| {
                        AlgoKitTransactError::EncodingError("group should be 32 byte hash".to_string())
                    })
                })
                .transpose()?,
        })
    }
}

impl From<algokit_transact::TransactionHeader> for TransactionHeader {
    fn from(tx: algokit_transact::TransactionHeader) -> Self {
        Self {
            transaction_type: tx.transaction_type.into(),
            sender: tx.sender.into(),
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx.genesis_hash.map(|b| ByteBuf::from(b.to_vec())),
            note: tx.note.map(|b| ByteBuf::from(b.to_vec())),
            rekey_to: tx.rekey_to.map(|a| a.into()),
            lease: tx.lease.map(|b| ByteBuf::from(b.to_vec())),
            group: tx.group.map(|b| ByteBuf::from(b.to_vec())),
        }
    }
}

impl From<algokit_transact::PayTransactionFields> for PayTransactionFields {
    fn from(tx: algokit_transact::PayTransactionFields) -> Self {
        Self {
            receiver: tx.receiver.into(),
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to.map(|a| a.into()),
        }
    }
}

impl TryFrom<PayTransactionFields> for algokit_transact::PayTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: PayTransactionFields) -> Result<Self, Self::Error> {
        Ok(Self {
            header: algokit_transact::TransactionHeader {
                transaction_type: algokit_transact::TransactionType::Payment,
                sender: algokit_transact::Address::from_pubkey(&[0; 32]), // This will be overridden by the Transaction conversion
                fee: 0,
                first_valid: 0,
                last_valid: 0,
                genesis_id: None,
                genesis_hash: None,
                note: None,
                rekey_to: None,
                lease: None,
                group: None,
            },
            amount: tx.amount,
            receiver: tx.receiver.try_into()?,
            close_remainder_to: tx.close_remainder_to.map(|a| a.try_into()).transpose()?,
        })
    }
}

impl From<algokit_transact::AssetTransferTransactionFields> for AssetTransferTransactionFields {
    fn from(tx: algokit_transact::AssetTransferTransactionFields) -> Self {
        Self {
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver.into(),
            asset_sender: tx.asset_sender.map(|a| a.into()),
            close_remainder_to: tx.close_remainder_to.map(|a| a.into()),
        }
    }
}

impl TryFrom<AssetTransferTransactionFields> for algokit_transact::AssetTransferTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: AssetTransferTransactionFields) -> Result<Self, Self::Error> {
        Ok(Self {
            header: algokit_transact::TransactionHeader {
                transaction_type: algokit_transact::TransactionType::AssetTransfer,
                sender: algokit_transact::Address::from_pubkey(&[0; 32]), // This will be overridden by the Transaction conversion
                fee: 0,
                first_valid: 0,
                last_valid: 0,
                genesis_id: None,
                genesis_hash: None,
                note: None,
                rekey_to: None,
                lease: None,
                group: None,
            },
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver.try_into()?,
            asset_sender: tx.asset_sender.map(|a| a.try_into()).transpose()?,
            close_remainder_to: tx.close_remainder_to.map(|a| a.try_into()).transpose()?,
        })
    }
}

impl TryFrom<algokit_transact::Transaction> for Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(tx: algokit_transact::Transaction) -> Result<Self, Self::Error> {
        match tx {
            algokit_transact::Transaction::Payment(payment) => {
                let header = payment.header.into();
                let pay_fields = PayTransactionFields {
                    receiver: payment.receiver.into(),
                    amount: payment.amount,
                    close_remainder_to: payment.close_remainder_to.map(|a| a.into()),
                };

                Ok(Self {
                    header,
                    pay_fields: Some(pay_fields),
                    asset_transfer_fields: None,
                })
            }
            algokit_transact::Transaction::AssetTransfer(asset_transfer) => {
                let header = asset_transfer.header.into();
                let asset_fields = AssetTransferTransactionFields {
                    asset_id: asset_transfer.asset_id,
                    amount: asset_transfer.amount,
                    receiver: asset_transfer.receiver.into(),
                    asset_sender: asset_transfer.asset_sender.map(|a| a.into()),
                    close_remainder_to: asset_transfer.close_remainder_to.map(|a| a.into()),
                };

                Ok(Self {
                    header,
                    pay_fields: None,
                    asset_transfer_fields: Some(asset_fields),
                })
            }
        }
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

// Each function need to be explicitly renamed for WASM
// and exported for UniFFI

/// Get the transaction type from the encoded transaction.
/// This is particularly useful when decoding a transaction that has an unknown type
#[ffi_func]
pub fn get_encoded_transaction_type(bytes: &[u8]) -> Result<TransactionType, AlgoKitTransactError> {
    let decoded = algokit_transact::Transaction::decode(bytes)?;

    match decoded {
        algokit_transact::Transaction::Payment(_) => Ok(TransactionType::Payment),
        algokit_transact::Transaction::AssetTransfer(_) => Ok(TransactionType::AssetTransfer),
    }
}

#[ffi_func]
pub fn encode_transaction(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = tx.try_into()?;
    Ok(ctx.encode()?)
}

#[ffi_func]
pub fn decode_transaction(bytes: &[u8]) -> Result<Transaction, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = algokit_transact::Transaction::decode(bytes)?;
    Ok(ctx.try_into()?)
}

#[ffi_func]
pub fn attach_signature(encoded_tx: &[u8], signature: &[u8]) -> Result<Vec<u8>, AlgoKitTransactError> {
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
        .map(|a| a.into())
        .map_err(|e| AlgoKitTransactError::EncodingError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_encoded_transaction_type() {
        let addr = algokit_transact::Address::from_pubkey(&[0; 32]);

        // Create a minimal payment transaction
        let header = TransactionHeader {
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
        };

        let pay_fields = PayTransactionFields {
            receiver: address_from_pub_key(&addr.pub_key).unwrap(),
            amount: 1000000,
            close_remainder_to: None,
        };

        let tx = Transaction {
            header,
            pay_fields: Some(pay_fields),
            asset_transfer_fields: None,
        };

        // Encode the transaction
        let encoded = encode_transaction(tx).unwrap();

        // Test the get_encoded_transaction_type function
        let tx_type = get_encoded_transaction_type(&encoded).unwrap();
        assert_eq!(tx_type, TransactionType::Payment);
    }
}
