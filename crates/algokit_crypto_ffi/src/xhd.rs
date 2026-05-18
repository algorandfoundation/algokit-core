//! UniFFI bindings for the [`algokit_crypto::xhd`] HD wallet primitives.
//!
//! Translates the strongly-typed core API into the byte-vector shape used by
//! UniFFI host bindings, and maps [`algokit_crypto::xhd::XhdError`] into the
//! FFI-compatible [`AlgoKitXhdError`].

use algokit_crypto::xhd::{
    self, DerivedAccount as RustDerivedAccount, KeyContext as RustKeyContext,
    XhdError as RustXhdError,
};

/// FFI-compatible mirror of [`algokit_crypto::xhd::KeyContext`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum XhdKeyContext {
    Address,
    Identity,
}

impl From<XhdKeyContext> for RustKeyContext {
    fn from(value: XhdKeyContext) -> Self {
        match value {
            XhdKeyContext::Address => RustKeyContext::Address,
            XhdKeyContext::Identity => RustKeyContext::Identity,
        }
    }
}

/// FFI-compatible mirror of [`algokit_crypto::xhd::DerivedAccount`] with byte
/// vectors in place of fixed-size arrays.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct XhdDerivedAccount {
    /// Extended private key: 32-byte scalar, 32-byte prefix, 32-byte chain code.
    pub extended_private_key: Vec<u8>,
    /// Ed25519 public key derived from the extended private key.
    pub public_key: Vec<u8>,
}

impl From<RustDerivedAccount> for XhdDerivedAccount {
    fn from(value: RustDerivedAccount) -> Self {
        Self {
            extended_private_key: value.xprv.to_vec(),
            public_key: value.public_key.to_vec(),
        }
    }
}

/// FFI-compatible error type for HD wallet operations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitXhdError {
    Error { err_msg: String },
}

impl std::fmt::Display for AlgoKitXhdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgoKitXhdError::Error { err_msg } => write!(f, "{}", err_msg),
        }
    }
}

impl From<RustXhdError> for AlgoKitXhdError {
    fn from(value: RustXhdError) -> Self {
        Self::Error {
            err_msg: value.to_string(),
        }
    }
}

/// Derives a 96-byte root extended private key from a 64-byte seed.
///
/// # Errors
///
/// Returns an error if `seed` is not exactly 64 bytes.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn xhd_root_key_from_seed(seed: Vec<u8>) -> Result<Vec<u8>, AlgoKitXhdError> {
    xhd::root_key_from_seed(&seed)
        .map(|xprv| xprv.to_vec())
        .map_err(Into::into)
}

/// Derives a 64-byte BIP39 seed from a 12/15/18/21/24-word mnemonic.
///
/// The BIP39 passphrase is hardcoded to the empty string; this binding does
/// not expose passphrase-based derivation.
///
/// # Errors
///
/// Returns an error if `mnemonic` is not a valid BIP39 phrase.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn xhd_seed_from_mnemonic(mnemonic: String) -> Result<Vec<u8>, AlgoKitXhdError> {
    xhd::seed_from_mnemonic(&mnemonic)
        .map(|seed| seed.to_vec())
        .map_err(Into::into)
}

/// Derives a 96-byte root extended private key directly from a BIP39 mnemonic.
///
/// Convenience wrapper around [`xhd_seed_from_mnemonic`] +
/// [`xhd_root_key_from_seed`].
///
/// # Errors
///
/// Returns an error if `mnemonic` is not a valid BIP39 phrase.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn xhd_root_key_from_mnemonic(mnemonic: String) -> Result<Vec<u8>, AlgoKitXhdError> {
    xhd::root_key_from_mnemonic(&mnemonic)
        .map(|xprv| xprv.to_vec())
        .map_err(Into::into)
}

/// Derives an extended private key at the BIP44 path
/// `m/44'/<coin>'/<account>'/0/<key_index>` using the Peikert derivation scheme.
///
/// The `coin` segment is determined by `key_context`: 283 for
/// [`XhdKeyContext::Address`] and 0 for [`XhdKeyContext::Identity`].
///
/// # Errors
///
/// Returns an error if `root_key` is not exactly 96 bytes or does not form a
/// valid extended private key.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn xhd_derive(
    root_key: Vec<u8>,
    key_context: XhdKeyContext,
    account: u32,
    key_index: u32,
) -> Result<XhdDerivedAccount, AlgoKitXhdError> {
    xhd::derive(&root_key, key_context.into(), account, key_index)
        .map(Into::into)
        .map_err(Into::into)
}

/// Signs `msg` with an already-derived 96-byte extended private key.
///
/// # Errors
///
/// Returns an error if `extended_key` is not exactly 96 bytes or does not form
/// a valid extended private key.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn xhd_raw_sign(extended_key: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>, AlgoKitXhdError> {
    xhd::raw_sign(&extended_key, &msg)
        .map(|sig| sig.to_vec())
        .map_err(Into::into)
}
