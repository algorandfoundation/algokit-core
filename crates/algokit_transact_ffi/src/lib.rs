use algokit_transact::constants::*;
use algokit_transact::{
    AlgorandMsgpack, Byte32, EstimateTransactionSize, TransactionId, Transactions,
};
use ffi_macros::{ffi_enum, ffi_func, ffi_record};
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
    #[error("{0}")]
    InputError(String),
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
                AlgoKitTransactError::InputError(e.to_string())
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
use js_sys::Uint8Array;
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
            address: value.to_string(),
            pub_key: value.pub_key.to_vec().into(),
        };
    }
}

impl TryFrom<Address> for algokit_transact::Address {
    type Error = AlgoKitTransactError;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] =
            value.pub_key.to_vec().try_into().map_err(|_| {
                AlgoKitTransactError::EncodingError(
                    format!(
                        "public key should be {} bytes",
                        ALGORAND_PUBLIC_KEY_BYTE_LENGTH
                    )
                    .to_string(),
                )
            })?;

        Ok(algokit_transact::Address::from_pubkey(&pub_key))
    }
}

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

    fn try_from(tx: Transaction) -> Result<Self, AlgoKitTransactError> {
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

        match tx.transaction_type {
            TransactionType::Payment => Ok(algokit_transact::Transaction::Payment(tx.try_into()?)),
            TransactionType::AssetTransfer => {
                Ok(algokit_transact::Transaction::AssetTransfer(tx.try_into()?))
            }
            _ => {
                return Err(Self::Error::DecodingError(
                    "Transaction type is not implemented".to_string(),
                ));
            }
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::TransactionHeader {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, AlgoKitTransactError> {
        Ok(Self {
            sender: tx.sender.try_into()?,
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx.genesis_hash.map(bytebuf_to_byte32).transpose()?,
            note: tx.note.map(ByteBuf::into_vec),
            rekey_to: tx.rekey_to.map(TryInto::try_into).transpose()?,
            lease: tx.lease.map(bytebuf_to_byte32).transpose()?,
            group: tx.group.map(bytebuf_to_byte32).transpose()?,
        })
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

impl TryFrom<Transaction> for algokit_transact::PaymentTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::Payment || tx.payment.is_none() {
            return Err(Self::Error::DecodingError(
                "Payment data missing".to_string(),
            ));
        }

        let data = tx.clone().payment.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        Ok(Self {
            header,
            amount: data.amount,
            receiver: data.receiver.try_into()?,
            close_remainder_to: data.close_remainder_to.map(TryInto::try_into).transpose()?,
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

impl TryFrom<Transaction> for algokit_transact::AssetTransferTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::AssetTransfer || tx.asset_transfer.is_none() {
            return Err(Self::Error::DecodingError(
                "Asset Transfer data missing".to_string(),
            ));
        }

        let data = tx.clone().asset_transfer.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        Ok(Self {
            header,
            asset_id: data.asset_id,
            amount: data.amount,
            receiver: data.receiver.try_into()?,
            asset_sender: data.asset_sender.map(TryInto::try_into).transpose()?,
            close_remainder_to: data.close_remainder_to.map(TryInto::try_into).transpose()?,
        })
    }
}

impl TryFrom<algokit_transact::Transaction> for Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(tx: algokit_transact::Transaction) -> Result<Self, AlgoKitTransactError> {
        match tx {
            algokit_transact::Transaction::Payment(payment) => {
                let payment_fields = payment.clone().into();
                build_transaction(
                    payment.header,
                    TransactionType::Payment,
                    Some(payment_fields),
                    None,
                )
            }
            algokit_transact::Transaction::AssetTransfer(asset_transfer) => {
                let asset_transfer_fields = asset_transfer.clone().into();
                build_transaction(
                    asset_transfer.header,
                    TransactionType::AssetTransfer,
                    None,
                    Some(asset_transfer_fields),
                )
            }
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

fn build_transaction(
    header: algokit_transact::TransactionHeader,
    transaction_type: TransactionType,
    payment: Option<PaymentTransactionFields>,
    asset_transfer: Option<AssetTransferTransactionFields>,
) -> Result<Transaction, AlgoKitTransactError> {
    Ok(Transaction {
        transaction_type,
        sender: header.sender.into(),
        fee: header.fee,
        first_valid: header.first_valid,
        last_valid: header.last_valid,
        genesis_id: header.genesis_id,
        genesis_hash: header.genesis_hash.map(byte32_to_bytebuf),
        note: header.note.map(Into::into),
        rekey_to: header.rekey_to.map(Into::into),
        lease: header.lease.map(byte32_to_bytebuf),
        group: header.group.map(byte32_to_bytebuf),
        payment,
        asset_transfer,
    })
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
/// Encode the transaction with the domain separation (e.g. "TX") prefix
pub fn encode_transaction(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = tx.try_into()?;
    Ok(ctx.encode()?)
}

#[cfg(feature = "ffi_wasm")]
#[ffi_func]
/// Encode transactions with the domain separation (e.g. "TX") prefix
pub fn encode_transactions(txs: Vec<Transaction>) -> Result<Vec<Uint8Array>, AlgoKitTransactError> {
    Ok(encode_transactions_impl(txs)?
        .iter()
        .map(|bytes| bytes.as_slice().into())
        .collect::<Vec<Uint8Array>>())
}

#[cfg(not(feature = "ffi_wasm"))]
#[ffi_func]
/// Encode transactions with the domain separation (e.g. "TX") prefix
pub fn encode_transactions(txs: Vec<Transaction>) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    encode_transactions_impl(txs)
}

fn encode_transactions_impl(txs: Vec<Transaction>) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    txs.iter().cloned().map(encode_transaction).collect()
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
        signature: signature.try_into().expect(&format!(
            "signature should be {} bytes",
            ALGORAND_SIGNATURE_BYTE_LENGTH
        )),
    };
    Ok(signed_tx.encode()?)
}

#[cfg(feature = "ffi_wasm")]
#[ffi_func]
pub fn attach_signatures(
    encoded_txs: Vec<Uint8Array>,
    signatures: Vec<Uint8Array>,
) -> Result<Vec<Uint8Array>, AlgoKitTransactError> {
    Ok(attach_signatures_impl(
        encoded_txs.iter().map(|bytes| bytes.to_vec()).collect(),
        signatures.iter().map(|bytes| bytes.to_vec()).collect(),
    )?
    .iter()
    .map(|s| s.as_slice().into())
    .collect())
}

#[cfg(not(feature = "ffi_wasm"))]
#[ffi_func]
pub fn attach_signatures(
    encoded_txs: Vec<Vec<u8>>,
    signatures: Vec<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    attach_signatures_impl(encoded_txs, signatures)
}

fn attach_signatures_impl(
    encoded_txs: Vec<Vec<u8>>,
    signatures: Vec<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    if encoded_txs.len() != signatures.len() {
        return Err(AlgoKitTransactError::InputError(
            "Number of transactions and signatures must match".to_string(),
        ));
    }

    encoded_txs
        .into_iter()
        .zip(signatures)
        .map(|(encoded_tx, sig)| attach_signature(&encoded_tx, &sig))
        .collect()
}

#[ffi_func]
/// Return the size of the transaction in bytes as if it was already signed and encoded.
/// This is useful for estimating the fee for the transaction.
pub fn estimate_transaction_size(transaction: &Transaction) -> Result<u64, AlgoKitTransactError> {
    let core_tx: algokit_transact::Transaction = transaction.clone().try_into()?;
    return core_tx
        .estimate_size()
        .map_err(|e| {
            AlgoKitTransactError::EncodingError(format!(
                "Failed to estimate transaction size: {}",
                e
            ))
        })?
        .try_into()
        .map_err(|_| {
            AlgoKitTransactError::EncodingError("Failed to convert size to u64".to_string())
        });
}

#[ffi_func]
pub fn address_from_pub_key(pub_key: &[u8]) -> Result<Address, AlgoKitTransactError> {
    Ok(
        algokit_transact::Address::from_pubkey(pub_key.try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError(
                format!(
                    "public key should be {} bytes",
                    ALGORAND_PUBLIC_KEY_BYTE_LENGTH
                )
                .to_string(),
            )
        })?)
        .into(),
    )
}

#[ffi_func]
pub fn address_from_string(address: &str) -> Result<Address, AlgoKitTransactError> {
    address
        .parse::<algokit_transact::Address>()
        .map(Into::into)
        .map_err(|e| AlgoKitTransactError::EncodingError(e.to_string()))
}

/// Get the raw 32-byte transaction ID for a transaction.
#[ffi_func]
pub fn get_transaction_id_raw(tx: &Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let tx_internal: algokit_transact::Transaction = tx.clone().try_into()?;
    let id_raw = tx_internal.id_raw()?;
    Ok(id_raw.to_vec())
}

/// Get the base32 transaction ID string for a transaction.
#[ffi_func]
pub fn get_transaction_id(tx: &Transaction) -> Result<String, AlgoKitTransactError> {
    let tx_internal: algokit_transact::Transaction = tx.clone().try_into()?;
    Ok(tx_internal.id()?)
}

/// Groups a collection of transactions by calculating and assigning the group to each transaction.
#[ffi_func]
pub fn group_transactions(txs: Vec<Transaction>) -> Result<Vec<Transaction>, AlgoKitTransactError> {
    let txs_internal: Vec<algokit_transact::Transaction> = txs
        .iter()
        .map(|tx| tx.clone().try_into())
        .collect::<Result<Vec<_>, _>>()?;

    let grouped_txs: Vec<Transaction> = txs_internal
        .assign_group()?
        .into_iter()
        .map(|tx| tx.try_into())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(grouped_txs)
}

/// Enum containing all constants used in this crate.
#[ffi_enum]
pub enum AlgorandConstant {
    /// Length of hash digests (32)
    HashLength,

    /// Length of the checksum used in Algorand addresses (4)
    ChecksumLength,

    /// Length of a base32-encoded Algorand address (58)
    AddressLength,

    /// Length of an Algorand public key in bytes (32)
    PublicKeyLength,

    /// Length of an Algorand secret key in bytes (32)
    SecretKeyLength,

    /// Length of an Algorand signature in bytes (64)
    SignatureLength,

    /// Increment in the encoded byte size when a signature is attached to a transaction (75)
    SignatureEncodingIncrLength,

    // The maximum number of transactions in a group (16)
    MaxTxGroupSize,
}

impl AlgorandConstant {
    /// Get the numeric value of the constant
    pub fn value(&self) -> u64 {
        match self {
            AlgorandConstant::HashLength => HASH_BYTES_LENGTH as u64,
            AlgorandConstant::ChecksumLength => ALGORAND_CHECKSUM_BYTE_LENGTH as u64,
            AlgorandConstant::AddressLength => ALGORAND_ADDRESS_LENGTH as u64,
            AlgorandConstant::PublicKeyLength => ALGORAND_PUBLIC_KEY_BYTE_LENGTH as u64,
            AlgorandConstant::SecretKeyLength => ALGORAND_SECRET_KEY_BYTE_LENGTH as u64,
            AlgorandConstant::SignatureLength => ALGORAND_SIGNATURE_BYTE_LENGTH as u64,
            AlgorandConstant::SignatureEncodingIncrLength => {
                ALGORAND_SIGNATURE_ENCODING_INCR as u64
            }
            AlgorandConstant::MaxTxGroupSize => MAX_TX_GROUP_SIZE as u64,
        }
    }
}

#[ffi_func]
pub fn get_algorand_constant(constant: AlgorandConstant) -> u64 {
    constant.value()
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{TestDataMother, TransactionMother};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_encoded_transaction_type() {
        let txn: Transaction = TransactionMother::simple_payment()
            .build()
            .unwrap()
            .try_into()
            .unwrap();

        // Encode the transaction
        let encoded = encode_transaction(txn).unwrap();

        // Test the get_encoded_transaction_type function
        let tx_type = get_encoded_transaction_type(&encoded).unwrap();
        assert_eq!(tx_type, TransactionType::Payment);
    }

    #[test]
    fn test_transaction_id_ffi() {
        let data = TestDataMother::simple_payment();
        let tx_ffi = data.transaction.try_into().unwrap();

        let actual_id = get_transaction_id(&tx_ffi).unwrap();
        let actual_id_raw = get_transaction_id_raw(&tx_ffi).unwrap();

        assert_eq!(actual_id, data.id);
        assert_eq!(actual_id_raw, data.id_raw);
    }

    #[test]
    fn test_group_transactions_ffi() {
        let expected_group = [
            202, 79, 82, 7, 197, 237, 213, 55, 117, 226, 131, 74, 221, 85, 86, 215, 64, 133, 212,
            7, 58, 234, 248, 162, 222, 53, 161, 29, 141, 101, 133, 49,
        ];
        let tx1 = TestDataMother::simple_payment()
            .transaction
            .try_into()
            .unwrap();
        let tx2 = TestDataMother::opt_in_asset_transfer()
            .transaction
            .try_into()
            .unwrap();
        let txs = vec![tx1, tx2];

        let grouped_txs = group_transactions(txs.clone()).unwrap();
        let encoded_grouped_txs = encode_transactions(grouped_txs.clone()).unwrap();
        let signed_encoded_grouped_txs = attach_signatures(
            encoded_grouped_txs,
            vec![
                [0; ALGORAND_SIGNATURE_BYTE_LENGTH].to_vec(),
                [1; ALGORAND_SIGNATURE_BYTE_LENGTH].to_vec(),
            ],
        )
        .unwrap();

        // TODO: NC - Finalise testing strategy

        assert_eq!(grouped_txs.len(), txs.len());
        for (original_tx, grouped_tx) in txs.iter().zip(grouped_txs.iter()) {
            assert_eq!(original_tx.group, None); // TODO: NC - This check is kind of redundant. Try fix this.
            assert_eq!(grouped_tx.group.as_ref().unwrap(), &expected_group);
        }
        assert_eq!(signed_encoded_grouped_txs.len(), txs.len());
    }
}
