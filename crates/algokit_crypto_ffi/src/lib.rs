use algokit_crypto::ed25519::{
    CryptoxideEd25519Keypair as RustCryptoxideEd25519Keypair,
    Ed25519KeyAndSigner as RustEd25519KeyAndSigner, Ed25519Signer as RustEd25519Signer,
};
use async_trait::async_trait;
use signature::Keypair;
use std::future::Future;
use std::sync::Arc;

#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

/// FFI-compatible error type for crypto operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitCryptoError {
    Error { message: String },
}

impl std::fmt::Display for AlgoKitCryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgoKitCryptoError::Error { message } => write!(f, "{}", message),
        }
    }
}

impl From<String> for AlgoKitCryptoError {
    fn from(message: String) -> Self {
        AlgoKitCryptoError::Error { message }
    }
}

/// FFI-compatible trait for Ed25519 signing operations
///
/// This trait is exported with `with_foreign` to allow foreign languages (Python, Swift, Kotlin, etc.)
/// to implement it and provide custom signing logic.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export(with_foreign))]
pub trait Ed25519Signer: Send + Sync {
    fn try_sign(&self, msg: Vec<u8>) -> Result<Vec<u8>, AlgoKitCryptoError>;
}

/// FFI-compatible trait that combines signing and keypair operations for Ed25519
///
/// This trait is exported with `with_foreign` to allow foreign languages to implement it.
/// Note: We don't use supertrait relationship with Ed25519SignerFfi because UniFFI's
/// with_foreign doesn't support trait inheritance properly. Instead, we duplicate the
/// try_sign method.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export(with_foreign))]
pub trait Ed25519KeyAndSigner: Send + Sync {
    fn try_sign(&self, msg: Vec<u8>) -> Result<Vec<u8>, AlgoKitCryptoError>;
    fn verifying_key(&self) -> Vec<u8>;
}

/// Wrapper struct to convert from FFI Ed25519Signer to Rust Ed25519Signer
struct RustEd25519SignerFromFfi {
    ffi_signer: Arc<dyn Ed25519Signer>,
}

fn block_on<F: Future>(future: F) -> F::Output {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.block_on(future)
    } else {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime")
            .block_on(future)
    }
}

#[async_trait]
impl RustEd25519Signer for RustEd25519SignerFromFfi {
    async fn try_sign(&self, msg: &[u8]) -> Result<[u8; 64], String> {
        let signature = self
            .ffi_signer
            .try_sign(msg.to_vec())
            .map_err(|e| match e {
                AlgoKitCryptoError::Error { message } => message,
            })?;

        signature
            .try_into()
            .map_err(|_| "Signature must be 64 bytes".to_string())
    }
}

/// Wrapper struct to convert from FFI Ed25519KeyAndSigner to Rust Ed25519KeyAndSigner
struct RustEd25519KeyAndSignerFromFfi {
    ffi_key_and_signer: Arc<dyn Ed25519KeyAndSigner>,
}

#[async_trait]
impl RustEd25519Signer for RustEd25519KeyAndSignerFromFfi {
    async fn try_sign(&self, msg: &[u8]) -> Result<[u8; 64], String> {
        let signature = self
            .ffi_key_and_signer
            .try_sign(msg.to_vec())
            .map_err(|e| match e {
                AlgoKitCryptoError::Error { message } => message,
            })?;

        signature
            .try_into()
            .map_err(|_| "Signature must be 64 bytes".to_string())
    }
}

impl signature::Keypair for RustEd25519KeyAndSignerFromFfi {
    type VerifyingKey = [u8; 32];

    fn verifying_key(&self) -> Self::VerifyingKey {
        let vk = self.ffi_key_and_signer.verifying_key();
        vk.try_into().expect("Verifying key must be 32 bytes")
    }
}

// Note: Ed25519KeyAndSigner is automatically implemented via blanket impl
// since RustEd25519KeyAndSignerFromFfi implements both Ed25519Signer and Keypair<VerifyingKey = [u8; 32]>

/// Wrapper struct to convert from Rust Ed25519Signer to FFI Ed25519Signer
struct FfiEd25519SignerFromRust {
    rust_signer: Arc<dyn RustEd25519Signer + Send + Sync>,
}

impl Ed25519Signer for FfiEd25519SignerFromRust {
    fn try_sign(&self, msg: Vec<u8>) -> Result<Vec<u8>, AlgoKitCryptoError> {
        let signature = block_on(self.rust_signer.try_sign(&msg))
            .map_err(|e| AlgoKitCryptoError::Error { message: e })?;
        Ok(signature.to_vec())
    }
}

/// Wrapper struct to convert from Rust Ed25519KeyAndSigner to FFI Ed25519KeyAndSigner
struct FfiEd25519KeyAndSignerFromRust {
    rust_key_and_signer: Arc<dyn RustEd25519KeyAndSigner + Send + Sync>,
}

impl Ed25519KeyAndSigner for FfiEd25519KeyAndSignerFromRust {
    fn try_sign(&self, msg: Vec<u8>) -> Result<Vec<u8>, AlgoKitCryptoError> {
        let signature = block_on(self.rust_key_and_signer.try_sign(&msg))
            .map_err(|e| AlgoKitCryptoError::Error { message: e })?;
        Ok(signature.to_vec())
    }

    fn verifying_key(&self) -> Vec<u8> {
        self.rust_key_and_signer.verifying_key().to_vec()
    }
}

/// FFI-compatible wrapper for CryptoxideEd25519Keypair
///
/// This struct wraps the Rust implementation and exposes it via FFI.
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Object))]
pub struct CryptoxideEd25519Keypair {
    inner: RustCryptoxideEd25519Keypair,
}

#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
impl CryptoxideEd25519Keypair {
    /// Generate a new keypair from an optional seed.
    /// If no seed is provided, a random seed is generated using the system's CSPRNG.
    #[cfg_attr(feature = "ffi_uniffi", uniffi::constructor)]
    pub fn try_generate(seed: Option<Vec<u8>>) -> Result<Self, AlgoKitCryptoError> {
        let seed_array: Option<[u8; 32]> = seed
            .map(|s| {
                s.try_into()
                    .map_err(|_| "Seed must be 32 bytes".to_string())
            })
            .transpose()
            .map_err(|e: String| AlgoKitCryptoError::Error { message: e })?;

        let inner = RustCryptoxideEd25519Keypair::try_generate(seed_array).map_err(|e| {
            AlgoKitCryptoError::Error {
                message: e.to_string(),
            }
        })?;

        Ok(Self { inner })
    }

    /// Get the verifying key (public key) as a byte vector
    pub fn verifying_key(&self) -> Vec<u8> {
        self.inner.verifying_key().to_vec()
    }

    /// Sign a message asynchronously
    pub async fn try_sign(&self, msg: Vec<u8>) -> Result<Vec<u8>, AlgoKitCryptoError> {
        let signature = self
            .inner
            .try_sign(&msg)
            .await
            .map_err(|e| AlgoKitCryptoError::Error { message: e })?;
        Ok(signature.to_vec())
    }
}

/// Helper function to wrap a Rust Ed25519Signer for FFI use
pub fn wrap_rust_signer(
    signer: Arc<dyn RustEd25519Signer + Send + Sync>,
) -> Arc<dyn Ed25519Signer> {
    Arc::new(FfiEd25519SignerFromRust {
        rust_signer: signer,
    })
}

/// Helper function to wrap a Rust Ed25519KeyAndSigner for FFI use
pub fn wrap_rust_key_and_signer(
    key_and_signer: Arc<dyn RustEd25519KeyAndSigner + Send + Sync>,
) -> Arc<dyn Ed25519KeyAndSigner> {
    Arc::new(FfiEd25519KeyAndSignerFromRust {
        rust_key_and_signer: key_and_signer,
    })
}

/// Helper function to wrap an FFI Ed25519Signer for Rust use
pub fn wrap_ffi_signer(signer: Arc<dyn Ed25519Signer>) -> Arc<dyn RustEd25519Signer + Send + Sync> {
    Arc::new(RustEd25519SignerFromFfi { ffi_signer: signer })
}

/// Helper function to wrap an FFI Ed25519KeyAndSigner for Rust use
pub fn wrap_ffi_key_and_signer(
    key_and_signer: Arc<dyn Ed25519KeyAndSigner>,
) -> Arc<dyn RustEd25519KeyAndSigner + Send + Sync> {
    Arc::new(RustEd25519KeyAndSignerFromFfi {
        ffi_key_and_signer: key_and_signer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cryptoxide_keypair_generation_and_signing() {
        let seed = vec![1u8; 32];
        let keypair =
            CryptoxideEd25519Keypair::try_generate(Some(seed)).expect("Failed to generate keypair");

        let message = b"Hello, Algorand!";
        let signature = keypair
            .try_sign(message.to_vec())
            .await
            .expect("Failed to sign message");

        assert_eq!(signature.len(), 64);

        let verifying_key = keypair.verifying_key();
        assert_eq!(verifying_key.len(), 32);
    }

    #[tokio::test]
    async fn test_cryptoxide_random_generation() {
        let keypair =
            CryptoxideEd25519Keypair::try_generate(None).expect("Failed to generate keypair");

        let message = b"Test message";
        let signature = keypair
            .try_sign(message.to_vec())
            .await
            .expect("Failed to sign message");

        assert_eq!(signature.len(), 64);

        let verifying_key = keypair.verifying_key();
        assert_eq!(verifying_key.len(), 32);
    }

    #[test]
    fn test_invalid_seed_length() {
        let seed = vec![1u8; 16]; // Wrong length
        let result = CryptoxideEd25519Keypair::try_generate(Some(seed));
        assert!(result.is_err());
    }
}
