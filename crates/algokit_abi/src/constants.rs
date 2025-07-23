use regex::Regex;
use std::sync::LazyLock;

pub const HASH_BYTES_LENGTH: usize = 32;
pub const LENGTH_ENCODE_BYTE_SIZE: usize = 2;
pub const ALGORAND_PUBLIC_KEY_BYTE_LENGTH: usize = 32;
pub const ALGORAND_CHECKSUM_BYTE_LENGTH: usize = 4;
// Boolean encoding
pub const BOOL_TRUE_BYTE: u8 = 0x80;
pub const BOOL_FALSE_BYTE: u8 = 0x00;

// Bit manipulation
pub const BITS_PER_BYTE: u8 = 8;

pub const ALGORAND_ADDRESS_LENGTH: usize = 58;

// ABI type parsing constants
pub const MAX_BIT_SIZE: u16 = 512;
pub const MAX_PRECISION: u8 = 160;

// Regex patterns for ABI type parsing
pub static STATIC_ARRAY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([a-z\d\[\](),]+)\[(0|[1-9][\d]*)]$").expect("Invalid static array regex")
});

pub static UFIXED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^ufixed([1-9][\d]*)x([1-9][\d]*)$").expect("Invalid ufixed regex")
});
