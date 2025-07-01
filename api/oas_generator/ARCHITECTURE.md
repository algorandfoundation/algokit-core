# Architecture

## Overview

The Rust OAS Generator is a Jinja2-based code generator that converts OpenAPI 3.x specifications into Rust API clients. The architecture emphasizes separation of concerns between parsing, analysis, and generation phases.

## Core Components

### 1. CLI Interface (`cli.py`)

Entry point providing command-line interface with argument parsing, file validation, and error handling. Orchestrates the parsing and generation pipeline.

### 2. OpenAPI Parser (`parser/oas_parser.py`)

Transforms OpenAPI JSON specifications into structured Python dataclasses:

- **ParsedSpec**: Root container for parsed specification
- **Operation**: HTTP operations with Rust-specific metadata
- **Schema**: Data models with type information
- **Parameter/Response**: Request/response components

#### Key Features

- Recursive reference resolution (`$ref`)
- Msgpack operation detection
- Vendor extension processing (`x-algokit-signed-txn`)
- Dependency graph construction for schema relationships

### 3. Template Engine (`generator/template_engine.py`)

Jinja2-based code generation system with specialized analyzers:

- **OperationAnalyzer**: Groups operations, analyzes parameters
- **ResponseAnalyzer**: Handles success/error response types
- **TypeAnalyzer**: Manages imports and type dependencies
- **RustTemplateEngine**: Configures Jinja2 environment with custom filters

### 4. Template Filters (`generator/filters.py`)

Custom Jinja2 filters for Rust code generation:

- `rust_doc_comment`: Format documentation strings
- `ensure_semver`: Validate semantic versioning
- `is_signed_transaction_field`: Algorand transaction detection
- `get_dependencies_for_schema`: Dynamic import generation

### 5. Templates (`templates/`)

Jinja2 templates organized by component type:

```
templates/
├── base/           # Core library files
├── apis/           # API endpoint implementations  
├── models/         # Data model definitions
└── Cargo.toml.j2   # Project configuration
```

## Data Flow

1. **Parse**: CLI loads OpenAPI spec → OASParser creates structured dataclasses
2. **Analyze**: Template engine analyzers extract metadata for code generation
3. **Generate**: Jinja2 templates render Rust code using parsed data and analysis
4. **Output**: Generated files written to target directory structure

## Type System

### OpenAPI → Rust Mapping

| OpenAPI Type | Rust Type |
|--------------|-----------|
| `string` | `String` |
| `integer` | `u32` or `u64` |
| `number` | `f32`/`f64` |
| `boolean` | `bool` |
| `array` | `Vec<T>` |
| `object` | `serde_json::Value` |

### Special Cases

- **References**: `#/components/schemas/Model` → `Model`
- **Msgpack Fields**: Base64-encoded properties → `Vec<u8>`
- **Keywords**: Rust reserved words escaped with `r#` prefix
- **Integers**: Type selection based on:
  - `u64` for fields marked with `x-algokit-bigint: true`
  - `u32` for fields with `format: "int32"`
  - `u32` for fields with `maximum ≤ 4,294,967,295`
  - `u32` for small bounded fields (e.g., `maximum ≤ 100`)
  - `u32` for enum-like fields (descriptions containing "value `1`", "type.", etc.)
  - `u64` as default for potentially large blockchain values
- **x-algokit-bigint**: Fields marked with this extension explicitly use `u64` for 64-bit precision

## Msgpack Integration

Supports binary encoding for Algorand blockchain integration:

1. **Detection**: Content-Type `application/msgpack` or vendor extensions
2. **Propagation**: Dependency graph traversal marks related schemas
3. **Implementation**: Affected schemas implement `AlgorandMsgpack` trait

## Error Handling

- **File Operations**: Graceful handling with backup/restore
- **JSON Parsing**: Detailed error messages for malformed specs
- **Template Rendering**: Context validation and error reporting
- **Type Resolution**: Circular reference detection

## Extension Points

- **Custom Filters**: Add domain-specific template functions
- **Template Override**: Replace default templates with custom implementations
- **Schema Extensions**: Support additional vendor extensions
- **Type Mappings**: Extend OpenAPI to Rust type conversions
