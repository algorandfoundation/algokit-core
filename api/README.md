# Algorand API Tools

This package contains tools for working with the Algorand API specifications and generating Rust HTTP client libraries using a custom Jinja2-based generator.

## Prerequisites

- [Python 3.12+](https://www.python.org/) - Required for the custom OAS generator
- [uv](https://docs.astral.sh/uv/) - Python package manager
- [Rust](https://rustup.rs/) - Required for compiling generated clients and running API tools
- [Bun](https://bun.sh/) - JavaScript runtime (only for convert-openapi script)

## Setup

```bash
# Install Python dependencies for the OAS generator
cd api/oas_generator
uv install

# Install JavaScript dependencies (only needed for convert-openapi)
cd ../
bun install
```

## Available Scripts

> NOTE: These scripts can be run from the repository root using `cargo api <command>`.

### Convert OpenAPI 2.0 to OpenAPI 3.0

Converts both Algod and Indexer OpenAPI 2.0 specs to OpenAPI 3.0:

```bash
cargo api convert-openapi
```

Convert individual specifications:

```bash
# Convert only algod spec
cargo api convert-algod

# Convert only indexer spec  
cargo api convert-indexer
```

The converted specs will be available at:
- `specs/algod.oas3.json`
- `specs/indexer.oas3.json`

### Generate Rust API Clients

Generate both Rust API clients using the custom Jinja2-based generator:

```bash
cargo api generate-all
```

Generate individual clients:

```bash
# Generate algod client only
cargo api generate-algod

# Generate indexer client only
cargo api generate-indexer
```

The generated Rust clients will be available at:
- `../crates/algod_client/`
- `../crates/indexer_client/`

### Development Scripts

```bash
# Test the OAS generator
cargo api test-oas

# Format the OAS generator code
cargo api format-oas

# Lint and type-check the OAS generator
cargo api lint-oas

# Format generated Rust code
cargo api format-algod
cargo api format-indexer
```

## Custom Rust OAS Generator

The project uses a custom Jinja2-based generator located in `oas_generator/` that creates optimized Rust API clients from OpenAPI 3.x specifications.

### Features

- **Complete Rust Client Generation**: APIs, models, and configuration
- **Msgpack Support**: Automatic detection and handling of binary encoding
- **Signed Transactions**: Algorand-specific vendor extension support (`x-algokit-signed-txn`)
- **Type Safety**: Comprehensive OpenAPI to Rust type mapping
- **Template-based**: Customizable Jinja2 templates for code generation

### Generated Structure

The generator creates complete Rust crates with the following structure:

```
crates/{algod_client,indexer_client}/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── apis/
    │   ├── mod.rs
    │   ├── client.rs
    │   └── {endpoint}.rs
    └── models/
        ├── mod.rs
        └── {model}.rs
```

## OpenAPI Specs for Algorand APIs

### Algod

The `algod.oas2.json` is taken directly from [go-algorand](https://github.com/algorand/go-algorand/blob/master/daemon/algod/api/algod.oas2.json). To convert the spec to OpenAPI 3.0, use `cargo api convert-algod` which runs the TypeScript script [scripts/convert-openapi.ts](scripts/convert-openapi.ts) via [swagger converter](https://converter.swagger.io/) endpoint.

### Indexer

The `indexer.oas2.json` is taken directly from [indexer](https://github.com/algorand/indexer/blob/master/api/indexer.oas2.json). To convert the spec to OpenAPI 3.0, use `cargo api convert-indexer` which runs the same TypeScript conversion script.

The current approach is to manually edit and tweak the OAS2 specs fixing known issues from the source repositories, then use the custom Rust OAS generator to generate clients from the v3 specs. OpenAPI v3 is preferred for client generation as it offers enhanced schema features, better component reusability, and improved type definitions compared to v2.

## Generator Configuration

The custom Rust generator is configured with:

- **Package names**: `algod_client`, `indexer_client`
- **Msgpack detection**: Automatic handling of binary-encoded fields
- **Algorand extensions**: Support for signed transaction via a vendor extension
- **Type safety**: Complete OpenAPI to Rust type mapping
- **Error handling**: Comprehensive error types and response handling

For detailed information about the generator architecture and customization options, see [`oas_generator/ARCHITECTURE.md`](oas_generator/ARCHITECTURE.md).
