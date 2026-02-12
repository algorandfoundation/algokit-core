use algokit_crypto::ed25519::{
    CryptoxideEd25519Keypair as RustCryptoxideEd25519Keypair, Ed25519Signer as RustEd25519Signer,
};

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

/// FFI-compatible error type for crypto operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitCryptoError {
    Error { err_msg: String },
}

impl std::fmt::Display for AlgoKitCryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgoKitCryptoError::Error { err_msg: message } => write!(f, "{}", message),
        }
    }
}

impl From<String> for AlgoKitCryptoError {
    fn from(message: String) -> Self {
        AlgoKitCryptoError::Error { err_msg: message }
    }
}

#[uniffi::export]
pub fn ed25519_raw_sign(secret_key: Vec<u8>, data: Vec<u8>) -> Result<Vec<u8>, AlgoKitCryptoError> {
    // Spin up a Tokio runtime to perform the async signing operation
    // eventually we want to have the exported function be async, but for now
    // we need to be sync to maintain swift 6 compatibility
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| AlgoKitCryptoError::from(format!("Failed to build Tokio runtime: {}", e)))?;

    let keypair =
        RustCryptoxideEd25519Keypair::try_generate(Some(secret_key.try_into().map_err(|_| {
            AlgoKitCryptoError::from("Secret key must be 32 bytes for Ed25519".to_string())
        })?))
        .map_err(|e| {
            AlgoKitCryptoError::from(format!("Failed to generate keypair from secret key: {}", e))
        })?;

    let signature = rt
        .block_on(keypair.try_sign(&data))
        .map_err(|e| AlgoKitCryptoError::from(format!("Failed to sign transaction: {}", e)))?;

    Ok(signature.to_vec())
}
