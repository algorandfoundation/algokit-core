# Rust OAS Generator

A Jinja2-based generator that produces Rust API clients from OpenAPI 3.x specifications.

## Overview

This tool replaces traditional OpenAPI generators with a custom implementation optimized for Rust client generation. It supports msgpack encoding and Algorand-specific vendor extensions for signed transactions.

## Installation

```bash
cd api/oas_generator
uv sync
```

## Usage

### Basic Generation

```bash
rust_oas_generator spec.json
```

### Custom Output Directory

```bash
rust_oas_generator spec.json --output ./my_client --package-name my_api_client
```

### Verbose Output

```bash
rust_oas_generator spec.json --verbose
```

## Features

- **Complete Rust Client Generation**: APIs, models, and configuration
- **Msgpack Support**: Automatic detection and handling of binary encoding
- **Signed Transactions**: Algorand-specific vendor extension support (`x-algokit-signed-txn`)
- **Type Safety**: Comprehensive OpenAPI to Rust type mapping
- **Template-based**: Customizable Jinja2 templates for code generation

## Generated Structure

```
generated/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── apis/
    │   ├── mod.rs
    │   ├── client.rs
    │   ├── configuration.rs
    │   └── {endpoint}.rs
    └── models/
        ├── mod.rs
        └── {model}.rs
```

## Requirements

- Python 3.12+
- OpenAPI 3.x specification (JSON format)

## Development

```bash
# Install dev dependencies
uv install --dev

# Run tests
pytest

# Lint code
ruff check

# Type check
mypy rust_oas_generator
```
