pub const HASH_BYTES_LENGTH: usize = 32;
pub const ALGORAND_CHECKSUM_BYTE_LENGTH: usize = 4;
pub const ALGORAND_ADDRESS_LENGTH: usize = 58;
pub const ALGORAND_PUBLIC_KEY_BYTE_LENGTH: usize = 32;
pub const ALGORAND_SECRET_KEY_BYTE_LENGTH: usize = 32;
pub const ALGORAND_SIGNATURE_BYTE_LENGTH: usize = 64;
pub const ALGORAND_SIGNATURE_ENCODING_INCR: usize = 75;
pub type Byte32 = [u8; 32];
pub const MAX_TX_GROUP_SIZE: usize = 16;

pub const MULTISIG_DOMAIN_SEPARATOR: &str = "MultisigAddr";
pub const EMPTY_SIGNATURE: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH] =
    [0; ALGORAND_SIGNATURE_BYTE_LENGTH];

// Application program size constraints
pub const MAX_EXTRA_PROGRAM_PAGES: u32 = 3;
pub const PROGRAM_PAGE_SIZE: usize = 2048; // In bytes

// Application reference limits
pub const MAX_APP_ARGS: usize = 16;
pub const MAX_ARGS_SIZE: usize = 2048; // Maximum size in bytes of all args combined
pub const MAX_OVERALL_REFERENCES: usize = 8;
pub const MAX_ACCOUNT_REFERENCES: usize = 4;
pub const MAX_APP_REFERENCES: usize = 8;
pub const MAX_ASSET_REFERENCES: usize = 8;
pub const MAX_BOX_REFERENCES: usize = 8;

// Application state schema limits
pub const MAX_GLOBAL_STATE_KEYS: u32 = 64;
pub const MAX_LOCAL_STATE_KEYS: u32 = 16;
