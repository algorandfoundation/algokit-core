use crate::abi_type::ABIType;
use crate::abi_value::ABIValue;
use crate::error::ABIError;
use sha2::{Digest, Sha512_256};
use std::fmt::Display;
use std::str::FromStr;

/// Constant for void return type in method signatures.
const VOID_RETURN_TYPE: &str = "void";

/// Represents a transaction type that can be used as an ABI method argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABITransactionType {
    /// Any transaction type
    Txn,
    /// Payment (algo transfer)
    Payment,
    /// Key registration (configure consensus participation)
    KeyRegistration,
    /// Asset configuration (create, configure, or destroy ASAs)
    AssetConfig,
    /// Asset transfer (ASA transfer)
    AssetTransfer,
    /// Asset freeze (freeze or unfreeze ASAs)
    AssetFreeze,
    /// Application call (create, update, delete and call an application)
    ApplicationCall,
}

impl FromStr for ABITransactionType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "txn" => Ok(ABITransactionType::Txn),
            "pay" => Ok(ABITransactionType::Payment),
            "keyreg" => Ok(ABITransactionType::KeyRegistration),
            "acfg" => Ok(ABITransactionType::AssetConfig),
            "axfer" => Ok(ABITransactionType::AssetTransfer),
            "afrz" => Ok(ABITransactionType::AssetFreeze),
            "appl" => Ok(ABITransactionType::ApplicationCall),
            _ => Err(ABIError::ValidationError(format!(
                "Invalid transaction type: {}",
                s
            ))),
        }
    }
}

impl Display for ABITransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ABITransactionType::Txn => "txn",
            ABITransactionType::Payment => "pay",
            ABITransactionType::KeyRegistration => "keyreg",
            ABITransactionType::AssetConfig => "acfg",
            ABITransactionType::AssetTransfer => "axfer",
            ABITransactionType::AssetFreeze => "afrz",
            ABITransactionType::ApplicationCall => "appl",
        };
        write!(f, "{}", s)
    }
}

/// Represents a reference type that can be used as an ABI method argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABIReferenceType {
    /// Reference to an account in the Accounts reference array
    Account,
    /// Reference to an application in the Applications reference array
    Application,
    /// Reference to an asset in the Assets reference array
    Asset,
}

/// Represents a reference value that can be used as an ABI method argument.
#[derive(Debug, Clone)]
pub enum ABIReferenceValue {
    /// The address to an Algorand account.
    Account(String),
    /// An Algorand asset ID.
    Asset(u64),
    /// An Algorand application ID.
    Application(u64),
}

impl FromStr for ABIReferenceType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account" => Ok(ABIReferenceType::Account),
            "application" => Ok(ABIReferenceType::Application),
            "asset" => Ok(ABIReferenceType::Asset),
            _ => Err(ABIError::ValidationError(format!(
                "Invalid reference type: {}",
                s
            ))),
        }
    }
}

impl Display for ABIReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ABIReferenceType::Account => "account",
            ABIReferenceType::Application => "application",
            ABIReferenceType::Asset => "asset",
        };
        write!(f, "{}", s)
    }
}

/// Represents the category of an ABI method argument, which can be a value, a transaction, or a reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ABIMethodArgType {
    /// A value that is directly encoded in the application arguments.
    Value(ABIType),
    /// A transaction that is placed immediately before the application call in the transaction group.
    Transaction(ABITransactionType),
    /// A reference to an account, asset, or application that is encoded as an index into a reference array.
    Reference(ABIReferenceType),
}

impl ABIMethodArgType {
    /// Check if this is a transaction argument.
    pub(crate) fn is_transaction(&self) -> bool {
        matches!(self, ABIMethodArgType::Transaction(_))
    }

    /// Check if this is a reference argument.
    pub(crate) fn is_reference(&self) -> bool {
        matches!(self, ABIMethodArgType::Reference(_))
    }

    /// Check if this is a value type argument (directly encoded in ApplicationArgs).
    pub(crate) fn is_value_type(&self) -> bool {
        matches!(self, ABIMethodArgType::Value(_))
    }
}

impl FromStr for ABIMethodArgType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check for direct transaction types first (e.g., "pay", "keyreg", etc.)
        if let Ok(tx_type) = ABITransactionType::from_str(s) {
            return Ok(ABIMethodArgType::Transaction(tx_type));
        }

        // Check for reference types
        if let Ok(ref_type) = ABIReferenceType::from_str(s) {
            return Ok(ABIMethodArgType::Reference(ref_type));
        }

        // Default to ABI value type
        let abi_type = ABIType::from_str(s)?;
        Ok(ABIMethodArgType::Value(abi_type))
    }
}

/// Represents a parsed ABI method, including its name, arguments, and return type.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ABIMethod {
    /// The name of the method.
    pub name: String,
    /// A list of the method's arguments.
    pub args: Vec<ABIMethodArg>,
    /// The return type of the method, or `None` if the method does not return a value.
    pub returns: Option<ABIType>,
    /// An optional description of the method.
    pub description: Option<String>,
}

/// Represents an argument in an ABI method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABIMethodArg {
    /// The type of the argument.
    pub arg_type: ABIMethodArgType,
    /// An optional name for the argument.
    pub name: Option<String>,
    /// An optional description of the argument.
    pub description: Option<String>,
}

impl ABIMethod {
    /// Creates a new ABI method.
    pub fn new(
        name: String,
        args: Vec<ABIMethodArg>,
        returns: Option<ABIType>,
        description: Option<String>,
    ) -> Self {
        Self {
            name,
            args,
            returns,
            description,
        }
    }

    /// Returns the number of transaction arguments in the method.
    pub fn transaction_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.arg_type.is_transaction())
            .count()
    }

    /// Returns the number of reference arguments in the method.
    pub fn reference_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.arg_type.is_reference())
            .count()
    }

    /// Returns the number of value-type arguments in the method.
    pub fn value_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.arg_type.is_value_type())
            .count()
    }

    /// Returns the method selector, which is the first 4 bytes of the SHA-512/256 hash of the method signature.
    pub fn selector(&self) -> Result<Vec<u8>, ABIError> {
        let signature = self.signature()?;
        if signature.chars().any(|c| c.is_whitespace()) {
            return Err(ABIError::ValidationError(
                "Method signature cannot contain whitespace".to_string(),
            ));
        }

        let mut hasher = Sha512_256::new();
        hasher.update(signature.as_bytes());
        let hash = hasher.finalize();

        Ok(hash[..4].to_vec())
    }

    /// Returns the method signature as a string.
    pub fn signature(&self) -> Result<String, ABIError> {
        if self.name.is_empty() {
            return Err(ABIError::ValidationError(
                "Method name cannot be empty".to_string(),
            ));
        }

        let arg_types: Vec<String> = self
            .args
            .iter()
            .map(|arg| match &arg.arg_type {
                ABIMethodArgType::Value(abi_type) => abi_type.to_string(),
                ABIMethodArgType::Transaction(tx_type) => tx_type.to_string(),
                ABIMethodArgType::Reference(ref_type) => ref_type.to_string(),
            })
            .collect();

        // Validate each argument type
        for arg_type in &arg_types {
            ABIMethodArgType::from_str(arg_type)?;
        }

        let return_type = self
            .returns
            .as_ref()
            .map(|r| r.to_string())
            .unwrap_or_else(|| VOID_RETURN_TYPE.to_string());

        let args_str = arg_types.join(",");
        let signature = format!("{}({}){}", self.name, args_str, return_type);

        if signature.chars().any(|c| c.is_whitespace()) {
            return Err(ABIError::ValidationError(
                "Generated signature contains whitespace".to_string(),
            ));
        }

        Ok(signature)
    }
}

impl FromStr for ABIMethod {
    type Err = ABIError;

    fn from_str(signature: &str) -> Result<Self, Self::Err> {
        if signature.chars().any(|c| c.is_whitespace()) {
            return Err(ABIError::ValidationError(
                "Method signature cannot contain whitespace".to_string(),
            ));
        }

        let open_paren_pos = signature.find('(').ok_or_else(|| {
            ABIError::ValidationError(
                "Method signature must contain opening parenthesis".to_string(),
            )
        })?;

        if open_paren_pos == 0 {
            return Err(ABIError::ValidationError(
                "Method name cannot be empty".to_string(),
            ));
        }
        let method_name = signature[..open_paren_pos].to_string();

        let close_paren_pos = find_matching_closing_paren(signature, open_paren_pos)?;

        let args_str = &signature[open_paren_pos + 1..close_paren_pos];

        let arguments = if args_str.is_empty() {
            Vec::new()
        } else {
            split_arguments_by_comma(args_str)?
        };

        let return_type = if close_paren_pos + 1 < signature.len() {
            signature[close_paren_pos + 1..].to_string()
        } else {
            VOID_RETURN_TYPE.to_string()
        };

        // Parse each argument
        let mut args = Vec::new();
        for (i, arg_type) in arguments.iter().enumerate() {
            let _type = ABIMethodArgType::from_str(arg_type)?;
            let arg_name = Some(format!("arg{}", i));
            let arg = ABIMethodArg::new(_type, arg_name, None);
            args.push(arg);
        }

        // Parse return type
        let returns = if return_type != VOID_RETURN_TYPE {
            let abi_return_type = ABIType::from_str(&return_type)?;
            Some(abi_return_type)
        } else {
            None
        };

        let parsed_method = ABIMethod::new(method_name, args, returns, None);

        Ok(parsed_method)
    }
}

impl ABIMethodArg {
    /// Creates a new ABI method argument.
    pub fn new(
        arg_type: ABIMethodArgType,
        name: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            arg_type,
            name,
            description,
        }
    }
}

/// Represents an ABI method return value with parsed data.
#[derive(Debug, Clone)]
pub struct ABIReturn {
    /// The method that was called.
    pub method: ABIMethod,
    /// The raw return value as bytes.
    pub raw_return_value: Vec<u8>,
    /// The parsed ABI return value.
    pub return_value: ABIValue,
}

/// Find the matching closing parenthesis for an opening parenthesis.
fn find_matching_closing_paren(s: &str, open_pos: usize) -> Result<usize, ABIError> {
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;

    for (i, &ch) in chars.iter().enumerate().skip(open_pos) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
    }

    Err(ABIError::ValidationError(
        "Mismatched parentheses in method signature".to_string(),
    ))
}

/// Split arguments by comma, respecting nested parentheses.
/// This is a specialized version of the tuple parsing logic for method arguments.
fn split_arguments_by_comma(args_str: &str) -> Result<Vec<String>, ABIError> {
    use crate::abi_type::parse_tuple_content;

    if args_str.is_empty() {
        return Ok(Vec::new());
    }

    // Use the shared tuple parsing logic, but with method-specific validation
    let arguments = parse_tuple_content(args_str)?;

    // Additional validation for method arguments: no empty arguments
    for arg in &arguments {
        if arg.trim().is_empty() {
            return Err(ABIError::ValidationError(
                "Empty argument in method signature".to_string(),
            ));
        }
    }

    Ok(arguments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi_type::parse_tuple_content;
    use hex;
    use rstest::rstest;

    // Transaction type parsing with round-trip validation
    #[rstest]
    #[case("txn", ABITransactionType::Txn)]
    #[case("pay", ABITransactionType::Payment)]
    #[case("keyreg", ABITransactionType::KeyRegistration)]
    #[case("acfg", ABITransactionType::AssetConfig)]
    #[case("axfer", ABITransactionType::AssetTransfer)]
    #[case("afrz", ABITransactionType::AssetFreeze)]
    #[case("appl", ABITransactionType::ApplicationCall)]
    fn transaction_type_from_str(#[case] input: &str, #[case] expected: ABITransactionType) {
        assert_eq!(ABITransactionType::from_str(input).unwrap(), expected);
        assert_eq!(expected.to_string(), input);
    }

    #[test]
    fn transaction_type_from_str_invalid() {
        assert!(ABITransactionType::from_str("invalid").is_err());
    }

    // Reference type parsing with round-trip validation
    #[rstest]
    #[case("account", ABIReferenceType::Account)]
    #[case("application", ABIReferenceType::Application)]
    #[case("asset", ABIReferenceType::Asset)]
    fn reference_type_from_str(#[case] input: &str, #[case] expected: ABIReferenceType) {
        assert_eq!(ABIReferenceType::from_str(input).unwrap(), expected);
        assert_eq!(expected.to_string(), input);
    }

    #[test]
    fn reference_type_from_str_invalid() {
        assert!(ABIReferenceType::from_str("invalid").is_err());
    }

    // Method argument type parsing - consolidated test
    #[rstest]
    #[case("pay", ABIMethodArgType::Transaction(ABITransactionType::Payment))]
    #[case("account", ABIMethodArgType::Reference(ABIReferenceType::Account))]
    fn method_arg_type_from_str_special(#[case] input: &str, #[case] expected: ABIMethodArgType) {
        assert_eq!(ABIMethodArgType::from_str(input).unwrap(), expected);
    }

    #[rstest]
    #[case("uint64")]
    #[case("(uint64,string)")]
    #[case("uint64[]")]
    fn method_arg_type_from_str_value(#[case] input: &str) {
        match ABIMethodArgType::from_str(input).unwrap() {
            ABIMethodArgType::Value(abi_type) => assert_eq!(abi_type.to_string(), input),
            _ => panic!("Expected Value type for: {}", input),
        }
    }

    #[test]
    fn method_arg_type_from_str_invalid() {
        assert!(ABIMethodArgType::from_str("invalid_type").is_err());
    }

    // Method parsing - essential cases only
    #[rstest]
    #[case("add(uint64,uint64)uint64", "add", Some("uint64"), 2)]
    #[case("getName()string", "getName", Some("string"), 0)]
    #[case("doSomething(uint64)", "doSomething", None, 1)]
    #[case("transfer(address,uint64,pay)bool", "transfer", Some("bool"), 3)]
    fn method_from_str_valid(
        #[case] signature: &str,
        #[case] expected_name: &str,
        #[case] expected_return: Option<&str>,
        #[case] expected_arg_count: usize,
    ) {
        let method = ABIMethod::from_str(signature).unwrap();
        assert_eq!(method.name, expected_name);
        assert_eq!(method.args.len(), expected_arg_count);

        if let Some(return_str) = expected_return {
            let expected_abi_type = ABIType::from_str(return_str).unwrap();
            assert_eq!(method.returns, Some(expected_abi_type));
        } else {
            assert_eq!(method.returns, None);
        }
    }

    #[rstest]
    #[case("add(uint64, uint64)uint64")] // whitespace
    #[case("(uint64)uint64")] // empty name
    #[case("method")] // no parenthesis
    fn method_from_str_invalid(#[case] signature: &str) {
        assert!(ABIMethod::from_str(signature).is_err());
    }

    // Method selector verification - critical for hash correctness
    #[rstest]
    #[case("add(uint64,uint64)uint64", "fe6bdf69")]
    #[case("optIn()void", "29314d95")]
    #[case("deposit(pay,uint64)void", "f2355b55")]
    #[case("bootstrap(pay,pay,application)void", "895c2a3b")]
    fn method_selector(#[case] signature: &str, #[case] expected_hex: &str) {
        let method = ABIMethod::from_str(signature).unwrap();
        let selector = method.selector().unwrap();
        assert_eq!(hex::encode(&selector), expected_hex);
        assert_eq!(selector.len(), 4);
    }

    // ARC-4 tuple parsing - essential cases
    #[rstest]
    #[case("uint64,string,bool", vec!["uint64", "string", "bool"])]
    #[case("(uint64,string),bool", vec!["(uint64,string)", "bool"])]
    #[case("", vec![])]
    fn parse_tuple_content_valid(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = parse_tuple_content(input).unwrap();
        let expected_strings: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        assert_eq!(result, expected_strings);
    }

    #[rstest]
    #[case(",uint64")] // leading comma
    #[case("uint64,")] // trailing comma
    #[case("uint64,,string")] // double comma
    fn parse_tuple_content_invalid(#[case] input: &str) {
        assert!(parse_tuple_content(input).is_err());
    }

    // Signature round-trip
    #[rstest]
    #[case("add(uint64,uint64)uint64")]
    #[case("optIn()void")]
    fn signature_round_trip(#[case] signature: &str) {
        let method = ABIMethod::from_str(signature).unwrap();
        assert_eq!(method.signature().unwrap(), signature);
    }

    // Method argument type predicates
    #[test]
    fn method_arg_type_predicates() {
        let tx_arg = ABIMethodArgType::Transaction(ABITransactionType::Payment);
        let ref_arg = ABIMethodArgType::Reference(ABIReferenceType::Account);
        let val_arg = ABIMethodArgType::Value(ABIType::from_str("uint64").unwrap());

        assert!(tx_arg.is_transaction() && !tx_arg.is_reference() && !tx_arg.is_value_type());
        assert!(!ref_arg.is_transaction() && ref_arg.is_reference() && !ref_arg.is_value_type());
        assert!(!val_arg.is_transaction() && !val_arg.is_reference() && val_arg.is_value_type());
    }

    // Edge cases
    #[test]
    fn empty_method_name_error() {
        let method = ABIMethod::new("".to_string(), vec![], None, None);
        assert!(method.signature().is_err());
    }

    #[test]
    fn selector_length() {
        let method = ABIMethod::new("test".to_string(), vec![], None, None);
        assert_eq!(method.selector().unwrap().len(), 4);
    }
}
