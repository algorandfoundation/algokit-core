#[cfg(feature = "ffi_uniffi")]
pub type InnerMutex<T> = tokio::sync::Mutex<T>;

#[cfg(feature = "ffi_wasm")]
pub type InnerMutex<T> = std::cell::RefCell<T>;

// Create a wrapper that provides a unified interface
pub struct UnifiedMutex<T>(InnerMutex<T>);

impl<T> UnifiedMutex<T> {
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
