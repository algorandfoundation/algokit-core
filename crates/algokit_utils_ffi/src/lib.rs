#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "ffi_uniffi")]
pub mod uniffi_ffi;

#[cfg(feature = "ffi_wasm")]
pub mod wasm;
