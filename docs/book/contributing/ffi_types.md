## FFI Types

This document provides some guidelines on how Rust types should be implemented for the FFI layer of utils.

### Tagged Enums

Not all languages support tagged enums, so we must turn them into structs.

#### Rust Implementation Crate

```rust
/// Configuration for application call resource population
#[derive(Debug, Clone)]
pub enum ResourcePopulation {
    /// Resource population is disabled
    Disabled,
    /// Resource population is enabled with optional access list usage
    Enabled { use_access_list: bool },
}
```

#### FFI Crate

```rust
#[derive(Debug, Clone)]
pub struct ResourcePopulation {
    enabled: bool,
    use_access_list: bool,
}

impl From<RustResourcePopulation> for ResourcePopulation {
    fn from(value: RustResourcePopulation) -> Self {
        match value {
            RustResourcePopulation::Disabled => ResourcePopulation {
                enabled: false,
                use_access_list: false,
            },
            RustResourcePopulation::Enabled { use_access_list } => ResourcePopulation {
                enabled: true,
                use_access_list,
            },
        }
    }
}

impl From<ResourcePopulation> for RustResourcePopulation {
    fn from(value: ResourcePopulation) -> Self {
        if value.enabled {
            RustResourcePopulation::Enabled {
                use_access_list: value.use_access_list,
            }
        } else {
            RustResourcePopulation::Disabled
        }
    }
}
```

### Structs

Structs can mostly have 1:1 mappings

#### Rust Implementation Crate

```rust
pub struct BuildParams {
    pub cover_app_call_inner_transaction_fees: bool,
    pub populate_app_call_resources: ResourcePopulation,
}
```

#### FFI Crate

```rust
#[derive(Debug, Clone)]
pub struct BuildParams {
    pub cover_app_call_inner_transaction_fees: bool,
    pub populate_app_call_resources: ResourcePopulation,
}

impl From<RustBuildParams> for BuildParams {
    fn from(value: RustBuildParams) -> Self {
        BuildParams {
            cover_app_call_inner_transaction_fees: value.cover_app_call_inner_transaction_fees,
            populate_app_call_resources: value.populate_app_call_resources.into(),
        }
    }
}

impl From<BuildParams> for RustBuildParams {
    fn from(value: BuildParams) -> Self {
        RustBuildParams {
            cover_app_call_inner_transaction_fees: value.cover_app_call_inner_transaction_fees,
            populate_app_call_resources: value.populate_app_call_resources.into(),
        }
    }
}
```

### Traits

Traits that are intended to be implemented by the FFI language should be defined as Uniffi [foreign traits](https://mozilla.github.io/uniffi-rs/0.28/foreign_traits.html). Foregin traits do not currently support reference types as arguments.

To convert between the two, you can use a struct that holds an `Arc` of the trait and then implement the corresponding trait for that struct.

#### Rust Implementation Crate

```rust
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String>;

    async fn sign_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<SignedTransaction, String> {
        let result = self.sign_transactions(&[transaction.clone()], &[0]).await?;
        Ok(result[0].clone())
    }
}
```

#### FFI Crate

```rust
#[uniffi::export(with_foreign)]
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, String>;

    async fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, String> {
        let result = self.sign_transactions(vec![transaction], vec![0]).await?;
        Ok(result[0].clone())
    }
}

struct RustTransactionSignerFromFfi {
    ffi_signer: Arc<dyn TransactionSigner>,
}

#[async_trait]
impl RustTransactionSigner for RustTransactionSignerFromFfi {
    async fn sign_transactions(
        &self,
        transactions: &[RustTransaction],
        indices: &[usize],
    ) -> Result<Vec<RustSignedTransaction>, String> {
        let ffi_txns: Result<Vec<Transaction>, _> = transactions
            .iter()
            .map(|t| t.to_owned().try_into())
            .collect();
        let ffi_txns = ffi_txns.map_err(|e| format!("Failed to convert transactions: {}", e))?;

        let ffi_signed_txns = self
            .ffi_signer
            .sign_transactions(ffi_txns, indices.iter().map(|&i| i as u8).collect())
            .await?;

        let signed_txns: Result<Vec<RustSignedTransaction>, _> = ffi_signed_txns
            .into_iter()
            .map(|st| st.try_into())
            .collect();
        signed_txns.map_err(|e| format!("Failed to convert signed transactions: {}", e))
    }
}

struct FfiTransactionSignerFromRust {
    rust_signer: Arc<dyn RustTransactionSigner>,
}

#[async_trait]
impl TransactionSigner for FfiTransactionSignerFromRust {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, String> {
        let rust_txns: Result<Vec<RustTransaction>, _> =
            transactions.into_iter().map(|t| t.try_into()).collect();
        let rust_txns = rust_txns.map_err(|e| format!("Failed to convert transactions: {}", e))?;

        let signed_txns = self
            .rust_signer
            .sign_transactions(
                &rust_txns,
                &indices.iter().map(|&i| i as usize).collect::<Vec<_>>(),
            )
            .await?;

        let ffi_signed_txns: Result<Vec<SignedTransaction>, _> =
            signed_txns.into_iter().map(|st| st.try_into()).collect();
        ffi_signed_txns.map_err(|e| format!("Failed to convert signed transactions: {}", e))
    }
}
```
