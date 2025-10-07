# Contributing Guide

## Principles

See the core principles in the repository's [README](../../README.md)

## Rust crates vs FFI libraries

The implementation of the rust crate should be completely seperate from the foreign interfaces. For example, [algokit_transact](../crates/algokit_transact/) does not depend on UniFFI. Instead, there's a seperate crate [algokit_transact_ffi](../crates/algokit_transact_ffi/) that provides the foreign interfaces.

## Development Tools and Commands

This repository provides several cargo binary commands to help with development and building packages for different languages.

### Available Binary Commands

#### 1. Package Building (`build_pkgs`)

```bash
cargo pkg <package> [language]
```

**Examples:**

- `cargo pkg algokit_transact python` - Build Python bindings
- `cargo pkg algokit_transact swift` - Build Swift bindings
- `cargo pkg algokit_transact` - Build all languages

#### 2. API Tools (`api_tools`)

```bash
cargo api <subcommand>
```

**Available subcommands:**

- `test-oas` - Test the OAS generator
- `format-oas` - Format the OAS generator code
- `lint-oas` - Lint and type-check the OAS generator
- `format-algod` / `format-indexer` / `format-kmd` - Format generated Rust clients
- `generate-algod` / `generate-indexer` / `generate-kmd` - Generate Rust API clients
- `generate-ts-algod` / `generate-ts-indexer` / `generate-ts-kmd` - Generate TypeScript API clients
- `generate-all` / `generate-ts-all` - Generate all Rust or TypeScript clients
- `convert-openapi` - Convert all OpenAPI specifications
- `convert-algod` / `convert-indexer` / `convert-kmd` - Convert individual OpenAPI specs

#### 3. Documentation Building

```bash
cargo run --bin build-docs --manifest-path docs/Cargo.toml
```

#### 4. Cargo Binary Management

```bash
cargo bin <args>
```

#### 5. UniFFI Bindings Generator

```bash
cargo run --bin uniffi-bindgen -- <args>
```

### 6. Pre-commit Hooks (Optional)

This repository includes [pre-commit](https://pre-commit.com/) hooks that run the same checks as `scripts/sanity.sh`.

**Run hooks manually on all files:**

```bash
pre-commit run --all-files
# or 
pre-commit install # to auto run on each commit
```

The hooks will automatically run `cargo fmt --check`, `cargo clippy`, `cargo check`, and the Rust test suite via `cargo t` (cargo-nextest, plus doc tests) on every commit.

## Development Workflow

### When Developing Core Rust Functionality

1. **Make changes to the core crates** (e.g., `algokit_transact`)
1. **Run tests** to ensure functionality works:

   ```bash
   cargo t -p algokit_transact
   ```

1. **Test FFI layer** if your changes affect the interface:

   ```bash
   cargo t -p algokit_transact_ffi
   ```

### When Developing Language Bindings

#### Python Development

1. **Build the Python bindings**:

   ```bash
   cargo pkg algokit_transact python
   ```

1. **Test Python bindings**:

   ```bash
   cd packages/python/algokit_transact
   poetry run pytest
   ```

#### Swift Development

1. **Build the Swift bindings**:

   ```bash
   cargo pkg algokit_transact swift
   ```

### Testing Your Changes

1. **Run Rust tests**:

   ```bash
   cargo t
   ```

1. **Run specific crate tests**:

   ```bash
   cargo t -p algokit_transact
   cargo t -p algokit_transact_ffi
   ```

1. **Run language-specific tests**:

   ```bash
   # Python
   cd packages/python/algokit_transact && poetry run pytest
   ```

1. **Run all tests** (as done in CI):

```bash
   ./scripts/test-all.sh                         # Comprehensive test script
```

Or manually:

```bash
cargo t                                        # Rust tests (cargo-nextest)
cargo pkg algokit_transact python            # Build Python
cd packages/python/algokit_transact && poetry run pytest  # Test Python
```

### Snapshot Testing (ABI Crate)

The `algokit_abi` crate uses [insta](https://insta.rs/) for snapshot testing to ensure consistent ARC56 contract parsing and serialization.

**Important for maintainers:**

- Tests may fail if snapshots need updating after code changes
- To review and approve new snapshots, run:

  ```bash
  cd crates/algokit_abi
  cargo insta review
  ```

- The `cargo-insta` tool is available in the workspace (no global installation needed)
- For more information on snapshot testing, see the [insta documentation](https://insta.rs/docs/)

**When snapshot tests fail:**

1. Review the snapshot diff carefully to ensure changes are intentional
1. Use `cargo insta review` to interactively approve/reject changes
1. Commit the updated `.snap` files along with your code changes

## Debugging Rust Code is VS Code

### Prerequisites

Install the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension for Visual Studio Code to debug Rust code.

### Debug Configurations

The project includes pre-configured debug configurations in `.vscode/launch.json`:

- **Debug unit tests in algokit_transact**: Debug tests for the core transaction functionality
- **Debug unit tests in algokit_transact_ffi**: Debug tests for the FFI bindings

### How to Debug

1. Set breakpoints by clicking in the gutter next to line numbers
1. Go to the Debug view (`Ctrl+Shift+D` or `Cmd+Shift+D`) and select a configuration for the crate you want to debug
1. Press `F5` to start debugging
1. Use the debug toolbar to step through code (`F10` for step over, `F11` for step into)
