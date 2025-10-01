use async_trait::async_trait;
use snafu::Snafu;
use std::collections::HashMap;

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[derive(Debug, Snafu)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum HttpError {
    #[snafu(display("HttpError: {message}"))]
    RequestError { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
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
pub struct HttpResponse {
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

#[cfg_attr(feature = "ffi_uniffi", uniffi::export(with_foreign))]
#[async_trait]
/// This trait must be implemented by any HTTP client that is used by our Rust crates.
/// It is assumed the implementing type will provide the hostname, port, headers, etc. as needed for each request.
///
/// By default, this trait requires the implementing type to be `Send + Sync`.
pub trait HttpClient: Send + Sync {
    async fn request(
        &self,
        http_method: HttpMethod,
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
                HttpError::RequestError {
                    message: format!("Invalid header name '{}': {}", header_name, e),
                }
            })?,
            reqwest::header::HeaderValue::from_str(header_value).map_err(|e| {
                HttpError::RequestError {
                    message: format!("Invalid header value '{}': {}", header_value, e),
                }
            })?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| HttpError::RequestError {
                message: format!("Failed to build HTTP client: {}", e),
            })?;
        Ok(DefaultHttpClient {
            client,
            base_url: base_url.to_string(),
        })
    }
}

#[cfg(feature = "default_client")]
#[async_trait]
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
        let method = reqwest::Method::from_bytes(method.as_str().as_bytes()).map_err(|e| {
            HttpError::RequestError {
                message: e.to_string(),
            }
        })?;

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
            .map_err(|e| HttpError::RequestError {
                message: e.to_string(),
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response text".to_string());
            return Err(HttpError::RequestError {
                message: format!("Request failed with status {}: {}", status, text),
            });
        }

        let response_headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .await
            .map_err(|e| HttpError::RequestError {
                message: e.to_string(),
            })?
            .to_vec();

        Ok(HttpResponse {
            body,
            headers: response_headers,
        })
    }
}
