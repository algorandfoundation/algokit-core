//! An [`HttpClient`] wrapper that records the raw bytes of the last response.
//!
//! The generated endpoint methods decode the response body internally and only hand back a typed
//! model, so there is no way to see the raw JSON a node actually returned. This wrapper delegates
//! every request to an inner [`DefaultHttpClient`] and stashes the response body on the way through,
//! letting a test assert the typed call succeeded *and* validate the untouched bytes against the
//! spec schema.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use algod_client::AlgodClient;
use algokit_http_client::{DefaultHttpClient, HttpClient, HttpError, HttpMethod, HttpResponse};
use async_trait::async_trait;

/// Localnet algod endpoint and token, matching [`AlgodClient::localnet`].
const LOCALNET_ALGOD_URL: &str = "http://localhost:4001";
const ALGOD_API_TOKEN_HEADER: &str = "X-Algo-API-Token";
const LOCALNET_TOKEN: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

/// Wraps a [`DefaultHttpClient`] and records the body of the most recent response.
pub struct CapturingHttpClient {
    inner: DefaultHttpClient,
    last_body: Mutex<Option<Vec<u8>>>,
}

impl CapturingHttpClient {
    /// Build a capturing client pointed at the local algod node.
    pub fn localnet() -> Arc<Self> {
        let inner = DefaultHttpClient::with_header(
            LOCALNET_ALGOD_URL,
            ALGOD_API_TOKEN_HEADER,
            LOCALNET_TOKEN,
        )
        .expect("failed to build localnet http client");
        Arc::new(Self {
            inner,
            last_body: Mutex::new(None),
        })
    }

    /// An [`AlgodClient`] backed by this capturing client.
    pub fn client(self: &Arc<Self>) -> AlgodClient {
        AlgodClient::new(self.clone())
    }

    /// The raw bytes of the most recent response. Panics if no request has been made yet.
    pub fn last_body(&self) -> Vec<u8> {
        self.last_body
            .lock()
            .expect("last_body mutex poisoned")
            .clone()
            .expect("no response captured yet")
    }
}

#[async_trait]
impl HttpClient for CapturingHttpClient {
    async fn request(
        &self,
        http_method: HttpMethod,
        path: String,
        query: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        let response = self
            .inner
            .request(http_method, path, query, body, headers)
            .await?;
        *self.last_body.lock().expect("last_body mutex poisoned") = Some(response.body.clone());
        Ok(response)
    }
}
