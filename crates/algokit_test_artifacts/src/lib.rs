//! # AlgoKit Test Artifacts
//!
//! Centralized contract artifacts for testing across algokit-core crates.
//!
//! This crate provides static access to various Algorand contract artifacts
//! used for testing purposes. Artifacts are organized by contract name in
//! individual folders.
//!
//! Each contract has its own folder named after the contract, containing
//! standardized file names like `application.arc56.json` or `application.json`.

/// Constant Product AMM contract artifacts
pub mod constant_product_amm {
    /// Automated Market Maker (AMM) contract (ARC56)
    ///
    /// A full-featured AMM implementation for testing complex scenarios
    /// including governance, liquidity provision, and trading operations.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/constant_product_amm/application.arc56.json");
}

/// Sandbox contract artifacts
pub mod sandbox {
    /// Sandbox contract (ARC56)
    ///
    /// A general-purpose contract with various methods for testing
    /// different application call scenarios.
    pub const APPLICATION_ARC56: &str = include_str!("../contracts/sandbox/application.arc56.json");
}

/// State management contract artifacts
pub mod state_management_demo {
    /// State management contract (ARC56)
    ///
    /// A comprehensive contract for testing state operations,
    /// struct handling, and complex ABI interactions.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/state_management_demo/application.arc56.json");
}

/// ABI payment call test contract artifacts
pub mod abi_payment_call_test {
    /// Simple call transaction test contract (ARC56)
    ///
    /// A minimal contract for testing basic ABI call transactions.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/abi_payment_call_test/application.arc56.json");
}

/// Template variables contract artifacts
pub mod template_variables {
    /// Template variables contract (ARC56)
    ///
    /// Contract with template variables for testing template substitution
    /// and dynamic contract configuration.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/template_variables/application.arc56.json");
}

/// Inner fee contract artifacts
pub mod inner_fee_contract {
    /// Inner fee coverage contract (ARC32)
    ///
    /// Contract for testing inner transaction fee coverage scenarios
    /// and budget management.
    pub const APPLICATION: &str = include_str!("../contracts/inner_fee_contract/application.json");
}

/// Nested contract artifacts
pub mod nested_contract {
    /// Nested functionality contract (ARC32)
    ///
    /// Contract for testing nested application call scenarios
    /// and complex transaction composition.
    pub const APPLICATION: &str = include_str!("../contracts/nested_contract/application.json");
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/nested_contract/application.arc56.json");
}

/// Nested struct storage contract artifacts
pub mod nested_struct_storage {
    /// Nested struct testing contract (ARC56)
    ///
    /// Contract specifically designed for testing nested data structures
    /// and complex ABI type handling.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/nested_struct_storage/application.arc56.json");
}

/// ARC56 struct operations contract artifacts
pub mod arc56_struct_operations {
    /// ARC56 test contract (ARC56)
    ///
    /// Contract for testing ARC56 features including complex structs,
    /// nested types, and advanced ABI method signatures.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/arc56_struct_operations/application.arc56.json");
}

/// Complex struct test contract artifacts
pub mod complex_struct_test {
    /// Structs testing contract (ARC56)
    ///
    /// Contract for testing advanced struct nesting patterns
    /// and complex type hierarchies.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/complex_struct_test/application.arc56.json");
}

/// Zero coupon bond contract artifacts
pub mod zero_coupon_bond {
    /// Zero coupon bond financial contract (ARC56)
    ///
    /// Complex financial contract with multiple structs for testing
    /// real-world DeFi scenarios and advanced state management.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/zero_coupon_bond/application.arc56.json");
}

/// NFD (Non Fungible Domains) contract artifacts
pub mod nfd {
    /// NFD instance contract (ARC56)
    ///
    /// Real-world contract for domain name trading with extensive
    /// method signatures and complex transaction patterns.
    pub const APPLICATION_ARC56: &str = include_str!("../contracts/nfd/application.arc56.json");
}

/// Reti validator registry contract artifacts
pub mod reti {
    /// Reti validator registry contract (ARC56)
    ///
    /// Validator registry contract with complex state management
    /// and advanced staking mechanisms.
    pub const APPLICATION_ARC56: &str = include_str!("../contracts/reti/application.arc56.json");
}

/// Void return test contract artifacts
pub mod void_return_test {
    /// Void minimal contract (ARC56)
    ///
    /// Minimal contract for testing edge cases and basic ARC56
    /// functionality with minimal complexity.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/void_return_test/application.arc56.json");
}

/// Nested contract calls test contract artifacts
pub mod nested_contract_calls {
    /// Nested transaction test contract (ARC56)
    ///
    /// Contract for testing nested application calls and
    /// complex transaction composition patterns.
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/nested_contract_calls/application.arc56.json");
}

/// Testing app contract artifacts
pub mod testing_app {
    /// General-purpose testing contract (ARC56)
    ///
    /// Contract with updatable/deletable template variables and
    /// various methods for comprehensive app deployer testing.
    pub const APPLICATION: &str = include_str!("../contracts/testing_app/application.json");
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/testing_app/application.arc56.json");
}

/// HelloWorld contract artifacts
pub mod hello_world {
    /// HelloWorld contract (ARC56)
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/hello_world/application.arc56.json");
}

/// Testing app (puya compiler) contract artifacts
pub mod testing_app_puya {
    /// Testing app (puya compiler) contract (ARC56)
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/testing_app_puya/application.arc56.json");
}

/// Testing app ARC56 templates (control-template capable)
pub mod testing_app_arc56_templates {
    /// ARC56 app spec used in template-var/error mapping tests
    pub const APP_SPEC_ARC56: &str =
        include_str!("../contracts/testing_app_arc56/app_spec.arc56.json");
}
/// Extra pages test contract artifacts
pub mod extra_pages_test {
    /// Aggregate application (ARC56) used by extra pages tests
    pub const APPLICATION_ARC56: &str =
        include_str!("../contracts/extra_pages_test/application.arc56.json");

    /// Small program variant (ARC56)
    pub const SMALL_ARC56: &str = include_str!("../contracts/extra_pages_test/small.arc56.json");

    /// Large program variant (ARC56)
    pub const LARGE_ARC56: &str = include_str!("../contracts/extra_pages_test/large.arc56.json");
}

/// State contract artifacts (control-aware spec)
pub mod state_contract {
    /// State contract (ARC56) with UPDATABLE/DELETABLE/VALUE template variables
    pub const STATE_ARC56: &str = include_str!("../contracts/state_contract/state.arc56.json");
}

/// Resource population contract artifacts
pub mod resource_population {
    /// Resource population testing contract (ARC32) targeting AVM V8
    ///
    /// Contract for testing complex resource population scenarios.
    pub const APPLICATION_V8: &str =
        include_str!("../contracts/resource_population/ResourcePackerv8.arc32.json");

    /// Resource population testing contract (ARC32) targeting AVM V9
    ///
    /// Contract for testing complex resource population scenarios.
    pub const APPLICATION_V9: &str =
        include_str!("../contracts/resource_population/ResourcePackerv9.arc32.json");
}

pub mod box_map_test {
    /// Box map testing contract (ARC56)
    ///
    /// Contract for testing box map operations and complex data handling.
    pub const APPLICATION_ARC56: &str = include_str!("../contracts/boxmap/application.arc56.json");
}
