//! BIP32-Ed25519 hierarchical deterministic key derivation and raw signing.
//!
//! This module provides stateless wrappers over the `ed25519-bip32` crate
//! (sourced from `xHD-Wallet-API-rs`) for deriving extended keys along the
//! BIP44 path `m/44'/<coin>'/<account>'/0/<key_index>` under the Peikert
//! derivation scheme, and for signing arbitrary bytes with an already-derived
//! 96-byte extended private key.
//!
//! All operations are stateless: the caller supplies seed or key material and
//! receives byte arrays in return. Wallet-level concerns such as secret
//! storage, zeroization, and BIP44 path bookkeeping are the responsibility of
//! the caller.

use ed25519_bip32::{
    DerivationScheme, Signature, XPrv,
    api::{KeyContext as UpstreamKeyContext, key_gen as upstream_key_gen},
};
use snafu::Snafu;

pub const HD_SEED_SIZE: usize = 64;
pub const XPRV_SIZE: usize = 96;
pub const PUBLIC_KEY_SIZE: usize = 32;
pub const SIGNATURE_SIZE: usize = 64;

/// Represents errors that can occur during HD wallet derivation or signing.
#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum XhdError {
    #[snafu(display("Seed must be {expected} bytes (got {found})"))]
    InvalidSeedLength { expected: usize, found: usize },

    #[snafu(display("Extended private key must be {expected} bytes (got {found})"))]
    InvalidXprvLength { expected: usize, found: usize },

    #[snafu(display("Extended private key bytes failed verification"))]
    InvalidXprv,
}

/// Selects the BIP44 `coin_type` slot used when deriving a key.
///
/// [`KeyContext::Address`] selects coin type 283 for Algorand spending and
/// transaction keys. [`KeyContext::Identity`] selects coin type 0 for Algorand
/// identity and authentication keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyContext {
    Address,
    Identity,
}

impl From<KeyContext> for UpstreamKeyContext {
    fn from(value: KeyContext) -> Self {
        match value {
            KeyContext::Address => UpstreamKeyContext::Address,
            KeyContext::Identity => UpstreamKeyContext::Identity,
        }
    }
}

/// Represents the result of deriving an account at a BIP44 leaf.
///
/// Contains the 96-byte extended private key (32-byte scalar, 32-byte prefix,
/// and 32-byte chain code) and the corresponding 32-byte ed25519 public key.
#[derive(Debug, Clone)]
pub struct DerivedAccount {
    /// Extended private key: 32-byte scalar, 32-byte prefix, 32-byte chain code.
    pub xprv: [u8; XPRV_SIZE],
    /// Ed25519 public key derived from the extended private key.
    pub public_key: [u8; PUBLIC_KEY_SIZE],
}

/// Derives a 96-byte root extended private key from a 64-byte seed.
///
/// The caller owns the returned bytes and is responsible for storage and zeroization.
///
/// # Errors
///
/// Returns [`XhdError::InvalidSeedLength`] if `seed` is not exactly [`HD_SEED_SIZE`] bytes.
pub fn root_key_from_seed(seed: &[u8]) -> Result<[u8; XPRV_SIZE], XhdError> {
    if seed.len() != HD_SEED_SIZE {
        return Err(XhdError::InvalidSeedLength {
            expected: HD_SEED_SIZE,
            found: seed.len(),
        });
    }

    let seed_array: [u8; HD_SEED_SIZE] =
        seed.try_into().map_err(|_| XhdError::InvalidSeedLength {
            expected: HD_SEED_SIZE,
            found: seed.len(),
        })?;

    let xprv = XPrv::from_seed(&seed_array);
    Ok(xprv.into())
}

/// Derives an extended private key at the BIP44 path
/// `m/44'/<coin>'/<account>'/0/<key_index>` using the Peikert derivation scheme.
///
/// The `coin` segment is determined by `key_context`: 283 for [`KeyContext::Address`]
/// and 0 for [`KeyContext::Identity`].
///
/// # Errors
///
/// Returns [`XhdError::InvalidXprvLength`] if `root_key` is not exactly [`XPRV_SIZE`]
/// bytes, or [`XhdError::InvalidXprv`] if the bytes do not form a valid extended
/// private key.
pub fn derive(
    root_key: &[u8],
    key_context: KeyContext,
    account: u32,
    key_index: u32,
) -> Result<DerivedAccount, XhdError> {
    let root_xprv = parse_xprv(root_key)?;

    let derived = upstream_key_gen(
        &root_xprv,
        key_context.into(),
        account,
        key_index,
        DerivationScheme::Peikert,
    )
    .map_err(|_| XhdError::InvalidXprv)?;

    let public_key = derived.public().public_key();
    let xprv: [u8; XPRV_SIZE] = derived.into();

    Ok(DerivedAccount { xprv, public_key })
}

/// Signs `msg` with an already-derived 96-byte extended private key.
///
/// Unlike standard ed25519 signing, this function does not perform the
/// seed-expansion step on `extended_key`: BIP32-Ed25519 keys are already in
/// expanded form, and re-hashing them would corrupt the signing material.
///
/// # Errors
///
/// Returns [`XhdError::InvalidXprvLength`] if `extended_key` is not exactly
/// [`XPRV_SIZE`] bytes, or [`XhdError::InvalidXprv`] if the bytes do not form
/// a valid extended private key.
pub fn raw_sign(extended_key: &[u8], msg: &[u8]) -> Result<[u8; SIGNATURE_SIZE], XhdError> {
    let xprv = parse_xprv(extended_key)?;
    let signature: Signature<Vec<u8>> = xprv.sign(msg);

    let mut out = [0u8; SIGNATURE_SIZE];
    out.copy_from_slice(signature.as_ref());
    Ok(out)
}

fn parse_xprv(bytes: &[u8]) -> Result<XPrv, XhdError> {
    if bytes.len() != XPRV_SIZE {
        return Err(XhdError::InvalidXprvLength {
            expected: XPRV_SIZE,
            found: bytes.len(),
        });
    }
    XPrv::from_slice_verified(bytes).map_err(|_| XhdError::InvalidXprv)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Vectors copied from xHD-Wallet-API-rs/src/api.rs.
    const SEED_HEX: &str = "3aff2db416b895ec3cf9a4f8d1e970bc9819920e7bf44a5e350477af0ef557b1511b0986debf78dd38c7c520cd44ff7c7231618f958e21ef0250733a8c1915ea";
    const ROOT_KEY_HEX: &str = "a8ba80028922d9fcfa055c78aede55b5c575bcd8d5a53168edf45f36d9ec8f4694592b4bc892907583e22669ecdf1b0409a9f3bd5549f2dd751b51360909cd05796b9206ec30e142e94b790a98805bf999042b55046963174ee6cee2d0375946";

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn root_key_from_known_seed() {
        let seed = hex_to_bytes(SEED_HEX);
        let root_key = root_key_from_seed(&seed).expect("valid seed");
        let expected = hex_to_bytes(ROOT_KEY_HEX);
        assert_eq!(root_key.as_slice(), expected.as_slice());
    }

    #[test]
    fn rejects_wrong_seed_length() {
        let err = root_key_from_seed(&[0u8; 32]).expect_err("should reject 32B seed");
        assert_eq!(
            err,
            XhdError::InvalidSeedLength {
                expected: 64,
                found: 32
            }
        );
    }

    #[test]
    fn address_derivation_vectors() {
        let root_key = hex_to_bytes(ROOT_KEY_HEX);
        let cases = [
            (
                0u32,
                0u32,
                "7bda7ac12627b2c259f1df6875d30c10b35f55b33ad2cc8ea2736eaa3ebcfab9",
            ),
            (
                0,
                1,
                "5bae8828f111064637ac5061bd63bc4fcfe4a833252305f25eeab9c64ecdf519",
            ),
            (
                0,
                2,
                "00a72635e97cba966529e9bfb4baf4a32d7b8cd2fcd8e2476ce5be1177848cb3",
            ),
            (
                1,
                0,
                "358d8c4382992849a764438e02b1c45c2ca4e86bbcfe10fd5b963f3610012bc9",
            ),
            (
                2,
                1,
                "1f0f75fbbca12b22523973191061b2f96522740e139a3420c730717ac5b0dfc0",
            ),
            (
                3,
                0,
                "f035316f915b342ea5fe78dccb59d907b93805732219d436a1bd8488ff4e5b1b",
            ),
        ];

        for (account, key_index, expected_hex) in cases {
            let derived = derive(&root_key, KeyContext::Address, account, key_index)
                .expect("derivation succeeds");
            let expected = hex_to_bytes(expected_hex);
            assert_eq!(
                derived.public_key.as_slice(),
                expected.as_slice(),
                "Address account={} key_index={}",
                account,
                key_index
            );
            assert_eq!(derived.xprv.len(), XPRV_SIZE);
        }
    }

    #[test]
    fn identity_derivation_vectors() {
        let root_key = hex_to_bytes(ROOT_KEY_HEX);
        let cases = [
            (
                0u32,
                0u32,
                "ff8b1863ef5e40d0a48c245f26a6dbdf5da94dc75a1851f51d8a04e547bd5f5a",
            ),
            (
                0,
                1,
                "2b46c2af0890493e486049d456509a0199e565b41a5fb622f0ea4b9337bd2b97",
            ),
            (
                0,
                2,
                "2713f135f19ef3dcfca73cb536b1e077b1165cd0b7bedbef709447319ff0016d",
            ),
            (
                1,
                0,
                "232847ae1bb95babcaa50c8033fab98f59e4b4ad1d89ac523a90c830e4ceee4a",
            ),
            (
                2,
                1,
                "8f68b6572860d84e8a41e38db1c8c692ded5eb291846f2e5bbfde774a9c6d16e",
            ),
        ];

        for (account, key_index, expected_hex) in cases {
            let derived = derive(&root_key, KeyContext::Identity, account, key_index)
                .expect("derivation succeeds");
            let expected = hex_to_bytes(expected_hex);
            assert_eq!(
                derived.public_key.as_slice(),
                expected.as_slice(),
                "Identity account={} key_index={}",
                account,
                key_index
            );
        }
    }

    #[test]
    fn raw_sign_round_trip() {
        let root_key = hex_to_bytes(ROOT_KEY_HEX);
        let derived = derive(&root_key, KeyContext::Address, 0, 0).expect("derivation succeeds");

        let message = b"Hello, Algorand!";
        let signature = raw_sign(&derived.xprv, message).expect("sign succeeds");
        assert_eq!(signature.len(), SIGNATURE_SIZE);

        let upstream_xprv = XPrv::from_slice_verified(&derived.xprv).expect("valid xprv");
        let xpub = upstream_xprv.public();
        let sig_for_verify = Signature::<u8>::from_slice(&signature).expect("64B signature");
        assert!(
            xpub.verify(message, &sig_for_verify),
            "signature should verify"
        );
    }

    #[test]
    fn raw_sign_rejects_wrong_xprv_length() {
        let err = raw_sign(&[0u8; 64], b"msg").expect_err("64B is not a valid xprv");
        assert_eq!(
            err,
            XhdError::InvalidXprvLength {
                expected: 96,
                found: 64
            }
        );
    }
}
