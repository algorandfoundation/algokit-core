use crate::constants::{
    Byte32, ALGORAND_ADDRESS_LENGTH, ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
};
use crate::error::AlgoKitTransactError;
use crate::utils::pub_key_to_checksum;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(transparent)]
pub struct Address {
    #[serde_as(as = "Bytes")]
    pub pub_key: Byte32,
}

impl Address {
    pub fn from_pubkey(pub_key: &Byte32) -> Self {
        Address { pub_key: *pub_key }
    }

    pub fn from_string(address: &str) -> Result<Self, AlgoKitTransactError> {
        if address.len() != ALGORAND_ADDRESS_LENGTH {
            return Err(AlgoKitTransactError::InvalidAddress(
                "address length is not 58".to_string(),
            ));
        }
        let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, address)
            .expect("decoded value should exist");

        let pub_key: [u8; 32] = decoded[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "could not decode address into 32-byte public key".to_string(),
                )
            })?;

        let checksum: [u8; 4] = decoded[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "could not get 4-byte checksum from decoded address".to_string(),
                )
            })?;

        let computed_checksum = pub_key_to_checksum(&pub_key);

        if computed_checksum != checksum {
            return Err(AlgoKitTransactError::InvalidAddress(
                "checksum is invalid".to_string(),
            ));
        }

        Ok(Self { pub_key })
    }

    pub fn checksum(&self) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
        pub_key_to_checksum(&self.pub_key)
    }

    pub fn address(&self) -> String {
        let mut address_bytes = [0u8; 36]; // 32 bytes pub_key + 4 bytes checksum

        address_bytes[..32].copy_from_slice(&self.pub_key);

        let checksum = self.checksum();
        address_bytes[32..].copy_from_slice(&checksum);

        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &address_bytes)
    }
}
