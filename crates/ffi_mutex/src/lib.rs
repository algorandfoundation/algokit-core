#[cfg(feature = "ffi_uniffi")]
pub type InnerMutex<T> = tokio::sync::Mutex<T>;

#[cfg(feature = "ffi_wasm")]
pub type InnerMutex<T> = std::cell::RefCell<T>;

/// A wrapper around `tokio::sync::Mutex` (for uniffi) or `std::cell::RefCell` (for WASM)
/// Uniffi requires that exported Objects and `Send + Sync` (thread-safe), this we need to use a mutex. We use tokio's mutex in particular
/// because it is async-aware, allowing us to use it in async functions.
///
/// WASM doesn't need to be thread-safe, but we still need the interior mutability so we can have the same API on our FfiMutex in both
/// cases (otherwise we have a bunch of different code paths in our implementations). We probably could use some unsafe code
/// (i.e `UnsafeCell`) in place of the `RefCell`, but to make our lives easier and avoid accidental UB we use `RefCell` instead. The
/// performance difference is likely negligible in practice.
pub struct FfiMutex<T>(InnerMutex<T>);

impl<T> FfiMutex<T> {
    pub fn new(value: T) -> Self {
        #[cfg(feature = "ffi_uniffi")]
        return Self(tokio::sync::Mutex::new(value));

        #[cfg(feature = "ffi_wasm")]
        return Self(std::cell::RefCell::new(value));
    }

    #[cfg(feature = "ffi_uniffi")]
    pub fn blocking_lock(&self) -> tokio::sync::MutexGuard<'_, T> {
        self.0.blocking_lock()
    }

    #[cfg(feature = "ffi_wasm")]
    pub fn blocking_lock(&self) -> std::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }

    #[cfg(feature = "ffi_uniffi")]
    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, T> {
        self.0.lock().await
    }

    #[cfg(feature = "ffi_wasm")]
    pub async fn lock(&self) -> std::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }
}
