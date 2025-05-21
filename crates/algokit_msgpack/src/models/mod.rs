pub mod account;
pub mod simulate;

pub use account::*;
pub use simulate::*;

use crate::ModelRegistry;

/// Register all models in the registry
pub fn register_all_models(registry: &mut ModelRegistry) {
    // Register simulation models
    simulate::register_simulation_models(registry);

    // Register account model
    account::register_account_model(registry);
}
