mod transactions;

use algokit_transact::constants::*;
use algokit_transact::{
    AlgorandMsgpack, EstimateTransactionSize, TransactionId, Transactions, Validate,
};
use ffi_macros::{ffi_enum, ffi_func, ffi_record};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

pub use transactions::ApplicationCallTransactionFields;
pub use transactions::AssetConfigTransactionFields;
pub use transactions::AssetFreezeTransactionFields;
pub use transactions::KeyRegistrationTransactionFields;

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
    #[error("MsgPackError: {0}")]
    MsgPackError(String),
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
pub struct Account {
    address: String,
    pub_key: ByteBuf,
}

#[ffi_record]
pub struct MultisigSignature {
    address: String,
    version: u8,
    threshold: u8,
    subsignatures: Vec<MultisigSubsignature>,
}

#[ffi_record]
pub struct MultisigSubsignature {
    address: String,
    signature: Option<ByteBuf>,
}

impl From<algokit_transact::Account> for Account {
    fn from(value: algokit_transact::Account) -> Self {
        Self {
            address: value.to_string(),
            pub_key: value.pub_key.to_vec().into(),
        }
    }
}

impl TryFrom<Account> for algokit_transact::Account {
    type Error = AlgoKitTransactError;

    fn try_from(value: Account) -> Result<Self, Self::Error> {
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

        Ok(algokit_transact::Account::from_pubkey(&pub_key))
    }
}

impl From<algokit_transact::Address> for Account {
    fn from(value: algokit_transact::Address) -> Self {
        Self {
            address: value.to_string(),
            pub_key: value.as_bytes().to_vec().into(),
        }
    }
}

impl TryFrom<Account> for algokit_transact::Address {
    type Error = AlgoKitTransactError;

    fn try_from(value: Account) -> Result<Self, Self::Error> {
        value.address.parse().map_err(Self::Error::from)
    }
}

impl From<algokit_transact::MultisigSignature> for MultisigSignature {
    fn from(value: algokit_transact::MultisigSignature) -> Self {
        Self {
            address: value.to_string(),
            version: value.version,
            threshold: value.threshold,
            subsignatures: value.subsignatures.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<MultisigSignature> for algokit_transact::MultisigSignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: MultisigSignature) -> Result<Self, Self::Error> {
        Ok(Self {
            version: value.version,
            threshold: value.threshold,
            subsignatures: value
                .subsignatures
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<algokit_transact::MultisigSubsignature> for MultisigSubsignature {
    fn from(value: algokit_transact::MultisigSubsignature) -> Self {
        Self {
            address: value.address.as_str(),
            signature: value.signature.map(|sig| sig.to_vec().into()),
        }
    }
}

impl TryFrom<MultisigSubsignature> for algokit_transact::MultisigSubsignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: MultisigSubsignature) -> Result<Self, Self::Error> {
        let address = value.address.parse()?;
        let signature = value
            .signature
            .map(|sig| {
                sig.to_vec().try_into().map_err(|_| {
                    AlgoKitTransactError::EncodingError(format!(
                        "signature should be {} bytes",
                        ALGORAND_SIGNATURE_BYTE_LENGTH
                    ))
                })
            })
            .transpose()?;

        Ok(Self { address, signature })
    }
}

#[ffi_record]
pub struct FeeParams {
    fee_per_byte: u64,
    min_fee: u64,
    extra_fee: Option<u64>,
    max_fee: Option<u64>,
}

#[ffi_record]
pub struct PaymentTransactionFields {
    receiver: String,

    amount: u64,

    close_remainder_to: Option<String>,
}

#[ffi_record]
pub struct AssetTransferTransactionFields {
    asset_id: u64,

    amount: u64,

    receiver: String,

    asset_sender: Option<String>,

    close_remainder_to: Option<String>,
}

#[ffi_record]
pub struct Transaction {
    /// The type of transaction
    transaction_type: TransactionType,

    /// The sender of the transaction
    sender: String,

    /// Optional transaction fee in microALGO.
    ///
    /// If not set, the fee will be interpreted as 0 by the network.
    fee: Option<u64>,

    first_valid: u64,

    last_valid: u64,

    genesis_hash: Option<ByteBuf>,

    genesis_id: Option<String>,

    note: Option<ByteBuf>,

    rekey_to: Option<String>,

    lease: Option<ByteBuf>,

    group: Option<ByteBuf>,

    payment: Option<PaymentTransactionFields>,

    asset_transfer: Option<AssetTransferTransactionFields>,

    asset_config: Option<AssetConfigTransactionFields>,

    application_call: Option<ApplicationCallTransactionFields>,

    key_registration: Option<KeyRegistrationTransactionFields>,

    asset_freeze: Option<AssetFreezeTransactionFields>,
}

impl TryFrom<Transaction> for algokit_transact::Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, AlgoKitTransactError> {
        // Ensure there is never more than 1 transaction type specific field set
        if [
            tx.payment.is_some(),
            tx.asset_transfer.is_some(),
            tx.asset_config.is_some(),
            tx.key_registration.is_some(),
            tx.application_call.is_some(),
            tx.asset_freeze.is_some(),
        ]
        .into_iter()
        .filter(|&x| x)
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
            TransactionType::KeyRegistration => Ok(algokit_transact::Transaction::KeyRegistration(
                tx.try_into()?,
            )),

            TransactionType::AssetConfig => {
                Ok(algokit_transact::Transaction::AssetConfig(tx.try_into()?))
            }

            TransactionType::ApplicationCall => Ok(algokit_transact::Transaction::ApplicationCall(
                tx.try_into()?,
            )),
            TransactionType::AssetFreeze => {
                Ok(algokit_transact::Transaction::AssetFreeze(tx.try_into()?))
            }
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::TransactionHeader {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, AlgoKitTransactError> {
        Ok(Self {
            sender: tx.sender.parse()?,
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx
                .genesis_hash
                .map(|buf| bytebuf_to_bytes::<32>(&buf))
                .transpose()?,
            note: tx.note.map(ByteBuf::into_vec),
            rekey_to: tx.rekey_to.map(|addr| addr.parse()).transpose()?,
            lease: tx
                .lease
                .map(|buf| bytebuf_to_bytes::<32>(&buf))
                .transpose()?,
            group: tx
                .group
                .map(|buf| bytebuf_to_bytes::<32>(&buf))
                .transpose()?,
        })
    }
}

impl From<algokit_transact::PaymentTransactionFields> for PaymentTransactionFields {
    fn from(tx: algokit_transact::PaymentTransactionFields) -> Self {
        Self {
            receiver: tx.receiver.as_str(),
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to.map(|addr| addr.as_str()),
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
            receiver: data.receiver.parse()?,
            close_remainder_to: data
                .close_remainder_to
                .map(|addr| addr.parse())
                .transpose()?,
        })
    }
}

impl From<algokit_transact::AssetTransferTransactionFields> for AssetTransferTransactionFields {
    fn from(tx: algokit_transact::AssetTransferTransactionFields) -> Self {
        Self {
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver.as_str(),
            asset_sender: tx.asset_sender.map(|addr| addr.as_str()),
            close_remainder_to: tx.close_remainder_to.map(|addr| addr.as_str()),
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
            receiver: data.receiver.parse()?,
            asset_sender: data.asset_sender.map(|addr| addr.parse()).transpose()?,
            close_remainder_to: data
                .close_remainder_to
                .map(|addr| addr.parse())
                .transpose()?,
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
                    None,
                    None,
                    None,
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
                    None,
                    None,
                    None,
                    None,
                )
            }
            algokit_transact::Transaction::AssetConfig(asset_config) => {
                let asset_config_fields = asset_config.clone().into();
                build_transaction(
                    asset_config.header.clone(),
                    TransactionType::AssetConfig,
                    None,
                    None,
                    Some(asset_config_fields),
                    None,
                    None,
                    None,
                )
            }
            algokit_transact::Transaction::ApplicationCall(application_call) => {
                let application_call_fields = application_call.clone().into();
                build_transaction(
                    application_call.header,
                    TransactionType::ApplicationCall,
                    None,
                    None,
                    None,
                    Some(application_call_fields),
                    None,
                    None,
                )
            }
            algokit_transact::Transaction::KeyRegistration(key_registration) => {
                let key_registration_fields = key_registration.clone().into();
                build_transaction(
                    key_registration.header,
                    TransactionType::KeyRegistration,
                    None,
                    None,
                    None,
                    None,
                    Some(key_registration_fields),
                    None,
                )
            }
            algokit_transact::Transaction::AssetFreeze(asset_freeze) => {
                let asset_freeze_fields = asset_freeze.clone().into();
                build_transaction(
                    asset_freeze.header,
                    TransactionType::AssetFreeze,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(asset_freeze_fields),
                )
            }
        }
    }
}

#[ffi_record]
pub struct SignedTransaction {
    /// The transaction that has been signed.
    pub transaction: Transaction,

    /// Optional Ed25519 signature authorizing the transaction.
    pub signature: Option<ByteBuf>,

    /// Optional auth address applicable if the transaction sender is a rekeyed account.
    pub auth_address: Option<String>,

    /// Optional multisig signature if the transaction is a multisig transaction.
    pub multisignature: Option<MultisigSignature>,
}

impl From<algokit_transact::SignedTransaction> for SignedTransaction {
    fn from(signed_tx: algokit_transact::SignedTransaction) -> Self {
        Self {
            transaction: signed_tx.transaction.try_into().unwrap(),
            signature: signed_tx.signature.map(|sig| sig.to_vec().into()),
            auth_address: signed_tx.auth_address.map(|addr| addr.as_str()),
            multisignature: signed_tx.multisignature.map(Into::into),
        }
    }
}

impl TryFrom<SignedTransaction> for algokit_transact::SignedTransaction {
    type Error = AlgoKitTransactError;

    fn try_from(signed_tx: SignedTransaction) -> Result<Self, Self::Error> {
        let signature = signed_tx
            .signature
            .map(|sig| {
                sig.to_vec().try_into().map_err(|_| {
                    AlgoKitTransactError::EncodingError(format!(
                        "signature should be {} bytes",
                        ALGORAND_SIGNATURE_BYTE_LENGTH
                    ))
                })
            })
            .transpose()?;

        Ok(Self {
            transaction: signed_tx.transaction.try_into()?,
            signature,
            auth_address: signed_tx
                .auth_address
                .map(|addr| addr.parse())
                .transpose()?,
            multisignature: signed_tx
                .multisignature
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

fn bytebuf_to_bytes<const N: usize>(buf: &ByteBuf) -> Result<[u8; N], AlgoKitTransactError> {
    buf.to_vec().try_into().map_err(|_| {
        AlgoKitTransactError::DecodingError(format!(
            "Expected {} bytes but got a different length",
            N
        ))
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
    asset_config: Option<AssetConfigTransactionFields>,
    application_call: Option<ApplicationCallTransactionFields>,
    key_registration: Option<KeyRegistrationTransactionFields>,
    asset_freeze: Option<AssetFreezeTransactionFields>,
) -> Result<Transaction, AlgoKitTransactError> {
    Ok(Transaction {
        transaction_type,
        sender: header.sender.as_str(),
        fee: header.fee,
        first_valid: header.first_valid,
        last_valid: header.last_valid,
        genesis_id: header.genesis_id,
        genesis_hash: header.genesis_hash.map(byte32_to_bytebuf),
        note: header.note.map(Into::into),
        rekey_to: header.rekey_to.map(|addr| addr.as_str()),
        lease: header.lease.map(byte32_to_bytebuf),
        group: header.group.map(byte32_to_bytebuf),
        payment,
        asset_transfer,
        asset_config,
        application_call,
        key_registration,
        asset_freeze,
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
        algokit_transact::Transaction::AssetConfig(_) => Ok(TransactionType::AssetConfig),
        algokit_transact::Transaction::ApplicationCall(_) => Ok(TransactionType::ApplicationCall),
        algokit_transact::Transaction::KeyRegistration(_) => Ok(TransactionType::KeyRegistration),
        algokit_transact::Transaction::AssetFreeze(_) => Ok(TransactionType::AssetFreeze),
    }
}

#[ffi_func]
/// Encode the transaction with the domain separation (e.g. "TX") prefix
pub fn encode_transaction(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = tx.try_into()?;
    Ok(ctx.encode()?)
}

/// Encode transactions to MsgPack with the domain separation (e.g. "TX") prefix.
///
/// # Parameters
/// * `txs` - A collection of transactions to encode
///
/// # Returns
/// A collection of MsgPack encoded bytes or an error if encoding fails.
#[cfg(feature = "ffi_wasm")]
#[ffi_func]
/// Encode transactions with the domain separation (e.g. "TX") prefix
pub fn encode_transactions(txs: Vec<Transaction>) -> Result<Vec<Uint8Array>, AlgoKitTransactError> {
    txs.into_iter()
        .map(|tx| encode_transaction(tx).map(|bytes| bytes.as_slice().into()))
        .collect()
}

/// Encode transactions to MsgPack with the domain separation (e.g. "TX") prefix.
///
/// # Parameters
/// * `txs` - A collection of transactions to encode
///
/// # Returns
/// A collection of MsgPack encoded bytes or an error if encoding fails.
#[cfg(not(feature = "ffi_wasm"))]
#[ffi_func]
pub fn encode_transactions(txs: Vec<Transaction>) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    txs.into_iter().map(encode_transaction).collect()
}

#[ffi_func]
/// Encode the transaction without the domain separation (e.g. "TX") prefix
/// This is useful for encoding the transaction for signing with tools that automatically add "TX" prefix to the transaction bytes.
pub fn encode_transaction_raw(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = tx.try_into()?;
    Ok(ctx.encode_raw()?)
}

/// Decodes MsgPack bytes into a transaction.
///
/// # Parameters
/// * `encoded_tx` - MsgPack encoded bytes representing a transaction.
///
/// # Returns
/// A decoded transaction or an error if decoding fails.
#[ffi_func]
pub fn decode_transaction(encoded_tx: &[u8]) -> Result<Transaction, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = algokit_transact::Transaction::decode(encoded_tx)?;
    ctx.try_into()
}

/// Decodes a collection of MsgPack bytes into a transaction collection.
///
/// # Parameters
/// * `encoded_txs` - A collection of MsgPack encoded bytes, each representing a transaction.
///
/// # Returns
/// A collection of decoded transactions or an error if decoding fails.
#[cfg(feature = "ffi_wasm")]
#[ffi_func]
pub fn decode_transactions(
    encoded_txs: Vec<Uint8Array>,
) -> Result<Vec<Transaction>, AlgoKitTransactError> {
    encoded_txs
        .iter()
        .map(|bytes| decode_transaction(bytes.to_vec().as_slice()))
        .collect()
}

/// Decodes a collection of MsgPack bytes into a transaction collection.
///
/// # Parameters
/// * `encoded_txs` - A collection of MsgPack encoded bytes, each representing a transaction.
///
/// # Returns
/// A collection of decoded transactions or an error if decoding fails.
#[cfg(not(feature = "ffi_wasm"))]
#[ffi_func]
pub fn decode_transactions(
    encoded_txs: Vec<Vec<u8>>,
) -> Result<Vec<Transaction>, AlgoKitTransactError> {
    encoded_txs
        .iter()
        .map(|tx| decode_transaction(tx))
        .collect()
}

/// Return the size of the transaction in bytes as if it was already signed and encoded.
/// This is useful for estimating the fee for the transaction.
#[ffi_func]
pub fn estimate_transaction_size(transaction: Transaction) -> Result<u64, AlgoKitTransactError> {
    let core_tx: algokit_transact::Transaction = transaction.try_into()?;
    core_tx
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
        })
}

#[ffi_func]
pub fn account_from_pub_key(pub_key: &[u8]) -> Result<Account, AlgoKitTransactError> {
    Ok(
        algokit_transact::Account::from_pubkey(pub_key.try_into().map_err(|_| {
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
pub fn account_from_address(address: &str) -> Result<Account, AlgoKitTransactError> {
    address
        .parse::<algokit_transact::Account>()
        .map(Into::into)
        .map_err(|e| AlgoKitTransactError::EncodingError(e.to_string()))
}

/// Get the raw 32-byte transaction ID for a transaction.
#[ffi_func]
pub fn get_transaction_id_raw(tx: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let tx_internal: algokit_transact::Transaction = tx.try_into()?;
    let id_raw = tx_internal.id_raw()?;
    Ok(id_raw.to_vec())
}

/// Get the base32 transaction ID string for a transaction.
#[ffi_func]
pub fn get_transaction_id(tx: Transaction) -> Result<String, AlgoKitTransactError> {
    let tx_internal: algokit_transact::Transaction = tx.try_into()?;
    Ok(tx_internal.id()?)
}

/// Groups a collection of transactions by calculating and assigning the group to each transaction.
#[ffi_func]
pub fn group_transactions(txs: Vec<Transaction>) -> Result<Vec<Transaction>, AlgoKitTransactError> {
    let txs_internal: Vec<algokit_transact::Transaction> = txs
        .into_iter()
        .map(|tx| tx.try_into())
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

    /// The maximum number of transactions in a group (16)
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

impl TryFrom<FeeParams> for algokit_transact::FeeParams {
    type Error = AlgoKitTransactError;

    fn try_from(value: FeeParams) -> Result<Self, Self::Error> {
        Ok(Self {
            fee_per_byte: value.fee_per_byte,
            min_fee: value.min_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
        })
    }
}

#[ffi_func]
pub fn assign_fee(
    txn: Transaction,
    fee_params: FeeParams,
) -> Result<Transaction, AlgoKitTransactError> {
    let txn_internal: algokit_transact::Transaction = txn.try_into()?;
    let fee_params_internal: algokit_transact::FeeParams = fee_params.try_into()?;

    let updated_txn = txn_internal.assign_fee(fee_params_internal)?;

    updated_txn.try_into()
}

/// Decodes a signed transaction.
///
/// # Parameters
/// * `bytes` - The MsgPack encoded signed transaction bytes
///
/// # Returns
/// The decoded SignedTransaction or an error if decoding fails.
#[ffi_func]
pub fn decode_signed_transaction(bytes: &[u8]) -> Result<SignedTransaction, AlgoKitTransactError> {
    let signed_tx = algokit_transact::SignedTransaction::decode(bytes)?;
    Ok(signed_tx.into())
}

/// Decodes a collection of MsgPack bytes into a signed transaction collection.
///
/// # Parameters
/// * `encoded_signed_txs` - A collection of MsgPack encoded bytes, each representing a signed transaction.
///
/// # Returns
/// A collection of decoded signed transactions or an error if decoding fails.
#[cfg(feature = "ffi_wasm")]
#[ffi_func]
pub fn decode_signed_transactions(
    encoded_signed_txs: Vec<Uint8Array>,
) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
    encoded_signed_txs
        .iter()
        .map(|bytes| decode_signed_transaction(bytes.to_vec().as_slice()))
        .collect()
}

/// Decodes a collection of MsgPack bytes into a signed transaction collection.
///
/// # Parameters
/// * `encoded_signed_txs` - A collection of MsgPack encoded bytes, each representing a signed transaction.
///
/// # Returns
/// A collection of decoded signed transactions or an error if decoding fails.
#[cfg(not(feature = "ffi_wasm"))]
#[ffi_func]
pub fn decode_signed_transactions(
    encoded_signed_txs: Vec<Vec<u8>>,
) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
    encoded_signed_txs
        .iter()
        .map(|tx| decode_signed_transaction(tx))
        .collect()
}

/// Encode a signed transaction to MsgPack for sending on the network.
///
/// This method performs canonical encoding. No domain separation prefix is applicable.
///
/// # Parameters
/// * `signed_tx` - The signed transaction to encode
///
/// # Returns
/// The MsgPack encoded bytes or an error if encoding fails.
#[ffi_func]
pub fn encode_signed_transaction(
    signed_tx: SignedTransaction,
) -> Result<Vec<u8>, AlgoKitTransactError> {
    let signed_tx_internal: algokit_transact::SignedTransaction = signed_tx.try_into()?;
    Ok(signed_tx_internal.encode()?)
}

/// Encode signed transactions to MsgPack for sending on the network.
///
/// This method performs canonical encoding. No domain separation prefix is applicable.
///
/// # Parameters
/// * `signed_txs` - A collection of signed transactions to encode
///
/// # Returns
/// A collection of MsgPack encoded bytes or an error if encoding fails.
#[cfg(feature = "ffi_wasm")]
#[ffi_func]
pub fn encode_signed_transactions(
    signed_txs: Vec<SignedTransaction>,
) -> Result<Vec<Uint8Array>, AlgoKitTransactError> {
    signed_txs
        .into_iter()
        .map(|tx| encode_signed_transaction(tx).map(|bytes| bytes.as_slice().into()))
        .collect()
}

/// Encode signed transactions to MsgPack for sending on the network.
///
/// This method performs canonical encoding. No domain separation prefix is applicable.
///
/// # Parameters
/// * `signed_txs` - A collection of signed transactions to encode
///
/// # Returns
/// A collection of MsgPack encoded bytes or an error if encoding fails.
#[cfg(not(feature = "ffi_wasm"))]
#[ffi_func]
pub fn encode_signed_transactions(
    signed_txs: Vec<SignedTransaction>,
) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    signed_txs
        .into_iter()
        .map(encode_signed_transaction)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{TestDataMother, TransactionMother};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_encoded_payment_transaction_type() {
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
    fn test_get_encoded_asset_transfer_transaction_type() {
        let txn: Transaction = TransactionMother::simple_asset_transfer()
            .build()
            .unwrap()
            .try_into()
            .unwrap();

        // Encode the transaction
        let encoded = encode_transaction(txn).unwrap();

        // Test the get_encoded_transaction_type function
        let tx_type = get_encoded_transaction_type(&encoded).unwrap();
        assert_eq!(tx_type, TransactionType::AssetTransfer);
    }

    #[test]
    fn test_payment_transaction_id_ffi() {
        let data = TestDataMother::simple_payment();
        let tx_ffi: Transaction = data.transaction.try_into().unwrap();

        let actual_id = get_transaction_id(tx_ffi.clone()).unwrap();
        let actual_id_raw = get_transaction_id_raw(tx_ffi.clone()).unwrap();

        assert_eq!(actual_id, data.id);
        assert_eq!(actual_id_raw, data.id_raw);
    }

    #[test]
    fn test_asset_transfer_transaction_id_ffi() {
        let data = TestDataMother::simple_asset_transfer();
        let tx_ffi: Transaction = data.transaction.try_into().unwrap();

        let actual_id = get_transaction_id(tx_ffi.clone()).unwrap();
        let actual_id_raw = get_transaction_id_raw(tx_ffi.clone()).unwrap();

        assert_eq!(actual_id, data.id);
        assert_eq!(actual_id_raw, data.id_raw);
    }

    #[test]
    fn test_group_transactions_ffi() {
        let expected_group = [
            157, 37, 101, 171, 205, 211, 38, 98, 250, 86, 254, 215, 115, 126, 212, 252, 24, 53,
            199, 142, 152, 75, 250, 200, 173, 128, 52, 142, 13, 193, 184, 137,
        ];
        let tx1 = TestDataMother::simple_payment()
            .transaction
            .try_into()
            .unwrap();
        let tx2 = TestDataMother::simple_asset_transfer()
            .transaction
            .try_into()
            .unwrap();
        let tx3 = TestDataMother::opt_in_asset_transfer()
            .transaction
            .try_into()
            .unwrap();
        let txs = vec![tx1, tx2, tx3];

        let grouped_txs = group_transactions(txs.clone()).unwrap();

        assert_eq!(grouped_txs.len(), txs.len());
        for grouped_tx in grouped_txs.into_iter() {
            assert_eq!(grouped_tx.group.unwrap(), &expected_group);
        }
    }
}
