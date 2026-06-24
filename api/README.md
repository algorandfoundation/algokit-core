# Algorand API Tools

This package contains tools for working with the Algorand API specifications and generating Rust HTTP client libraries using a custom Jinja2-based generator.

## Prerequisites

- [Python 3.12+](https://www.python.org/) - Required for the custom OAS generator
- [uv](https://docs.astral.sh/uv/) - Python package manager
- [Rust](https://rustup.rs/) - Required for compiling generated clients and running API tools
- [curl](https://curl.se/) - Used by `convert-*` to fetch the pinned OAS3 specs

## Setup

```bash
# Install Python dependencies for the OAS generator
cd api/oas_generator
uv install
```

## Available Scripts

> NOTE: These scripts can be run from the repository root using `cargo api <command>`.

### Fetch OpenAPI 3.0 specs

Fetches the Algod, Indexer, and KMD OAS3 specs from the pinned
[`algokit-oas-generator`](https://github.com/algorandfoundation/algokit-oas-generator)
commit (recorded in `specs/.oas-generator-sha`) into `specs/`:

```bash
cargo api convert-openapi
```

Fetch individual specifications:

```bash
# Convert only algod spec
cargo api convert-algod

# Convert only indexer spec
cargo api convert-indexer

# Convert only kmd spec
cargo api convert-kmd
```

The converted specs will be available at:

- `specs/algod.oas3.json`
- `specs/indexer.oas3.json`
- `specs/kmd.oas3.json`

### Generate Rust API Clients

Generate all Rust API clients using the custom Jinja2-based generator:

```bash
cargo api generate-all
```

Generate individual clients:

```bash
# Generate algod client only
cargo api generate-algod

# Generate indexer client only
cargo api generate-indexer

# Generate kmd client only
cargo api generate-kmd
```

The generated Rust clients will be available at:

- `../crates/algod_client/`
- `../crates/indexer_client/`
- `../crates/kmd_client/`

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
cargo api format-kmd
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
crates/{algod_client,indexer_client,kmd_client}/
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

The OAS3 specs in `specs/{algod,indexer,kmd}.oas3.json` are the canonical, Algorand-patched
specs published by [`algokit-oas-generator`](https://github.com/algorandfoundation/algokit-oas-generator)
— the same source consumed by `algokit-utils-ts` and `algokit-utils-py`. `algokit-core` no
longer maintains its own OAS2→OAS3 converter; `cargo api convert-*` simply fetches the published
spec for the pinned upstream commit.

### Pinning and upgrading

The pinned upstream commit is recorded in `specs/.oas-generator-sha` and mirrored by
`OAS_GENERATOR_SHA` in [`../tools/api_tools/src/main.rs`](../tools/api_tools/src/main.rs). To pull
a newer spec:

1. Update the SHA in `specs/.oas-generator-sha` and `OAS_GENERATOR_SHA`.
2. Run `cargo api convert-openapi` to re-fetch `specs/*.oas3.json`.
3. Run `cargo api generate-all` to regenerate the Rust clients.
4. Review the spec and client diffs and commit.

Algorand-specific spec fixes and vendor extensions (`x-algokit-bigint`, `x-algokit-signed-txn`,
`x-algokit-bytes-base64`, `x-algokit-field-rename`, `x-algokit-byte-length`,
`x-algorand-format: "Address"`, and the box/holding/locals reference markers) are produced
upstream and consumed by the Rust generator — see [`oas_generator/ARCHITECTURE.md`](oas_generator/ARCHITECTURE.md).

## Generator Configuration

The custom Rust generator is configured with:

- **Package names**: `algod_client`, `indexer_client`, `kmd_client`
- **Msgpack detection**: Automatic handling of binary-encoded fields
- **Algorand extensions**: Support for signed transaction via a vendor extension
- **Type safety**: Complete OpenAPI to Rust type mapping
- **Error handling**: Comprehensive error types and response handling

For detailed information about the generator architecture and customization options, see [`oas_generator/ARCHITECTURE.md`](oas_generator/ARCHITECTURE.md).
