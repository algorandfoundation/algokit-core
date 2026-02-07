use cryptoxide::ed25519;
use signature::Keypair;
use signature::Signer;

/// Trait that combines Signer and Keypair for Ed25519 (64-byte signature, 32-byte public key).
/// This allows using both traits in a trait object since Rust only allows one non-auto trait per trait object.
pub trait Ed25519KeyAndSigner: Signer<[u8; 64]> + Keypair<VerifyingKey = [u8; 32]> {}

impl<T> Ed25519KeyAndSigner for T where T: Signer<[u8; 64]> + Keypair<VerifyingKey = [u8; 32]> {}

pub trait Ed25519Generator {
    type Error: std::error::Error;

    fn try_generate(seed: Option<[u8; 32]>) -> Result<impl Ed25519KeyAndSigner, Self::Error>;
}

pub struct CryptoixdeEd25519Keypair {
    keypair: [u8; 64],
}

impl Signer<[u8; 64]> for CryptoixdeEd25519Keypair {
    fn try_sign(&self, msg: &[u8]) -> Result<[u8; 64], signature::Error> {
        let signature = ed25519::signature(msg, &self.keypair);
        Ok(signature)
    }
}

impl Keypair for CryptoixdeEd25519Keypair {
    type VerifyingKey = [u8; 32];

    fn verifying_key(&self) -> Self::VerifyingKey {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(&self.keypair[32..]);
        pk
    }
}

impl Ed25519Generator for CryptoixdeEd25519Keypair {
    type Error = getrandom::Error;

    fn try_generate(seed: Option<[u8; 32]>) -> Result<impl Ed25519KeyAndSigner, Self::Error> {
        let seed = match seed {
            Some(s) => s,
            None => {
                let mut seed = [0u8; 32];
                getrandom::fill(&mut seed)?;
                seed
            }
        };
        let (keypair, _) = ed25519::keypair(&seed);
        Ok(CryptoixdeEd25519Keypair { keypair })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_keypair_generation_and_signing() {
        let seed = [1u8; 32];
        let keypair =
            CryptoixdeEd25519Keypair::try_generate(Some(seed)).expect("Failed to generate keypair");
        let message = b"Hello, Algorand!";
        let signature = keypair.try_sign(message).expect("Failed to sign message");

        // Verify the signature using the verifying key
        let verifying_key = keypair.verifying_key();
        let is_valid = ed25519::verify(message, &verifying_key, &signature);
        assert!(is_valid, "Signature verification failed");
    }
}
