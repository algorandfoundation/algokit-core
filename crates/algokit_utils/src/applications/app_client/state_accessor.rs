use super::{AppClient, AppClientError};
use crate::clients::app_manager::AppState;
use algokit_abi::arc56_contract::{ABIStorageKey, ABIStorageMap};
use algokit_abi::{ABIType, ABIValue};
use async_trait::async_trait;
use base64::Engine;
use num_bigint::BigUint;
use std::collections::HashMap;

pub struct BoxStateAccessor<'app_client> {
    client: &'app_client AppClient,
}

pub struct StateAccessor<'app_client> {
    pub(crate) client: &'app_client AppClient,
}

impl<'app_client> StateAccessor<'app_client> {
    /// Create a new state accessor for the given app client.
    pub fn new(client: &'app_client AppClient) -> Self {
        Self { client }
    }

    /// Get an accessor for the application's global state.
    pub fn global_state(&self) -> AppStateAccessor<'_> {
        let provider = GlobalStateProvider {
            client: self.client,
        };
        AppStateAccessor::new("global".to_string(), Box::new(provider))
    }

    /// Get an accessor for an account's local state with this application.
    pub fn local_state(&self, address: &str) -> AppStateAccessor<'_> {
        let provider = LocalStateProvider {
            client: self.client,
            address: address.to_string(),
        };
        AppStateAccessor::new("local".to_string(), Box::new(provider))
    }

    /// Get an accessor for the application's box storage.
    pub fn box_storage(&self) -> BoxStateAccessor<'app_client> {
        BoxStateAccessor {
            client: self.client,
        }
    }
}

type GetStateResult = Result<HashMap<Vec<u8>, AppState>, AppClientError>;

#[async_trait(?Send)]
pub trait StateProvider {
    async fn get_app_state(&self) -> GetStateResult;
    fn get_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, AppClientError>;
    fn get_storage_maps(&self) -> Result<HashMap<String, ABIStorageMap>, AppClientError>;
}

struct GlobalStateProvider<'app_client> {
    client: &'app_client AppClient,
}

#[async_trait(?Send)]
impl StateProvider for GlobalStateProvider<'_> {
    async fn get_app_state(&self) -> GetStateResult {
        self.client.get_global_state().await
    }

    fn get_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, AppClientError> {
        self.client
            .app_spec
            .get_global_abi_storage_keys()
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    fn get_storage_maps(&self) -> Result<HashMap<String, ABIStorageMap>, AppClientError> {
        self.client
            .app_spec
            .get_global_abi_storage_maps()
            .map_err(|e| AppClientError::ABIError { source: e })
    }
}

struct LocalStateProvider<'app_client> {
    client: &'app_client AppClient,
    address: String,
}

#[async_trait(?Send)]
impl StateProvider for LocalStateProvider<'_> {
    async fn get_app_state(&self) -> GetStateResult {
        self.client.get_local_state(&self.address).await
    }

    fn get_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, AppClientError> {
        self.client
            .app_spec
            .get_local_abi_storage_keys()
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    fn get_storage_maps(&self) -> Result<HashMap<String, ABIStorageMap>, AppClientError> {
        self.client
            .app_spec
            .get_local_abi_storage_maps()
            .map_err(|e| AppClientError::ABIError { source: e })
    }
}

pub struct AppStateAccessor<'provider> {
    name: String,
    provider: Box<dyn StateProvider + 'provider>,
}

impl<'provider> AppStateAccessor<'provider> {
    /// Create a new app state accessor with the given name and provider.
    pub fn new(name: String, provider: Box<dyn StateProvider + 'provider>) -> Self {
        Self { name, provider }
    }

    /// Get all ABI-decoded state values for this storage type.
    pub async fn get_all(&self) -> Result<HashMap<String, Option<ABIValue>>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_key_map = self.provider.get_storage_keys()?;

        let mut result = HashMap::new();
        for (key_name, storage_key) in storage_key_map {
            let abi_value = self.decode_storage_key(&key_name, &storage_key, &state)?;
            result.insert(key_name, abi_value);
        }
        Ok(result)
    }

    /// Get a specific ABI-decoded state value by key name.
    pub async fn get_value(&self, key_name: &str) -> Result<Option<ABIValue>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_key_map = self.provider.get_storage_keys()?;

        let storage_key =
            storage_key_map
                .get(key_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state key '{}' not found", self.name, key_name),
                })?;

        self.decode_storage_key(key_name, storage_key, &state)
    }

    fn decode_storage_key(
        &self,
        key_name: &str,
        storage_key: &ABIStorageKey,
        state: &HashMap<Vec<u8>, AppState>,
    ) -> Result<Option<ABIValue>, AppClientError> {
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&storage_key.key)
            .map_err(|e| AppClientError::AppStateError {
                message: format!("Failed to decode {} key '{}': {}", self.name, key_name, e),
            })?;

        let value = state.get(&key_bytes);

        match value {
            None => Ok(None),
            Some(app_state) => Ok(Some(decode_app_state(&storage_key.value_type, app_state)?)),
        }
    }

    /// Get all key-value pairs from an ABI-defined state map.
    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<ABIValue, ABIValue>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_map_map = self.provider.get_storage_maps()?;
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state map '{}' not found", self.name, map_name),
                })?;
        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };

        let mut result = HashMap::new();
        for (key, app_state) in state.iter() {
            if !key.starts_with(&prefix_bytes) {
                continue;
            }

            let tail = &key[prefix_bytes.len()..];
            let decoded_key = storage_map
                .key_type
                .decode(tail)
                .map_err(|e| AppClientError::ABIError { source: e })?;

            let decoded_value = decode_app_state(&storage_map.value_type, app_state)?;
            result.insert(decoded_key, decoded_value);
        }

        Ok(result)
    }

    /// Get a specific value from an ABI-defined state map by key.
    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: ABIValue,
    ) -> Result<Option<ABIValue>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_map_map = self.provider.get_storage_maps()?;
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state map '{}' not found", self.name, map_name),
                })?;

        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };
        let encoded_key = storage_map
            .key_type
            .encode(&key)
            .map_err(|e| AppClientError::ABIError { source: e })?;
        let full_key = [prefix_bytes, encoded_key].concat();

        let value = state.get(&full_key);

        match value {
            None => Ok(None),
            Some(app_state) => Ok(Some(decode_app_state(&storage_map.value_type, app_state)?)),
        }
    }
}

impl BoxStateAccessor<'_> {
    /// Get all ABI-decoded box values for this application.
    pub async fn get_all(&self) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let box_storage_keys = self
            .client
            .app_spec
            .get_box_abi_storage_keys()
            .map_err(|e| AppClientError::ABIError { source: e })?;
        let mut results: HashMap<String, ABIValue> = HashMap::new();

        for (box_name, storage_key) in box_storage_keys {
            let box_name_bytes = base64::engine::general_purpose::STANDARD
                .decode(&storage_key.key)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode box key '{}': {}", box_name, e),
                })?;

            // TODO: what to do when it failed to fetch the box?
            let box_value = self.client.get_box_value(&box_name_bytes).await?;
            let abi_value = storage_key
                .value_type
                .decode(&box_value)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            results.insert(box_name, abi_value);
        }

        Ok(results)
    }

    /// Get a specific ABI-decoded box value by name.
    pub async fn get_value(&self, name: &str) -> Result<ABIValue, AppClientError> {
        let box_storage_keys = self
            .client
            .app_spec
            .get_box_abi_storage_keys()
            .map_err(|e| AppClientError::ABIError { source: e })?;

        let storage_key =
            box_storage_keys
                .get(name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("Box key '{}' not found", name),
                })?;

        let box_name_bytes = base64::engine::general_purpose::STANDARD
            .decode(&storage_key.key)
            .map_err(|e| AppClientError::AppStateError {
                message: format!("Failed to decode box key '{}': {}", name, e),
            })?;

        // TODO: what to do when it failed to fetch the box?
        let box_value = self.client.get_box_value(&box_name_bytes).await?;
        storage_key
            .value_type
            .decode(&box_value)
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    /// Get all key-value pairs from an ABI-defined box map.
    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<ABIValue, ABIValue>, AppClientError> {
        let storage_map_map = self
            .client
            .app_spec
            .get_box_abi_storage_maps()
            .map_err(|e| AppClientError::ABIError { source: e })?;
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("Box map '{}' not found", map_name),
                })?;

        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };

        let box_names = self.client.get_box_names().await?;
        let box_names = box_names
            .iter()
            .filter(|box_name| box_name.name_raw.starts_with(&prefix_bytes))
            .collect::<Vec<_>>();

        let mut results: HashMap<ABIValue, ABIValue> = HashMap::new();
        for box_name in box_names {
            let tail = &box_name.name_raw[prefix_bytes.len()..];
            let decoded_key = storage_map
                .key_type
                .decode(tail)
                .map_err(|e| AppClientError::ABIError { source: e })?;

            let box_value = self.client.get_box_value(&box_name.name_raw).await?;
            let decoded_value = storage_map
                .value_type
                .decode(&box_value)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            results.insert(decoded_key, decoded_value);
        }

        Ok(results)
    }

    /// Get a specific value from an ABI-defined box map by key.
    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: &ABIValue,
    ) -> Result<Option<ABIValue>, AppClientError> {
        let storage_map_map = self
            .client
            .app_spec
            .get_box_abi_storage_maps()
            .map_err(|e| AppClientError::ABIError { source: e })?;
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("Box map '{}' not found", map_name),
                })?;

        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };

        let encoded_key = storage_map
            .key_type
            .encode(key)
            .map_err(|e| AppClientError::ABIError { source: e })?;
        let full_key = [prefix_bytes, encoded_key].concat();

        let box_value = match self.client.get_box_value(&full_key).await {
            Ok(val) => val,
            Err(AppClientError::AppStateError { .. }) => return Ok(None),
            Err(e) => return Err(e),
        };

        let decoded = storage_map
            .value_type
            .decode(&box_value)
            .map_err(|e| AppClientError::ABIError { source: e })?;
        Ok(Some(decoded))
    }
}

fn decode_app_state(
    value_type: &ABIType,
    app_state: &AppState,
) -> Result<ABIValue, AppClientError> {
    match &app_state {
        AppState::Uint(uint_app_state) => Ok(ABIValue::Uint(BigUint::from(uint_app_state.value))),
        AppState::Bytes(bytes_app_state) => Ok(value_type
            .decode(&bytes_app_state.value_raw)
            .map_err(|e| AppClientError::ABIError { source: e })?),
    }
}
