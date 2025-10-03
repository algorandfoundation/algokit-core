mod multisig;
pub mod transactions;

use algokit_transact::constants::*;
use algokit_transact::{
    AlgorandMsgpack, EstimateTransactionSize, TransactionId, Transactions, Validate,
};
use ffi_macros::{ffi_enum, ffi_func, ffi_record};
use serde::{Deserialize, Serialize};

pub use multisig::{MultisigSignature, MultisigSubsignature};
pub use transactions::AppCallTransactionFields;
pub use transactions::AssetConfigTransactionFields;
pub use transactions::AssetFreezeTransactionFields;
pub use transactions::AssetTransferTransactionFields;
pub use transactions::KeyRegistrationTransactionFields;
pub use transactions::PaymentTransactionFields;
pub use transactions::StateProofTransactionFields;
pub use transactions::{HeartbeatProof, HeartbeatTransactionFields};

use snafu::Snafu;

// snafu is used to easily create errors than can be propagated to the language bindings
// UniFFI will create classes for errors (i.e. `MsgPackError.EncodingError` in Python)
#[derive(Debug, Snafu)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitTransactError {
    #[snafu(display("EncodingError: {message}"))]
    EncodingError { message: String },
    #[snafu(display("DecodingError: {message}"))]
    DecodingError { message: String },
    #[snafu(display("{message}"))]
    InputError { message: String },
    #[snafu(display("MsgPackError: {message}"))]
    MsgPackError { message: String },
}

// Convert errors from the Rust crate into the FFI-specific errors
impl From<algokit_transact::AlgoKitTransactError> for AlgoKitTransactError {
    fn from(e: algokit_transact::AlgoKitTransactError) -> Self {
        match e {
            algokit_transact::AlgoKitTransactError::DecodingError { .. } => {
                AlgoKitTransactError::DecodingError {
                    message: e.to_string(),
                }
            }
            algokit_transact::AlgoKitTransactError::EncodingError { .. } => {
                AlgoKitTransactError::EncodingError {
                    message: e.to_string(),
                }
            }
            algokit_transact::AlgoKitTransactError::MsgpackDecodingError { .. } => {
                AlgoKitTransactError::DecodingError {
                    message: e.to_string(),
                }
            }
            algokit_transact::AlgoKitTransactError::MsgpackEncodingError { .. } => {
                AlgoKitTransactError::EncodingError {
                    message: e.to_string(),
                }
            }
            algokit_transact::AlgoKitTransactError::UnknownTransactionType { .. } => {
                AlgoKitTransactError::DecodingError {
                    message: e.to_string(),
                }
            }
            algokit_transact::AlgoKitTransactError::InputError { message } => {
                AlgoKitTransactError::InputError { message }
            }
            algokit_transact::AlgoKitTransactError::InvalidAddress { .. } => {
                AlgoKitTransactError::DecodingError {
                    message: e.to_string(),
                }
            }
            algokit_transact::AlgoKitTransactError::InvalidMultisigSignature { .. } => {
                AlgoKitTransactError::DecodingError {
                    message: e.to_string(),
                }
            }
        }
    }
}

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

// This becomes an enum in UniFFI language bindings and a
// string literal union in TS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum TransactionType {
    Payment,
    AssetTransfer,
    AssetFreeze,
    AssetConfig,
    KeyRegistration,
    AppCall,
    Heartbeat,
    StateProof,
}

#[ffi_record]
pub struct KeyPairAccount {
    pub_key: Vec<u8>,
}

impl From<algokit_transact::KeyPairAccount> for KeyPairAccount {
    fn from(value: algokit_transact::KeyPairAccount) -> Self {
        Self {
            pub_key: value.pub_key.to_vec(),
        }
    }
}

impl TryFrom<KeyPairAccount> for algokit_transact::KeyPairAccount {
    type Error = AlgoKitTransactError;

    fn try_from(value: KeyPairAccount) -> Result<Self, Self::Error> {
        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] =
            vec_to_array(&value.pub_key, "public key").map_err(|e| {
                AlgoKitTransactError::DecodingError {
                    message: format!("Error while decoding a public key: {}", e),
                }
            })?;

        Ok(algokit_transact::KeyPairAccount::from_pubkey(&pub_key))
    }
}

impl From<algokit_transact::Address> for KeyPairAccount {
    fn from(value: algokit_transact::Address) -> Self {
        Self {
            pub_key: value.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<KeyPairAccount> for algokit_transact::Address {
    type Error = AlgoKitTransactError;

    fn try_from(value: KeyPairAccount) -> Result<Self, Self::Error> {
        let impl_keypair_account: algokit_transact::KeyPairAccount = value.try_into()?;
        Ok(impl_keypair_account.address())
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

    genesis_hash: Option<Vec<u8>>,

    genesis_id: Option<String>,

    note: Option<Vec<u8>>,

    rekey_to: Option<String>,

    lease: Option<Vec<u8>>,

    group: Option<Vec<u8>>,

    payment: Option<PaymentTransactionFields>,

    asset_transfer: Option<AssetTransferTransactionFields>,

    asset_config: Option<AssetConfigTransactionFields>,

    app_call: Option<AppCallTransactionFields>,

    key_registration: Option<KeyRegistrationTransactionFields>,

    asset_freeze: Option<AssetFreezeTransactionFields>,

    heartbeat: Option<HeartbeatTransactionFields>,

    state_proof: Option<StateProofTransactionFields>,
}

impl TryFrom<Transaction> for algokit_transact::Transaction {
    type Error = AlgoKitTransactError;

    fn try_from(transaction: Transaction) -> Result<Self, AlgoKitTransactError> {
        // Ensure there is never more than 1 transaction type specific field set
        if [
            transaction.payment.is_some(),
            transaction.asset_transfer.is_some(),
            transaction.asset_config.is_some(),
            transaction.key_registration.is_some(),
            transaction.app_call.is_some(),
            transaction.asset_freeze.is_some(),
            transaction.heartbeat.is_some(),
            transaction.state_proof.is_some(),
        ]
        .into_iter()
        .filter(|&x| x)
        .count()
            > 1
        {
            return Err(Self::Error::DecodingError {
                message: "Multiple transaction type specific fields set".to_string(),
            });
        }

        match transaction.transaction_type {
            TransactionType::Payment => Ok(algokit_transact::Transaction::Payment(
                transaction.try_into()?,
            )),
            TransactionType::AssetTransfer => Ok(algokit_transact::Transaction::AssetTransfer(
                transaction.try_into()?,
            )),
            TransactionType::KeyRegistration => Ok(algokit_transact::Transaction::KeyRegistration(
                transaction.try_into()?,
            )),

            TransactionType::AssetConfig => Ok(algokit_transact::Transaction::AssetConfig(
                transaction.try_into()?,
            )),

            TransactionType::AppCall => Ok(algokit_transact::Transaction::AppCall(
                transaction.try_into()?,
            )),
            TransactionType::AssetFreeze => Ok(algokit_transact::Transaction::AssetFreeze(
                transaction.try_into()?,
            )),
            TransactionType::Heartbeat => Ok(algokit_transact::Transaction::Heartbeat(
                transaction.try_into()?,
            )),
            TransactionType::StateProof => Ok(algokit_transact::Transaction::StateProof(
                transaction.try_into()?,
            )),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::TransactionHeader {
    type Error = AlgoKitTransactError;

    fn try_from(transaction: Transaction) -> Result<Self, AlgoKitTransactError> {
        Ok(Self {
            sender: transaction.sender.parse()?,
            fee: transaction.fee,
            first_valid: transaction.first_valid,
            last_valid: transaction.last_valid,
            genesis_id: transaction.genesis_id,
            genesis_hash: transaction
                .genesis_hash
                .map(|buf| vec_to_array::<32>(&buf, "genesis hash"))
                .transpose()?,
            note: transaction.note,
            rekey_to: transaction.rekey_to.map(|addr| addr.parse()).transpose()?,
            lease: transaction
                .lease
                .map(|buf| vec_to_array::<32>(&buf, "lease"))
                .transpose()?,
            group: transaction
                .group
                .map(|buf| vec_to_array::<32>(&buf, "group ID"))
                .transpose()?,
        })
    }
}

impl From<algokit_transact::Transaction> for Transaction {
    fn from(transaction: algokit_transact::Transaction) -> Self {
        match transaction {
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
                    None,
                    None,
                )
            }
            algokit_transact::Transaction::AppCall(app_call) => {
                let app_call_fields = app_call.clone().into();
                build_transaction(
                    app_call.header,
                    TransactionType::AppCall,
                    None,
                    None,
                    None,
                    Some(app_call_fields),
                    None,
                    None,
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
                    None,
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
                    None,
                    None,
                )
            }
            algokit_transact::Transaction::Heartbeat(heartbeat) => {
                let heartbeat_fields = heartbeat.clone().into();
                build_transaction(
                    heartbeat.header,
                    TransactionType::Heartbeat,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(heartbeat_fields),
                    None,
                )
            }
            algokit_transact::Transaction::StateProof(state_proof) => {
                let state_proof_fields = state_proof.clone().into();
                build_transaction(
                    state_proof.header,
                    TransactionType::StateProof,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(state_proof_fields),
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
    pub signature: Option<Vec<u8>>,

    /// Optional auth address applicable if the transaction sender is a rekeyed account.
    pub auth_address: Option<String>,

    /// Optional multisig signature if the transaction is a multisig transaction.
    pub multisignature: Option<MultisigSignature>,
}

impl From<algokit_transact::SignedTransaction> for SignedTransaction {
    fn from(signed_transaction: algokit_transact::SignedTransaction) -> Self {
        Self {
            transaction: signed_transaction.transaction.into(),
            signature: signed_transaction.signature.map(|sig| sig.into()),
            auth_address: signed_transaction.auth_address.map(|addr| addr.as_str()),
            multisignature: signed_transaction.multisignature.map(Into::into),
        }
    }
}

impl TryFrom<SignedTransaction> for algokit_transact::SignedTransaction {
    type Error = AlgoKitTransactError;

    fn try_from(signed_transaction: SignedTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: signed_transaction.transaction.try_into()?,
            signature: signed_transaction
                .signature
                .map(|sig| vec_to_array(&sig, "signature"))
                .transpose()
                .map_err(|e| AlgoKitTransactError::DecodingError {
                    message: format!(
                        "Error while decoding the signature in a signed transaction: {}",
                        e
                    ),
                })?,
            auth_address: signed_transaction
                .auth_address
                .map(|addr| addr.parse())
                .transpose()?,
            multisignature: signed_transaction
                .multisignature
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

fn vec_to_array<const N: usize>(
    buf: &[u8],
    context: &str,
) -> Result<[u8; N], AlgoKitTransactError> {
    buf.to_vec()
        .try_into()
        .map_err(|_| AlgoKitTransactError::DecodingError {
            message: format!(
                "Expected {} {} bytes but got {} bytes",
                context,
                N,
                buf.len(),
            ),
        })
}

#[allow(clippy::too_many_arguments)]
fn build_transaction(
    header: algokit_transact::TransactionHeader,
    transaction_type: TransactionType,
    payment: Option<PaymentTransactionFields>,
    asset_transfer: Option<AssetTransferTransactionFields>,
    asset_config: Option<AssetConfigTransactionFields>,
    app_call: Option<AppCallTransactionFields>,
    key_registration: Option<KeyRegistrationTransactionFields>,
    asset_freeze: Option<AssetFreezeTransactionFields>,
    heartbeat: Option<HeartbeatTransactionFields>,
    state_proof: Option<StateProofTransactionFields>,
) -> Transaction {
    Transaction {
        transaction_type,
        sender: header.sender.as_str(),
        fee: header.fee,
        first_valid: header.first_valid,
        last_valid: header.last_valid,
        genesis_id: header.genesis_id,
        genesis_hash: header.genesis_hash.map(Into::into),
        note: header.note,
        rekey_to: header.rekey_to.map(|addr| addr.as_str()),
        lease: header.lease.map(Into::into),
        group: header.group.map(Into::into),
        payment,
        asset_transfer,
        asset_config,
        app_call,
        key_registration,
        asset_freeze,
        heartbeat,
        state_proof,
    }
}

/// Get the transaction type from the encoded transaction.
/// This is particularly useful when decoding a transaction that has an unknown type
#[ffi_func]
pub fn get_encoded_transaction_type(
    encoded_transaction: &[u8],
) -> Result<TransactionType, AlgoKitTransactError> {
    let decoded = algokit_transact::Transaction::decode(encoded_transaction)?;

    match decoded {
        algokit_transact::Transaction::Payment(_) => Ok(TransactionType::Payment),
        algokit_transact::Transaction::AssetTransfer(_) => Ok(TransactionType::AssetTransfer),
        algokit_transact::Transaction::AssetConfig(_) => Ok(TransactionType::AssetConfig),
        algokit_transact::Transaction::AppCall(_) => Ok(TransactionType::AppCall),
        algokit_transact::Transaction::KeyRegistration(_) => Ok(TransactionType::KeyRegistration),
        algokit_transact::Transaction::AssetFreeze(_) => Ok(TransactionType::AssetFreeze),
        algokit_transact::Transaction::Heartbeat(_) => Ok(TransactionType::Heartbeat),
        algokit_transact::Transaction::StateProof(_) => Ok(TransactionType::StateProof),
    }
}

#[ffi_func]
/// Encode the transaction with the domain separation (e.g. "TX") prefix
pub fn encode_transaction(transaction: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = transaction.try_into()?;
    Ok(ctx.encode()?)
}

/// Encode transactions to MsgPack with the domain separation (e.g. "TX") prefix.
///
/// # Parameters
/// * `transactions` - A collection of transactions to encode
///
/// # Returns
/// A collection of MsgPack encoded bytes or an error if encoding fails.
#[ffi_func]
pub fn encode_transactions(
    transactions: Vec<Transaction>,
) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    transactions.into_iter().map(encode_transaction).collect()
}

#[ffi_func]
/// Encode the transaction without the domain separation (e.g. "TX") prefix
/// This is useful for encoding the transaction for signing with tools that automatically add "TX" prefix to the transaction bytes.
pub fn encode_transaction_raw(transaction: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let ctx: algokit_transact::Transaction = transaction.try_into()?;
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
    Ok(ctx.into())
}

/// Decodes a collection of MsgPack bytes into a transaction collection.
///
/// # Parameters
/// * `encoded_txs` - A collection of MsgPack encoded bytes, each representing a transaction.
///
/// # Returns
/// A collection of decoded transactions or an error if decoding fails.
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
        .estimate_size()?
        .try_into()
        .map_err(|_| AlgoKitTransactError::EncodingError {
            message: "Failed to convert size to u64".to_string(),
        })
}

#[ffi_func]
pub fn address_from_public_key(public_key: &[u8]) -> Result<String, AlgoKitTransactError> {
    Ok(
        algokit_transact::KeyPairAccount::from_pubkey(public_key.try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError {
                message: format!(
                    "public key should be {} bytes",
                    ALGORAND_PUBLIC_KEY_BYTE_LENGTH
                ),
            }
        })?)
        .to_string(),
    )
}

#[ffi_func]
pub fn public_key_from_address(address: &str) -> Result<Vec<u8>, AlgoKitTransactError> {
    Ok(address
        .parse::<algokit_transact::KeyPairAccount>()
        .map(|a| a.pub_key.to_vec())?)
}

/// Get the raw 32-byte transaction ID for a transaction.
#[ffi_func]
pub fn get_transaction_id_raw(transaction: Transaction) -> Result<Vec<u8>, AlgoKitTransactError> {
    let tx: algokit_transact::Transaction = transaction.try_into()?;
    let id_raw = tx.id_raw()?;
    Ok(id_raw.to_vec())
}

/// Get the base32 transaction ID string for a transaction.
#[ffi_func]
pub fn get_transaction_id(transaction: Transaction) -> Result<String, AlgoKitTransactError> {
    let tx: algokit_transact::Transaction = transaction.try_into()?;
    Ok(tx.id()?)
}

/// Groups a collection of transactions by calculating and assigning the group to each transaction.
#[ffi_func]
pub fn group_transactions(
    transactions: Vec<Transaction>,
) -> Result<Vec<Transaction>, AlgoKitTransactError> {
    let txs: Vec<algokit_transact::Transaction> = transactions
        .into_iter()
        .map(|tx| tx.try_into())
        .collect::<Result<Vec<_>, _>>()?;

    let grouped_txs: Vec<Transaction> = txs
        .assign_group()?
        .into_iter()
        .map(|tx| tx.into())
        .collect();

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
pub fn calculate_fee(
    transaction: Transaction,
    fee_params: FeeParams,
) -> Result<u64, AlgoKitTransactError> {
    let txn: algokit_transact::Transaction = transaction.try_into()?;
    let fee_params_internal: algokit_transact::FeeParams = fee_params.try_into()?;
    Ok(txn.calculate_fee(fee_params_internal)?)
}

#[ffi_func]
pub fn assign_fee(
    transaction: Transaction,
    fee_params: FeeParams,
) -> Result<Transaction, AlgoKitTransactError> {
    let txn: algokit_transact::Transaction = transaction.try_into()?;
    let fee_params_internal: algokit_transact::FeeParams = fee_params.try_into()?;

    let updated_txn = txn.assign_fee(fee_params_internal)?;

    Ok(updated_txn.into())
}

/// Decodes a signed transaction.
///
/// # Parameters
/// * `encoded_signed_transaction` - The MsgPack encoded signed transaction bytes
///
/// # Returns
/// The decoded SignedTransaction or an error if decoding fails.
#[ffi_func]
pub fn decode_signed_transaction(
    encoded_signed_transaction: &[u8],
) -> Result<SignedTransaction, AlgoKitTransactError> {
    let signed_transaction =
        algokit_transact::SignedTransaction::decode(encoded_signed_transaction)?;
    Ok(signed_transaction.into())
}

/// Decodes a collection of MsgPack bytes into a signed transaction collection.
///
/// # Parameters
/// * `encoded_signed_transactions` - A collection of MsgPack encoded bytes, each representing a signed transaction.
///
/// # Returns
/// A collection of decoded signed transactions or an error if decoding fails.
#[ffi_func]
pub fn decode_signed_transactions(
    encoded_signed_transactions: Vec<Vec<u8>>,
) -> Result<Vec<SignedTransaction>, AlgoKitTransactError> {
    encoded_signed_transactions
        .iter()
        .map(|tx| decode_signed_transaction(tx))
        .collect()
}

/// Encode a signed transaction to MsgPack for sending on the network.
///
/// This method performs canonical encoding. No domain separation prefix is applicable.
///
/// # Parameters
/// * `signed_transaction` - The signed transaction to encode
///
/// # Returns
/// The MsgPack encoded bytes or an error if encoding fails.
#[ffi_func]
pub fn encode_signed_transaction(
    signed_transaction: SignedTransaction,
) -> Result<Vec<u8>, AlgoKitTransactError> {
    let stx: algokit_transact::SignedTransaction = signed_transaction.try_into()?;
    Ok(stx.encode()?)
}

/// Encode signed transactions to MsgPack for sending on the network.
///
/// This method performs canonical encoding. No domain separation prefix is applicable.
///
/// # Parameters
/// * `signed_transactions` - A collection of signed transactions to encode
///
/// # Returns
/// A collection of MsgPack encoded bytes or an error if encoding fails.
#[ffi_func]
pub fn encode_signed_transactions(
    signed_transactions: Vec<SignedTransaction>,
) -> Result<Vec<Vec<u8>>, AlgoKitTransactError> {
    signed_transactions
        .into_iter()
        .map(encode_signed_transaction)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_group_transactions_ffi() {
        let expected_group = [
            157, 37, 101, 171, 205, 211, 38, 98, 250, 86, 254, 215, 115, 126, 212, 252, 24, 53,
            199, 142, 152, 75, 250, 200, 173, 128, 52, 142, 13, 193, 184, 137,
        ];
        let tx1 = TestDataMother::simple_payment().transaction.into();
        let tx2 = TestDataMother::simple_asset_transfer().transaction.into();
        let tx3 = TestDataMother::opt_in_asset_transfer().transaction.into();
        let txs = vec![tx1, tx2, tx3];

        let grouped_txs = group_transactions(txs.clone()).unwrap();

        assert_eq!(grouped_txs.len(), txs.len());
        for grouped_tx in grouped_txs.into_iter() {
            assert_eq!(grouped_tx.group.unwrap(), &expected_group);
        }
    }
}
