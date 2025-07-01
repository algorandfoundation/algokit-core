use async_trait::async_trait;
use std::collections::HashMap;

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum HttpError {
    #[error("HttpError: {0}")]
    RequestError(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_wasm", derive(serde::Serialize, serde::Deserialize))]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
#[cfg_attr(feature = "ffi_wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "ffi_wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "ffi_wasm", derive(serde::Serialize, serde::Deserialize))]
pub struct HttpResponse {
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

#[cfg(not(feature = "ffi_wasm"))]
#[cfg_attr(feature = "ffi_uniffi", uniffi::export(with_foreign))]
#[async_trait]
/// This trait must be implemented by any HTTP client that is used by our Rust crates.
/// It is assumed the implementing type will provide the hostname, port, headers, etc. as needed for each request.
///
/// By default, this trait requires the implementing type to be `Send + Sync`.
/// For WASM targets, enable the `ffi_wasm` feature to use a different implementation that is compatible with WASM.
pub trait HttpClient: Send + Sync {
    async fn request(
        &self,
        method: HttpMethod,
        path: String,
        query: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError>;
}

#[cfg(feature = "default_client")]
pub struct DefaultHttpClient {
    client: reqwest::Client,
    base_url: String,
}

#[cfg(feature = "default_client")]
impl DefaultHttpClient {
    pub fn new(base_url: &str) -> Self {
        DefaultHttpClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn with_header(
        base_url: &str,
        header_name: &str,
        header_value: &str,
    ) -> Result<Self, HttpError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::HeaderName::from_bytes(header_name.as_bytes()).map_err(|e| {
                HttpError::RequestError(format!("Invalid header name '{}': {}", header_name, e))
            })?,
            reqwest::header::HeaderValue::from_str(header_value).map_err(|e| {
                HttpError::RequestError(format!("Invalid header value '{}': {}", header_value, e))
            })?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| HttpError::RequestError(format!("Failed to build HTTP client: {}", e)))?;
        Ok(DefaultHttpClient {
            client,
            base_url: base_url.to_string(),
        })
    }
}

#[cfg(feature = "default_client")]
#[cfg_attr(feature = "ffi_wasm", async_trait(?Send))]
#[cfg_attr(not(feature = "ffi_wasm"), async_trait)]
impl HttpClient for DefaultHttpClient {
    async fn request(
        &self,
        method: HttpMethod,
        path: String,
        query: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let method = reqwest::Method::from_bytes(method.as_str().as_bytes())
            .map_err(|e| HttpError::RequestError(e.to_string()))?;

        let mut request_builder = self.client.request(method, &url);

        if let Some(query_params) = query {
            request_builder = request_builder.query(&query_params);
        }

        if let Some(header_params) = headers {
            for (key, value) in header_params {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(body_data) = body {
            request_builder = request_builder.body(body_data);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| HttpError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response text".to_string());
            return Err(HttpError::RequestError(format!(
                "Request failed with status {}: {}",
                status, text
            )));
        }

        let response_headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .await
            .map_err(|e| HttpError::RequestError(e.to_string()))?
            .to_vec();

        Ok(HttpResponse {
            body,
            headers: response_headers,
        })
    }
}

// WASM-specific implementations
#[cfg(feature = "ffi_wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ffi_wasm")]
use js_sys::Uint8Array;

#[cfg(feature = "ffi_wasm")]
use tsify_next::Tsify;

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
pub trait HttpClient {
    async fn request(
        &self,
        method: HttpMethod,
        path: String,
        query: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError>;
}

#[wasm_bindgen]
#[cfg(feature = "ffi_wasm")]
extern "C" {
    /// The interface for the JavaScript-based HTTP client that will be used in WASM environments.
    ///
    /// This mirrors the `HttpClient` trait, but wasm-bindgen doesn't support foreign traits so we define it separately.
    pub type WasmHttpClient;

    #[wasm_bindgen(method, catch)]
    async fn request(
        this: &WasmHttpClient,
        method: &str,
        path: &str,
        query: &JsValue,
        body: &JsValue,
        headers: &JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[cfg(feature = "ffi_wasm")]
#[async_trait(?Send)]
impl HttpClient for WasmHttpClient {
    async fn request(
        &self,
        method: HttpMethod,
        path: String,
        query: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        let query_js = match query {
            Some(q) => serde_wasm_bindgen::to_value(&q).unwrap_or(JsValue::NULL),
            None => JsValue::NULL,
        };

        let body_js = match body {
            Some(b) => {
                let array = Uint8Array::new_with_length(b.len() as u32);
                array.copy_from(&b);
                array.into()
            }
            None => JsValue::NULL,
        };

        let headers_js = match headers {
            Some(h) => serde_wasm_bindgen::to_value(&h).unwrap_or(JsValue::NULL),
            None => JsValue::NULL,
        };

        let result = self
            .request(method.as_str(), &path, &query_js, &body_js, &headers_js)
            .await
            .map_err(|e| {
                HttpError::RequestError(
                    e.as_string().unwrap_or(
                        "A HTTP error occurred in JavaScript, but it cannot be converted to a string"
                            .to_string(),
                    ),
                )
            })?;

        // Parse the response from JavaScript
        let response = HttpResponse::from_js(result)
            .map_err(|e| HttpError::RequestError(format!("Failed to parse response: {:?}", e)))?;

        Ok(response)
    }
}
