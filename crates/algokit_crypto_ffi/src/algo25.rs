use algokit_crypto::algo25::{
    MnemonicError as RustMnemonicError,
    master_derivation_key_to_mnemonic as rust_master_derivation_key_to_mnemonic,
    mnemonic_from_seed as rust_mnemonic_from_seed,
    mnemonic_to_master_derivation_key as rust_mnemonic_to_master_derivation_key,
    secret_key_to_mnemonic as rust_secret_key_to_mnemonic,
    seed_from_mnemonic as rust_seed_from_mnemonic,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
pub enum MnemonicErrorKind {
    InvalidSeedLength,
    NotInWordsList,
    FailedToDecodeMnemonic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct MnemonicError {
    pub kind: MnemonicErrorKind,
    pub expected: Option<u64>,
    pub found: Option<u64>,
}

impl From<RustMnemonicError> for MnemonicError {
    fn from(value: RustMnemonicError) -> Self {
        match value {
            RustMnemonicError::InvalidSeedLength { expected, found } => Self {
                kind: MnemonicErrorKind::InvalidSeedLength,
                expected: Some(expected as u64),
                found: Some(found as u64),
            },
            RustMnemonicError::NotInWordsList => Self {
                kind: MnemonicErrorKind::NotInWordsList,
                expected: None,
                found: None,
            },
            RustMnemonicError::FailedToDecodeMnemonic => Self {
                kind: MnemonicErrorKind::FailedToDecodeMnemonic,
                expected: None,
                found: None,
            },
        }
    }
}

impl From<MnemonicError> for RustMnemonicError {
    fn from(value: MnemonicError) -> Self {
        match value.kind {
            MnemonicErrorKind::InvalidSeedLength => RustMnemonicError::InvalidSeedLength {
                expected: value.expected.unwrap_or_default() as usize,
                found: value.found.unwrap_or_default() as usize,
            },
            MnemonicErrorKind::NotInWordsList => RustMnemonicError::NotInWordsList,
            MnemonicErrorKind::FailedToDecodeMnemonic => RustMnemonicError::FailedToDecodeMnemonic,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitAlgo25Error {
    Error { err_msg: String },
}

impl std::fmt::Display for AlgoKitAlgo25Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgoKitAlgo25Error::Error { err_msg: message } => write!(f, "{}", message),
        }
    }
}

impl From<RustMnemonicError> for AlgoKitAlgo25Error {
    fn from(value: RustMnemonicError) -> Self {
        let ffi_error: MnemonicError = value.into();
        let details = match ffi_error.kind {
            MnemonicErrorKind::InvalidSeedLength => format!(
                "invalid seed length (expected {}, found {})",
                ffi_error.expected.unwrap_or_default(),
                ffi_error.found.unwrap_or_default(),
            ),
            MnemonicErrorKind::NotInWordsList => {
                "mnemonic contains a word that is not in the wordlist".to_string()
            }
            MnemonicErrorKind::FailedToDecodeMnemonic => "failed to decode mnemonic".to_string(),
        };

        Self::Error { err_msg: details }
    }
}

#[uniffi::export]
pub fn mnemonic_from_seed(seed: Vec<u8>) -> Result<String, AlgoKitAlgo25Error> {
    rust_mnemonic_from_seed(&seed).map_err(Into::into)
}

#[uniffi::export]
pub fn seed_from_mnemonic(mnemonic: &str) -> Result<Vec<u8>, AlgoKitAlgo25Error> {
    rust_seed_from_mnemonic(mnemonic)
        .map(|seed| seed.to_vec())
        .map_err(Into::into)
}

#[uniffi::export]
pub fn secret_key_to_mnemonic(secret_key: Vec<u8>) -> Result<String, AlgoKitAlgo25Error> {
    rust_secret_key_to_mnemonic(&secret_key).map_err(Into::into)
}

#[uniffi::export]
pub fn mnemonic_to_master_derivation_key(mnemonic: &str) -> Result<Vec<u8>, AlgoKitAlgo25Error> {
    rust_mnemonic_to_master_derivation_key(mnemonic)
        .map(|seed| seed.to_vec())
        .map_err(Into::into)
}

#[uniffi::export]
pub fn master_derivation_key_to_mnemonic(mdk: Vec<u8>) -> Result<String, AlgoKitAlgo25Error> {
    rust_master_derivation_key_to_mnemonic(&mdk).map_err(Into::into)
}
