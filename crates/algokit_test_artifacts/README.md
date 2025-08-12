# AlgoKit Test Artifacts

Centralized Algorand contract artifacts for testing across algokit-core crates.

## Usage

```toml
[dev-dependencies]
algokit_test_artifacts = { path = "../algokit_test_artifacts" }
```

```rust
use algokit_test_artifacts::{constant_product_amm, sandbox};

let arc56_content = constant_product_amm::APPLICATION_ARC56;
let sandbox_contract = sandbox::APPLICATION_ARC56;
```

For full list of available contracts, inspect the contracts folder inside the crate.

## Adding New Contracts

1. Place artifact in `contracts/{contract_name}/application.arc56.json`
2. Add module and constant in `src/lib.rs`
3. Ensure contract is ARC56 compliant
