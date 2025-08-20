use algod_client::{
    apis::{AlgodClient, Error as AlgodError},
    models::TealKeyValue,
};
use algokit_abi::{ABIMethod, ABIReturn, ABIType, ABIValue};
use algokit_transact::Address;
use base64::{Engine, engine::general_purpose::STANDARD as Base64};
use sha2::{Digest, Sha256};
use snafu::Snafu;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum TealTemplateValue {
    Int(u64),
    Bytes(Vec<u8>),
    String(String),
}

#[derive(Debug, Clone)]
pub struct DeploymentMetadata {
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
}

pub type TealTemplateParams = HashMap<String, TealTemplateValue>;

#[derive(Debug, Clone)]
pub struct CompiledTeal {
    pub teal: String,
    pub compiled: String,
    pub compiled_hash: String,
    pub compiled_base64_to_bytes: Vec<u8>,
    pub source_map: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub key_raw: Vec<u8>,
    pub key_base64: String,
    pub value_raw: Option<Vec<u8>>,
    pub value_base64: Option<String>,
    pub value: AppStateValue,
}

#[derive(Debug, Clone)]
pub enum AppStateValue {
    Uint(u64),
    Bytes(String),
}

#[derive(Debug, Clone)]
pub struct AppInformation {
    /// The application ID
    pub app_id: u64,
    /// The address of the application account
    pub app_address: Address,
    /// The approval program as bytecode
    pub approval_program: Vec<u8>,
    /// The clear state program as bytecode
    pub clear_state_program: Vec<u8>,
    /// The creator address of the application
    pub creator: String,
    /// Number of local state integers allocated
    pub local_ints: u32,
    /// Number of local state byte slices allocated
    pub local_byte_slices: u32,
    /// Number of global state integers allocated
    pub global_ints: u32,
    /// Number of global state byte slices allocated
    pub global_byte_slices: u32,
    /// Number of extra program pages (if any)
    pub extra_program_pages: Option<u32>,
    /// The current global state of the application
    /// Keys are stored as Vec<u8> for binary data support, matching TypeScript UInt8Array typing
    pub global_state: HashMap<Vec<u8>, AppState>,
}

#[derive(Debug, Clone)]
pub struct BoxName {
    /// The raw box name as bytes
    pub name_raw: Vec<u8>,
    /// The box name encoded as base64
    pub name_base64: String,
    /// The box name as a UTF-8 string (if valid)
    pub name: String,
}

/// Box identifier represented as binary data.
/// Box identifiers in Algorand are arbitrary binary data that can contain
/// non-UTF-8 bytes. They are base64-encoded when sent over HTTP APIs as JSON responses.
pub type BoxIdentifier = Vec<u8>;

pub const UPDATABLE_TEMPLATE_NAME: &str = "TMPL_UPDATABLE";
pub const DELETABLE_TEMPLATE_NAME: &str = "TMPL_DELETABLE";

/// Manages TEAL compilation and application state.
#[derive(Clone)]
pub struct AppManager {
    algod_client: Arc<AlgodClient>,
    compilation_results: Arc<Mutex<HashMap<String, CompiledTeal>>>,
}

impl AppManager {
    pub fn new(algod_client: Arc<AlgodClient>) -> Self {
        Self {
            algod_client,
            compilation_results: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a SHA256 hash of the TEAL code for use as cache key.
    /// This optimization reduces memory usage by storing a fixed-size hash
    /// instead of the full TEAL code string as the cache key.
    fn hash_teal_code(teal_code: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(teal_code.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub async fn compile_teal(&self, teal_code: &str) -> Result<CompiledTeal, AppManagerError> {
        let cache_key = Self::hash_teal_code(teal_code);

        // Check cache first
        {
            let cache = self.compilation_results.lock().unwrap();
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        let compile_response = self
            .algod_client
            .teal_compile(teal_code.as_bytes().to_vec(), Some(true))
            .await
            .map_err(|e| AppManagerError::AlgodClientError { source: e })?;

        let result = CompiledTeal {
            teal: teal_code.to_string(),
            compiled: Base64.encode(&compile_response.result),
            compiled_hash: compile_response.hash.clone(),
            compiled_base64_to_bytes: compile_response.result.clone(),
            source_map: compile_response.sourcemap,
        };

        // Cache the result
        {
            let mut cache = self.compilation_results.lock().unwrap();
            cache.insert(cache_key, result.clone());
        }

        Ok(result)
    }

    pub async fn compile_teal_template(
        &self,
        teal_template_code: &str,
        template_params: Option<&TealTemplateParams>,
        deployment_metadata: Option<&DeploymentMetadata>,
    ) -> Result<CompiledTeal, AppManagerError> {
        let mut teal_code = Self::strip_teal_comments(teal_template_code);

        if let Some(params) = template_params {
            teal_code = Self::replace_template_variables(&teal_code, params)?;
        }

        if let Some(metadata) = deployment_metadata {
            teal_code =
                Self::replace_teal_template_deploy_time_control_params(&teal_code, metadata)?;
        }

        self.compile_teal(&teal_code).await
    }

    pub fn get_compilation_result(&self, teal_code: &str) -> Option<CompiledTeal> {
        let cache_key = Self::hash_teal_code(teal_code);
        let cache = self.compilation_results.lock().unwrap();
        cache.get(&cache_key).cloned()
    }

    pub async fn get_by_id(&self, app_id: u64) -> Result<AppInformation, AppManagerError> {
        let app = self
            .algod_client
            .get_application_by_id(app_id)
            .await
            .map_err(|e| AppManagerError::AlgodClientError { source: e })?;

        Ok(AppInformation {
            app_id,
            app_address: Address::from_app_id(&app_id),
            approval_program: Base64.decode(&app.params.approval_program).map_err(|e| {
                AppManagerError::DecodingError {
                    message: e.to_string(),
                }
            })?,
            clear_state_program: Base64
                .decode(&app.params.clear_state_program)
                .map_err(|e| AppManagerError::DecodingError {
                    message: e.to_string(),
                })?,
            creator: app.params.creator,
            local_ints: app
                .params
                .local_state_schema
                .as_ref()
                .map(|s| s.num_uint as u32)
                .unwrap_or(0),
            local_byte_slices: app
                .params
                .local_state_schema
                .as_ref()
                .map(|s| s.num_byte_slice as u32)
                .unwrap_or(0),
            global_ints: app
                .params
                .global_state_schema
                .as_ref()
                .map(|s| s.num_uint as u32)
                .unwrap_or(0),
            global_byte_slices: app
                .params
                .global_state_schema
                .as_ref()
                .map(|s| s.num_byte_slice as u32)
                .unwrap_or(0),
            extra_program_pages: app.params.extra_program_pages.map(|p| p as u32),
            global_state: Self::decode_app_state(&app.params.global_state.unwrap_or_default())?,
        })
    }

    /// Get global state of application.
    /// Returns state keys as Vec<u8> for binary data support, matching TypeScript UInt8Array typing.
    pub async fn get_global_state(
        &self,
        app_id: u64,
    ) -> Result<HashMap<Vec<u8>, AppState>, AppManagerError> {
        let app_info = self.get_by_id(app_id).await?;
        Ok(app_info.global_state)
    }

    /// Get local state for account in application.
    /// Returns state keys as Vec<u8> for binary data support, matching TypeScript UInt8Array typing.
    pub async fn get_local_state(
        &self,
        app_id: u64,
        address: &str,
    ) -> Result<HashMap<Vec<u8>, AppState>, AppManagerError> {
        let app_info = self
            .algod_client
            .account_application_information(address, app_id, None)
            .await
            .map_err(|e| AppManagerError::AlgodClientError { source: e })?;

        let local_state = app_info
            .app_local_state
            .and_then(|state| state.key_value)
            .ok_or(AppManagerError::StateNotFound)?;

        Self::decode_app_state(&local_state)
    }

    /// Get names of all boxes for application.
    pub async fn get_box_names(&self, app_id: u64) -> Result<Vec<BoxName>, AppManagerError> {
        let box_result = self
            .algod_client
            .get_application_boxes(app_id, None)
            .await
            .map_err(|e| AppManagerError::AlgodClientError { source: e })?;

        let mut box_names = Vec::new();
        for b in box_result.boxes {
            let name_raw = b.name;
            let name_base64 = Base64.encode(&name_raw);
            let name =
                String::from_utf8(name_raw.clone()).unwrap_or_else(|_| format!("{:?}", name_raw));

            box_names.push(BoxName {
                name_raw,
                name_base64,
                name,
            });
        }
        Ok(box_names)
    }

    /// Get value stored in box.
    pub async fn get_box_value(
        &self,
        app_id: u64,
        box_name: &BoxIdentifier,
    ) -> Result<Vec<u8>, AppManagerError> {
        let (_, name_bytes) = Self::get_box_reference(box_name);
        let name_base64 = Base64.encode(&name_bytes);

        let box_result = self
            .algod_client
            .get_application_box_by_name(app_id, &name_base64)
            .await
            .map_err(|e| AppManagerError::AlgodClientError { source: e })?;

        Base64
            .decode(&box_result.value)
            .map_err(|e| AppManagerError::DecodingError {
                message: e.to_string(),
            })
    }

    /// Get values for multiple boxes.
    pub async fn get_box_values(
        &self,
        app_id: u64,
        box_names: &[BoxIdentifier],
    ) -> Result<Vec<Vec<u8>>, AppManagerError> {
        let mut values = Vec::new();
        for box_name in box_names {
            values.push(self.get_box_value(app_id, box_name).await?);
        }
        Ok(values)
    }

    /// Decode box value using ABI type.
    ///
    /// This method takes an ABIType directly and uses it to decode the box value,
    /// returning an ABIValue directly for simpler usage patterns that match the
    /// TypeScript and Python implementations.
    ///
    /// # Arguments
    /// * `app_id` - The application ID
    /// * `box_name` - The box name identifier
    /// * `abi_type` - The ABI type to use for decoding
    ///
    /// # Returns
    /// An ABIValue containing the decoded box value
    pub async fn get_box_value_from_abi_type(
        &self,
        app_id: u64,
        box_name: &BoxIdentifier,
        abi_type: &ABIType,
    ) -> Result<ABIValue, AppManagerError> {
        let raw_value = self.get_box_value(app_id, box_name).await?;
        let decoded_value =
            abi_type
                .decode(&raw_value)
                .map_err(|e| AppManagerError::ABIDecodeError {
                    message: e.to_string(),
                })?;
        Ok(decoded_value)
    }

    /// Decode multiple box values using ABI type.
    ///
    /// This method takes an ABIType directly and uses it to decode multiple box values,
    /// returning ABIValue objects directly for simpler usage patterns that match the
    /// TypeScript and Python implementations.
    ///
    /// # Arguments
    /// * `app_id` - The application ID
    /// * `box_names` - The box name identifiers
    /// * `abi_type` - The ABI type to use for decoding
    ///
    /// # Returns
    /// A vector of ABIValue objects containing the decoded box values
    pub async fn get_box_values_from_abi_type(
        &self,
        app_id: u64,
        box_names: &[BoxIdentifier],
        abi_type: &ABIType,
    ) -> Result<Vec<ABIValue>, AppManagerError> {
        let mut values = Vec::new();
        for box_name in box_names {
            values.push(
                self.get_box_value_from_abi_type(app_id, box_name, abi_type)
                    .await?,
            );
        }
        Ok(values)
    }

    /// Get ABI return value from transaction confirmation.
    pub fn get_abi_return(
        confirmation_data: &[u8],
        method: &ABIMethod,
    ) -> Result<Option<ABIReturn>, AppManagerError> {
        if let Some(return_type) = &method.returns {
            let return_value = return_type.decode(confirmation_data).map_err(|e| {
                AppManagerError::ABIDecodeError {
                    message: e.to_string(),
                }
            })?;

            Ok(Some(ABIReturn {
                method: method.clone(),
                raw_return_value: confirmation_data.to_vec(),
                return_value,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get box reference from identifier.
    pub fn get_box_reference(box_id: &BoxIdentifier) -> (u64, Vec<u8>) {
        (0, box_id.clone())
    }

    /// Decode application state from raw format.
    /// Keys are decoded from base64 to Vec<u8> for binary data support, matching TypeScript UInt8Array typing.
    pub fn decode_app_state(
        state: &[TealKeyValue],
    ) -> Result<HashMap<Vec<u8>, AppState>, AppManagerError> {
        let mut state_values = HashMap::new();

        for state_val in state {
            let key_raw =
                Base64
                    .decode(&state_val.key)
                    .map_err(|e| AppManagerError::DecodingError {
                        message: e.to_string(),
                    })?;

            // TODO(stabilization): Consider r#type pattern consistency across API vs ABI types (PR #229 comment)
            let (value_raw, value_base64, value) = match state_val.value.r#type {
                1 => {
                    // Bytes - now already decoded from base64 by serde
                    let value_raw = state_val.value.bytes.clone();
                    let value_base64 = Base64.encode(&value_raw);
                    let value_str = String::from_utf8(value_raw.clone())
                        .unwrap_or_else(|_| hex::encode(&value_raw));
                    (
                        Some(value_raw),
                        Some(value_base64),
                        AppStateValue::Bytes(value_str),
                    )
                }
                2 => (None, None, AppStateValue::Uint(state_val.value.uint)),
                _ => {
                    return Err(AppManagerError::DecodingError {
                        message: format!("Unknown state data type: {}", state_val.value.r#type),
                    });
                }
            };

            state_values.insert(
                key_raw.clone(),
                AppState {
                    key_raw: key_raw.clone(),
                    key_base64: Base64.encode(&key_raw),
                    value_raw,
                    value_base64,
                    value,
                },
            );
        }

        Ok(state_values)
    }

    /// Replace template variables in TEAL code.
    pub fn replace_template_variables(
        program: &str,
        template_values: &TealTemplateParams,
    ) -> Result<String, AppManagerError> {
        let mut program_lines: Vec<String> = program.lines().map(|line| line.to_string()).collect();

        for (template_variable_name, template_value) in template_values {
            let token = if template_variable_name.starts_with("TMPL_") {
                template_variable_name.clone()
            } else {
                format!("TMPL_{}", template_variable_name)
            };

            let value = match template_value {
                TealTemplateValue::Int(i) => i.to_string(),
                TealTemplateValue::String(s) => {
                    if s.parse::<i64>().is_ok() {
                        s.clone()
                    } else {
                        format!("0x{}", hex::encode(s.as_bytes()))
                    }
                }
                TealTemplateValue::Bytes(b) => format!("0x{}", hex::encode(b)),
            };

            program_lines = Self::replace_template_variable(&program_lines, &token, &value);
        }

        Ok(program_lines.join("\n"))
    }

    /// Replace template variable with proper boundary checking.
    fn replace_template_variable(
        program_lines: &[String],
        token: &str,
        replacement: &str,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let token_index_offset = replacement.len() as i32 - token.len() as i32;

        for line in program_lines {
            let comment_index = Self::find_unquoted_string(line, "//").unwrap_or(line.len());
            let mut code = line[..comment_index].to_string();
            let comment = &line[comment_index..];
            let mut trailing_index = 0;

            while let Some(token_index) = Self::find_template_token(&code, token, trailing_index) {
                trailing_index = token_index + token.len();
                let prefix = &code[..token_index];
                let suffix = &code[trailing_index..];
                code = format!("{}{}{}", prefix, replacement, suffix);
                trailing_index = ((trailing_index as i32) + token_index_offset).max(0) as usize;
            }

            result.push(format!("{}{}", code, comment));
        }

        result
    }

    /// Find template token with boundary checking.
    fn find_template_token(line: &str, token: &str, start_index: usize) -> Option<usize> {
        let end_index = line.len();
        let mut index = start_index;

        while index < end_index {
            if let Some(token_index) = Self::find_unquoted_string(&line[index..], token) {
                let actual_token_index = index + token_index;
                let trailing_index = actual_token_index + token.len();

                // Check boundaries - ensure it's a whole token
                let valid_start = actual_token_index == 0
                    || !Self::is_valid_token_character(
                        line.chars()
                            .nth(actual_token_index.saturating_sub(1))
                            .unwrap_or(' '),
                    );
                let valid_end = trailing_index >= line.len()
                    || !Self::is_valid_token_character(
                        line.chars().nth(trailing_index).unwrap_or(' '),
                    );

                if valid_start && valid_end {
                    return Some(actual_token_index);
                }
                index = trailing_index;
            } else {
                break;
            }
        }
        None
    }

    /// Check if character is valid for token.
    fn is_valid_token_character(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    /// Replace deploy-time control parameters.
    pub fn replace_teal_template_deploy_time_control_params(
        teal_template_code: &str,
        params: &DeploymentMetadata,
    ) -> Result<String, AppManagerError> {
        let mut result = teal_template_code.to_string();

        if let Some(updatable) = params.updatable {
            if !teal_template_code.contains(UPDATABLE_TEMPLATE_NAME) {
                return Err(AppManagerError::TemplateVariableNotFound {
                    message: format!(
                        "Deploy-time updatability control requested, but {} not present in TEAL code",
                        UPDATABLE_TEMPLATE_NAME
                    ),
                });
            }
            result = result.replace(UPDATABLE_TEMPLATE_NAME, &(updatable as u8).to_string());
        }

        if let Some(deletable) = params.deletable {
            if !teal_template_code.contains(DELETABLE_TEMPLATE_NAME) {
                return Err(AppManagerError::TemplateVariableNotFound {
                    message: format!(
                        "Deploy-time deletability control requested, but {} not present in TEAL code",
                        DELETABLE_TEMPLATE_NAME
                    ),
                });
            }
            result = result.replace(DELETABLE_TEMPLATE_NAME, &(deletable as u8).to_string());
        }

        Ok(result)
    }

    /// Strip comments from TEAL code.
    pub fn strip_teal_comments(teal_code: &str) -> String {
        teal_code
            .lines()
            .map(|line| {
                if let Some(comment_pos) = Self::find_unquoted_string(line, "//") {
                    line[..comment_pos].trim_end()
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Find unquoted string in TEAL line.
    fn find_unquoted_string(line: &str, token: &str) -> Option<usize> {
        let mut in_quotes = false;
        let mut in_base64 = false;
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                ' ' | '(' if !in_quotes && Self::last_token_base64(line, i) => {
                    in_base64 = true;
                }
                ' ' | ')' if !in_quotes && in_base64 => {
                    in_base64 = false;
                }
                '\\' if in_quotes => {
                    // Skip next character
                    i += 1;
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                _ if !in_quotes && !in_base64 => {
                    if i + token.len() <= line.len() && &line[i..i + token.len()] == token {
                        return Some(i);
                    }
                }
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// Check if last token is base64.
    fn last_token_base64(line: &str, index: usize) -> bool {
        if let Some(last_token) = line[..index].split_whitespace().last() {
            matches!(last_token, "base64" | "b64")
        } else {
            false
        }
    }
}

/// Errors that can occur during app manager operations.
#[derive(Debug, Snafu)]
pub enum AppManagerError {
    #[snafu(display("Algod client error: {source}"))]
    AlgodClientError { source: AlgodError },

    #[snafu(display("Template variable not found: {message}"))]
    TemplateVariableNotFound { message: String },

    #[snafu(display("Decoding error: {message}"))]
    DecodingError { message: String },

    #[snafu(display("State not found"))]
    StateNotFound,

    #[snafu(display("ABI decode error: {message}"))]
    ABIDecodeError { message: String },
}
