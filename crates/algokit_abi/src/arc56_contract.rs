use crate::abi_method::{ABIDefaultValue, ABIMethod, ABIMethodArg, ABIMethodArgType};
use crate::abi_type::ABIType;
use crate::constants::VOID_RETURN_TYPE;
use crate::error::ABIError;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// An ABI-encoded type string
pub type ABITypeString = String;

/// The name of a defined struct
pub type StructName = String;

/// Raw byteslice without the length prefixed that is specified in ARC-4
pub const AVM_BYTES: &str = "AVMBytes";

/// A utf-8 string without the length prefix that is specified in ARC-4
pub const AVM_STRING: &str = "AVMString";

/// A 64-bit unsigned integer
pub const AVM_UINT64: &str = "AVMUint64";

/// Native AVM types
pub type AVMType = String;

/// Information about a single field in a struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    /// The name of the struct field
    pub name: String,
    /// The type of the struct field's value
    #[serde(rename = "type")]
    pub field_type: StructFieldType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StructFieldType {
    Value(String),
    Nested(Vec<StructField>),
}

/// Enum representing different call types for app transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CallOnApplicationComplete {
    ClearState,
    CloseOut,
    DeleteApplication,
    NoOp,
    OptIn,
    UpdateApplication,
}

/// Enum representing different create types for app transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CreateOnApplicationComplete {
    DeleteApplication,
    NoOp,
    OptIn,
}

/// Supported bare actions for the contract.
/// An action is a combination of call/create and an OnComplete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BareActions {
    /// OnCompletes this method allows when appID !== 0
    pub call: Vec<CallOnApplicationComplete>,
    /// OnCompletes this method allows when appID === 0
    pub create: Vec<CreateOnApplicationComplete>,
}

/// The compiled bytecode for the app.
/// MUST be omitted if included as part of ARC23.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteCode {
    pub approval: String,
    pub clear: String,
}

/// Enum representing different compiler types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compiler {
    Algod,
    Puya,
}

/// Represents compiler version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerVersion {
    #[serde(rename = "commitHash", skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<u32>,
}

/// Information used to get the given byteCode and/or PC values in sourceInfo.
/// MUST be given if byteCode or PC values are present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerInfo {
    pub compiler: Compiler,
    #[serde(rename = "compilerVersion")]
    pub compiler_version: CompilerVersion,
}

/// Network-specific app information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "appID")]
    pub app_id: u64,
}

/// The scratch variables used during runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchVariable {
    pub slot: u32,
    #[serde(rename = "type")]
    pub var_type: String,
}

/// The pre-compiled TEAL that may contain template variables.
/// MUST be omitted if included as part of ARC23.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub approval: String,
    pub clear: String,
}

impl Source {
    /// Get decoded approval program source
    pub fn get_decoded_approval(&self) -> Result<String, ABIError> {
        self.decode_source(&self.approval)
    }

    /// Get decoded clear program source
    pub fn get_decoded_clear(&self) -> Result<String, ABIError> {
        self.decode_source(&self.clear)
    }

    fn decode_source(&self, b64_text: &str) -> Result<String, ABIError> {
        let decoded =
            general_purpose::STANDARD
                .decode(b64_text)
                .map_err(|e| ABIError::ValidationError {
                    message: format!("Failed to decode base64: {}", e),
                })?;
        Ok(String::from_utf8_lossy(&decoded).to_string())
    }
}

/// State schema for global and local state allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSchema {
    pub bytes: u32,
    pub ints: u32,
}

/// Defines the values that should be used for state allocation when creating the app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "global")]
    pub global_state: StateSchema,
    #[serde(rename = "local")]
    pub local_state: StateSchema,
}

/// Template variables are variables in the TEAL that should be substituted prior to compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariables {
    #[serde(rename = "type")]
    pub var_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Event argument information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventArg {
    #[serde(rename = "type")]
    pub arg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "struct", skip_serializing_if = "Option::is_none")]
    pub struct_name: Option<String>,
}

/// Describes a single key in app storage with parsed ABI types.
#[derive(Debug, Clone)]
pub struct ABIStorageKey {
    pub key: String,
    pub key_type: ABIType,
    pub value_type: ABIType,
    pub desc: Option<String>,
}

/// Describes a storage map with parsed ABI types.
#[derive(Debug, Clone)]
pub struct ABIStorageMap {
    pub key_type: ABIType,
    pub value_type: ABIType,
    pub desc: Option<String>,
    pub prefix: Option<String>,
}

/// ARC-28 events are described using an extension of the original interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub args: Vec<EventArg>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// Method actions information
/// An action is a combination of call/create and an OnComplete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    /// OnCompletes this method allows when appID === 0
    pub create: Vec<CreateOnApplicationComplete>,
    /// OnCompletes this method allows when appID !== 0
    pub call: Vec<CallOnApplicationComplete>,
}

/// Source of default value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DefaultValueSource {
    Box,
    Global,
    Local,
    Literal,
    Method,
}

/// Default value information for method arguments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultValue {
    /// Base64 encoded bytes, base64 ARC4 encoded uint64, or UTF-8 method selector
    pub data: String,
    /// Where the default value is coming from
    /// - box: The data key signifies the box key to read the value from
    /// - global: The data key signifies the global state key to read the value from
    /// - local: The data key signifies the local state key to read the value from (for the sender)
    /// - literal: the value is a literal and should be passed directly as the argument
    pub source: DefaultValueSource,
    /// How the data is encoded. This is the encoding for the data provided here, not the arg type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodArg {
    /// The type of the argument. The `struct` field should also be checked to determine if this arg is a struct.
    #[serde(rename = "type")]
    pub arg_type: String,
    /// The default value that clients should use
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<DefaultValue>,
    /// Optional, user-friendly description for the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Optional, user-friendly name for the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If the type is a struct, the name of the struct
    #[serde(rename = "struct", skip_serializing_if = "Option::is_none")]
    pub struct_name: Option<String>,
}

/// Recommended box references to include
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boxes {
    /// The base64 encoded box key
    pub key: String,
    /// The number of bytes being read from the box
    #[serde(rename = "readBytes")]
    pub read_bytes: u32,
    /// The number of bytes being written to the box
    #[serde(rename = "writeBytes")]
    pub write_bytes: u32,
    /// The app ID for the box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<u64>,
}

/// Information that clients can use when calling the method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendations {
    /// Recommended foreign accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    /// Recommended foreign apps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<u64>>,
    /// Recommended foreign assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<u64>>,
    /// Recommended box references to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxes: Option<Boxes>,
    /// The number of inner transactions the caller should cover the fees for
    #[serde(
        rename = "innerTransactionCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub inner_transaction_count: Option<u32>,
}

/// Information about the method's return value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Returns {
    /// The type of the return value, or "void" to indicate no return value.
    /// The `struct` field should also be checked to determine if this return value is a struct.
    #[serde(rename = "type")]
    pub return_type: String,
    /// Optional, user-friendly description for the return value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// If the type is a struct, the name of the struct
    #[serde(rename = "struct", skip_serializing_if = "Option::is_none")]
    pub struct_name: Option<String>,
}

/// PC offset method types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PcOffsetMethod {
    Cblocks,
    None,
}

/// Source code location information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub pc: Vec<u32>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teal: Option<u32>,
}

/// Describes a single key in app storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageKey {
    pub key: String,
    #[serde(rename = "keyType")]
    pub key_type: String,
    #[serde(rename = "valueType")]
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// Describes a mapping of key-value pairs in storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMap {
    #[serde(rename = "keyType")]
    pub key_type: String,
    #[serde(rename = "valueType")]
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// Storage keys for different storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    #[serde(rename = "box")]
    pub box_keys: HashMap<String, StorageKey>,
    #[serde(rename = "global")]
    pub global_state: HashMap<String, StorageKey>,
    #[serde(rename = "local")]
    pub local_state: HashMap<String, StorageKey>,
}

/// Storage maps for different storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maps {
    #[serde(rename = "box")]
    pub box_maps: HashMap<String, StorageMap>,
    #[serde(rename = "global")]
    pub global_state: HashMap<String, StorageMap>,
    #[serde(rename = "local")]
    pub local_state: HashMap<String, StorageMap>,
}

/// App state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub keys: Keys,
    pub maps: Maps,
    pub schema: Schema,
}

/// The source information for the program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramSourceInfo {
    #[serde(rename = "pcOffsetMethod")]
    pub pc_offset_method: PcOffsetMethod,
    #[serde(rename = "sourceInfo")]
    pub source_info: Vec<SourceInfo>,
}

/// Information about the TEAL programs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfoModel {
    pub approval: ProgramSourceInfo,
    pub clear: ProgramSourceInfo,
}

/// Describes a method in the contract.
/// This interface is an extension of the interface described in ARC-4.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    /// An action is a combination of call/create and an OnComplete
    pub actions: Actions,
    /// The arguments of the method, in order
    pub args: Vec<MethodArg>,
    /// The name of the method
    pub name: String,
    /// Information about the method's return value
    pub returns: Returns,
    /// Optional, user-friendly description for the method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// ARC-28 events that MAY be emitted by this method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// If this method does not write anything to the ledger (ARC-22)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    /// Information that clients can use when calling the method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Recommendations>,
}

impl Method {
    pub fn signature(&self) -> Result<String, ABIError> {
        let args_str = self
            .args
            .iter()
            .map(|arg| arg.arg_type.as_str())
            .collect::<Vec<_>>()
            .join(",");

        let signature = format!("{}({}){}", self.name, args_str, self.returns.return_type);

        Ok(signature)
    }
}

/// ARC-56 app specification.
/// Describes the entire contract as an extension of the ARC-4 interface.
/// See https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0056.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc56Contract {
    pub arcs: Vec<u32>,
    #[serde(rename = "bareActions")]
    pub bare_actions: BareActions,
    pub methods: Vec<Method>,
    pub name: String,
    pub state: State,
    pub structs: HashMap<String, Vec<StructField>>,
    #[serde(rename = "byteCode", skip_serializing_if = "Option::is_none")]
    pub byte_code: Option<ByteCode>,
    #[serde(rename = "compilerInfo", skip_serializing_if = "Option::is_none")]
    pub compiler_info: Option<CompilerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, Network>>,
    #[serde(rename = "scratchVariables", skip_serializing_if = "Option::is_none")]
    pub scratch_variables: Option<HashMap<String, ScratchVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "sourceInfo", skip_serializing_if = "Option::is_none")]
    pub source_info: Option<SourceInfoModel>,
    #[serde(rename = "templateVariables", skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<HashMap<String, TemplateVariables>>,
}

impl Arc56Contract {
    /// Create Arc56Contract from JSON string
    pub fn from_json(json_str: &str) -> Result<Self, ABIError> {
        serde_json::from_str(json_str).map_err(|e| ABIError::ValidationError {
            message: format!("Failed to parse ARC-56 JSON: {}", e),
        })
    }

    /// Convert Arc56Contract to JSON string with optional indentation
    ///
    /// # Parameters
    /// * `indent` - Optional number of spaces for indentation. If None, produces compact JSON.
    pub fn to_json(&self, indent: Option<usize>) -> Result<String, ABIError> {
        match indent {
            None => {
                // Compact JSON
                serde_json::to_string(self).map_err(|e| ABIError::EncodingError {
                    message: format!("Failed to serialize ARC-56 to JSON: {}", e),
                })
            }
            Some(0) => {
                // Pretty JSON with default formatting
                serde_json::to_string_pretty(self).map_err(|e| ABIError::EncodingError {
                    message: format!("Failed to serialize ARC-56 to pretty JSON: {}", e),
                })
            }
            Some(indent_size) => {
                // Custom indentation
                let indent_bytes = vec![b' '; indent_size];
                let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
                let mut buf = Vec::new();
                let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
                self.serialize(&mut ser)
                    .map_err(|e| ABIError::EncodingError {
                        message: format!("Failed to serialize ARC-56 with indent: {}", e),
                    })?;
                String::from_utf8(buf).map_err(|e| ABIError::EncodingError {
                    message: format!("Failed to convert serialized JSON to string: {}", e),
                })
            }
        }
    }

    /// Get a method by name or signature
    pub fn get_method(&self, method_name_or_signature: &str) -> Result<&Method, ABIError> {
        if !method_name_or_signature.contains('(') {
            // Filter by method name
            let methods: Vec<&Method> = self
                .methods
                .iter()
                .filter(|m| m.name == method_name_or_signature)
                .collect();

            if methods.is_empty() {
                return Err(ABIError::ValidationError {
                    message: format!(
                        "Unable to find method {} in {} app",
                        method_name_or_signature, self.name
                    ),
                });
            }

            if methods.len() > 1 {
                let signatures: Result<Vec<String>, ABIError> =
                    methods.iter().map(|m| m.signature()).collect();
                let signatures = signatures?;
                return Err(ABIError::ValidationError {
                    message: format!(
                        "Received a call to method {} in contract {}, but this resolved to multiple methods; \
                     please pass in an ABI signature instead: {}",
                        method_name_or_signature,
                        self.name,
                        signatures.join(", ")
                    ),
                });
            }

            Ok(methods[0])
        } else {
            // Find by signature
            self.methods
                .iter()
                .find(|m| {
                    m.signature()
                        .is_ok_and(|sig| sig == method_name_or_signature)
                })
                .ok_or_else(|| ABIError::ValidationError {
                    message: format!(
                        "Unable to find method {} in {} app",
                        method_name_or_signature, self.name
                    ),
                })
        }
    }

    /// Build an ABIMethod from an ARC-56 Method
    fn to_abi_method(&self, method: &Method) -> Result<ABIMethod, ABIError> {
        // Resolve argument types
        let args: Result<Vec<ABIMethodArg>, ABIError> = method
            .args
            .iter()
            .map(|arg| {
                let arg_type = self.resolve_method_arg_type(arg)?;
                let default_value = self.resolve_default_value(&arg.default_value)?;

                Ok(ABIMethodArg::new(
                    arg_type,
                    arg.name.clone(),
                    arg.desc.clone(),
                    default_value,
                ))
            })
            .collect();

        // Resolve return type
        let returns = if method.returns.return_type == VOID_RETURN_TYPE {
            None
        } else if let Some(struct_name) = &method.returns.struct_name {
            Some(ABIType::from_struct(struct_name, &self.structs)?)
        } else {
            Some(ABIType::from_str(&method.returns.return_type)?)
        };

        Ok(ABIMethod::new(
            method.name.clone(),
            args?,
            returns,
            method.desc.clone(),
        ))
    }

    fn resolve_method_arg_type(&self, arg: &MethodArg) -> Result<ABIMethodArgType, ABIError> {
        if let Some(struct_name) = &arg.struct_name {
            let abi_type = ABIType::from_struct(struct_name, &self.structs)?;
            return Ok(ABIMethodArgType::Value(abi_type));
        }

        ABIMethodArgType::from_str(&arg.arg_type)
    }

    fn resolve_default_value(
        &self,
        default_value: &Option<DefaultValue>,
    ) -> Result<Option<ABIDefaultValue>, ABIError> {
        let resolved_default_value = if let Some(default_value) = default_value {
            let resolved_value_type = if let Some(ref value_type) = default_value.value_type {
                let abi_type = if self.structs.contains_key(value_type) {
                    ABIType::from_struct(value_type, &self.structs)
                } else {
                    ABIType::from_str(value_type)
                }?;
                Some(abi_type)
            } else {
                None
            };

            Some(ABIDefaultValue {
                data: default_value.data.clone(),
                source: default_value.source.clone(),
                value_type: resolved_value_type,
            })
        } else {
            None
        };

        Ok(resolved_default_value)
    }

    /// Get a method by name or signature and convert to ABIMethod
    pub fn find_abi_method(&self, method_name_or_signature: &str) -> Result<ABIMethod, ABIError> {
        let arc56_method = self.get_method(method_name_or_signature)?;
        self.to_abi_method(arc56_method)
    }

    /// Get decoded TEAL sources (approval, clear) from the optional `source` field
    pub fn decoded_teal(&self) -> Result<(String, String), ABIError> {
        let src = self.source.as_ref().ok_or(ABIError::ValidationError {
            message: "Missing source in ARC-56 contract".to_string(),
        })?;
        let approval = src.get_decoded_approval()?;
        let clear = src.get_decoded_clear()?;
        Ok((approval, clear))
    }

    pub fn get_global_abi_storage_key(&self, key_name: &str) -> Result<ABIStorageKey, ABIError> {
        let storage_key = self.state.keys.global_state.get(key_name).ok_or_else(|| {
            ABIError::ValidationError {
                message: format!(
                    "Global storage key '{}' not found in contract '{}'",
                    key_name, self.name
                ),
            }
        })?;
        self.convert_storage_key(storage_key)
    }

    pub fn get_local_abi_storage_key(&self, key_name: &str) -> Result<ABIStorageKey, ABIError> {
        let storage_key =
            self.state
                .keys
                .local_state
                .get(key_name)
                .ok_or_else(|| ABIError::ValidationError {
                    message: format!(
                        "Local storage key '{}' not found in contract '{}'",
                        key_name, self.name
                    ),
                })?;
        self.convert_storage_key(storage_key)
    }

    pub fn get_global_abi_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, ABIError> {
        self.state
            .keys
            .global_state
            .iter()
            .map(|(name, storage_key)| {
                let abi_storage_key = self.convert_storage_key(storage_key)?;
                Ok((name.clone(), abi_storage_key))
            })
            .collect()
    }

    pub fn get_local_abi_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, ABIError> {
        self.state
            .keys
            .local_state
            .iter()
            .map(|(name, storage_key)| {
                let abi_storage_key = self.convert_storage_key(storage_key)?;
                Ok((name.clone(), abi_storage_key))
            })
            .collect()
    }

    pub fn get_box_abi_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, ABIError> {
        self.state
            .keys
            .box_keys
            .iter()
            .map(|(name, storage_key)| {
                let abi_storage_key = self.convert_storage_key(storage_key)?;
                Ok((name.clone(), abi_storage_key))
            })
            .collect()
    }

    fn convert_storage_key(&self, storage_key: &StorageKey) -> Result<ABIStorageKey, ABIError> {
        let key_type = self.resolve_storage_type(&storage_key.key_type)?;
        let value_type = self.resolve_storage_type(&storage_key.value_type)?;

        Ok(ABIStorageKey {
            key: storage_key.key.clone(),
            key_type,
            value_type,
            desc: storage_key.desc.clone(),
        })
    }

    fn convert_storage_map(&self, storage_map: &StorageMap) -> Result<ABIStorageMap, ABIError> {
        let key_type = self.resolve_storage_type(&storage_map.key_type)?;
        let value_type = self.resolve_storage_type(&storage_map.value_type)?;

        Ok(ABIStorageMap {
            key_type,
            value_type,
            desc: storage_map.desc.clone(),
            prefix: storage_map.prefix.clone(),
        })
    }

    pub fn get_global_abi_storage_maps(&self) -> Result<HashMap<String, ABIStorageMap>, ABIError> {
        self.state
            .maps
            .global_state
            .iter()
            .map(|(name, storage_map)| {
                let abi_storage_map = self.convert_storage_map(storage_map)?;
                Ok((name.clone(), abi_storage_map))
            })
            .collect()
    }

    pub fn get_local_abi_storage_maps(&self) -> Result<HashMap<String, ABIStorageMap>, ABIError> {
        self.state
            .maps
            .local_state
            .iter()
            .map(|(name, storage_map)| {
                let abi_storage_map = self.convert_storage_map(storage_map)?;
                Ok((name.clone(), abi_storage_map))
            })
            .collect()
    }

    pub fn get_box_abi_storage_maps(&self) -> Result<HashMap<String, ABIStorageMap>, ABIError> {
        self.state
            .maps
            .box_maps
            .iter()
            .map(|(name, storage_map)| {
                let abi_storage_map = self.convert_storage_map(storage_map)?;
                Ok((name.clone(), abi_storage_map))
            })
            .collect()
    }

    fn resolve_storage_type(&self, type_str: &str) -> Result<ABIType, ABIError> {
        if self.structs.contains_key(type_str) {
            ABIType::from_struct(type_str, &self.structs)
        } else {
            ABIType::from_str(type_str).map_err(|e| ABIError::ValidationError {
                message: format!("Failed to parse storage type '{}': {}", type_str, e),
            })
        }
    }
}
