use async_trait::async_trait;
use cryptoxide::ed25519;
use signature::Keypair;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[async_trait]
pub trait Ed25519Signer {
    async fn try_sign(&self, msg: &[u8]) -> Result<[u8; 64], String>;
}

/// Trait that combines Signer and Keypair for Ed25519 (64-byte signature, 32-byte public key).
/// This allows using both traits in a trait object since Rust only allows one non-auto trait per trait object.
pub trait Ed25519KeyAndSigner: Ed25519Signer + Keypair<VerifyingKey = [u8; 32]> {}

impl<T> Ed25519KeyAndSigner for T where T: Ed25519Signer + Keypair<VerifyingKey = [u8; 32]> {}

/// An Ed25519 keypair implementation using the cryptoxide library.
/// The private key is automatically zeroed from memory when the struct is dropped.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct CryptoxideEd25519Keypair {
    /// The 64-byte keypair (32 bytes private key + 32 bytes public key).
    /// Marked with `zeroize` to ensure secure memory clearing.
    #[zeroize]
    keypair: [u8; 64],
}

impl CryptoxideEd25519Keypair {
    /// Generate a new keypair from an optional seed.
    /// If no seed is provided, a random seed is generated using the system's CSPRNG.
    pub fn try_generate(seed: Option<[u8; 32]>) -> Result<Self, getrandom::Error> {
        let seed = match seed {
            Some(s) => s,
            None => {
                let mut seed = [0u8; 32];
                getrandom::fill(&mut seed)?;
                seed
            }
        };
        let (keypair, _) = ed25519::keypair(&seed);
        Ok(Self { keypair })
    }
}

#[async_trait]
impl Ed25519Signer for CryptoxideEd25519Keypair {
    async fn try_sign(&self, msg: &[u8]) -> Result<[u8; 64], String> {
        let signature = ed25519::signature(msg, &self.keypair);
        Ok(signature)
    }
}

impl Keypair for CryptoxideEd25519Keypair {
    type VerifyingKey = [u8; 32];

    fn verifying_key(&self) -> Self::VerifyingKey {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(&self.keypair[32..]);
        pk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ed25519_keypair_generation_and_signing() {
        let seed = [1u8; 32];
        let keypair =
            CryptoxideEd25519Keypair::try_generate(Some(seed)).expect("Failed to generate keypair");
        let message = b"Hello, Algorand!";
        let signature = keypair
            .try_sign(message)
            .await
            .expect("Failed to sign message");

        // Verify the signature using the verifying key
        let verifying_key = keypair.verifying_key();
        let is_valid = ed25519::verify(message, &verifying_key, &signature);
        assert!(is_valid, "Signature verification failed");
    }

    #[tokio::test]
    async fn test_ed25519_random_generation() {
        let keypair =
            CryptoxideEd25519Keypair::try_generate(None).expect("Failed to generate keypair");
        let message = b"Test message";
        let signature = keypair
            .try_sign(message)
            .await
            .expect("Failed to sign message");

        let verifying_key = keypair.verifying_key();
        let is_valid = ed25519::verify(message, &verifying_key, &signature);
        assert!(
            is_valid,
            "Signature verification failed for randomly generated key"
        );
    }
}
