use num_bigint::BigUint;

/// Represents a value that can be encoded or decoded as an ABI type.
#[derive(Debug, Clone, PartialEq)]
pub enum ABIValue {
    /// A boolean value.
    Bool(bool),
    /// An unsigned integer value.
    Uint(BigUint),
    /// A string value.
    String(String),
    /// A byte value.
    Byte(u8),
    /// An array of ABI values.
    Array(Vec<ABIValue>),
    /// An Algorand address.
    Address(String),
}
