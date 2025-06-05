pub mod simulate;

pub use simulate::*;

use crate::ModelRegistry;

/// Register all models in the registry
pub fn register_all_models(registry: &mut ModelRegistry) {
    // Register simulation models
    simulate::register_simulation_models(registry);
}
