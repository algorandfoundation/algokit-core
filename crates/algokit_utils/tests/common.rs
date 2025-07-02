use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging for tests. Call this once at the start of any test that needs logging.
/// Safe to call multiple times - will only initialize once across the entire test suite.
pub fn init_test_logging() {
    INIT.call_once(|| {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .format_target(true) // Include target in output for better debugging
            .format_module_path(false) // Keep output cleaner
            .try_init();
    });
}
