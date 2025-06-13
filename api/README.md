# Algorand API Tools

This package contains tools for working with the Algorand API specifications and generating HTTP client libraries.

## Prerequisites

- [Bun](https://bun.sh/) - JavaScript runtime and package manager
- [OpenJDK](https://adoptium.net/) - Java Development Kit

## Setup

```bash
# Install dependencies
bun install
```

## Available Scripts

> NOTE: These scripts must be run inside the `./api` directory.

### Convert OpenAPI 2.0 to OpenAPI 3.0

Converts the Algod OpenAPI 2.0 spec to OpenAPI 3.0:

```bash
bun run convert-openapi
```

The converted spec will be available at `specs/algod.oas3.json`.

### Generate API Clients

Generates TypeScript and Python API clients based on the OpenAPI spec:

```bash
bun run generate:{algod_api}:{py|ts}
```

The generated API clients will be available in the `./packages/` directory:

- `./packages/typescript/algod_api/` - algod TypeScript client
- `./packages/python/algod_api/` - algod Python client

## OpenAPI Specs for algorand apis

## Algod

The `algod.oas2.json` is taken directly from [go-algorand](https://github.com/algorand/go-algorand/blob/master/daemon/algod/api/algod.oas2.json). The script under [scripts/convert-openapi.ts](scripts/convert-openapi.ts) is used to convert the spec to OpenAPI 3.0 via [swagger converter](https://converter.swagger.io/) endpoint. The current approach is to manually edit and tweak the algod.oas2.json fixing known issues on a spec from go-algorand, then use the openapi-generator-cli to generate clients on the v3 spec. OpenAPI v3 is preferred for client generation as it offers enhanced schema features, better component reusability, and improved type definitions compared to v2. Additionally, most modern code generators like openapi-generator-cli provide better support and more accurate code generation when working with v3 specifications.

## OpenAPI Generator Configuration

The client generation is configured with the following options:

### TypeScript Client

- Package name: `@algorandfoundation/algokit-algod-api`
- ES6 support: true
- Manually refined tsconfig setup to build cjs, esm clients along with browser support.
- Custom tests defined in `oas_templates/typescript/custom-tests/` that implement tests for initial batch of transaction endpoints. More endpoint tests are to be added in the future.

### Python Client

- Package name: `algokit_algod_api`.
- Ignoring various unneeded supporting files like tox.ini, git_push.sh, etc.
- Various improvements to make auto generated code compatible with poetry and more modern python conventions and practices.
- Custom tests defined in `oas_templates/python/custom-tests/` that implement tests for initial batch of transaction endpoints. More endpoint tests are to be added in the future.
