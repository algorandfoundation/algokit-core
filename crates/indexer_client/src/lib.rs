#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

pub mod apis;
pub mod models;

// Re-export the main client for convenience
pub use apis::IndexerClient;
