# Algorand API Tools

This package contains tools for working with the Algorand API specifications and generating HTTP client libraries.

## Prerequisites

- [Bun](https://bun.sh/) - JavaScript runtime and package manager

## Setup

```bash
# Install dependencies
bun install
```

## Available Scripts

### Convert OpenAPI 2.0 to OpenAPI 3.0

Converts the Algod OpenAPI 2.0 spec to OpenAPI 3.0:

```bash
bun run convert-openapi
```

The converted spec will be available at `specs/algod.oas3.json`.

### Generate API Clients

Generates TypeScript and Python API clients based on the OpenAPI spec:

```bash
bun run generate-clients
```

The generated clients will be available in the `../generated/` directory:

- `../generated/typescript/` - TypeScript client
- `../generated/python/` - Python client

## OpenAPI Specs for algorand apis

## Algod

The `algod.oas2.json` is taken directly from [go-algorand](https://github.com/algorand/go-algorand/blob/master/daemon/algod/api/algod.oas2.json). The script under [scripts/convert-openapi.ts](scripts/convert-openapi.ts) is used to convert the spec to OpenAPI 3.0 via [swagger converter](https://converter.swagger.io/) endpoint. The current approach is to manually edit and tweak the algod.oas2.json fixing known issues on a spec from go-algorand, then use the openapi-generator-cli to generate clients on the v3 spec. OpenAPI v3 is preferred for client generation as it offers enhanced schema features, better component reusability, and improved type definitions compared to v2. Additionally, most modern code generators like openapi-generator-cli provide better support and more accurate code generation when working with v3 specifications.

## OpenAPI Generator Configuration

The client generation is configured with the following options:

### TypeScript Client

- Package name: `@algorand/algod-client`
- ES6 support: true
- TypeScript 3+ features: true

### Python Client

- Package name: `algorand_algod_client`
- Ignoring various unneeded supporting files like tox.ini, git_push.sh, etc.

You can customize these options by editing the scripts in the `scripts/` directory.
