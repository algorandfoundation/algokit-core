# AlgoKit Core TypeScript Packages

This directory contains the TypeScript packages for AlgoKit Core, providing JavaScript/TypeScript developers with comprehensive tools for building on the Algorand blockchain.

## Packages

This workspace includes the following packages:

- **`algokit_common`** - Common utilities and types shared across AlgoKit packages
- **`algokit_abi`** - ABI encoding/decoding utilities for Algorand smart contracts
- **`algokit_transact`** - Transaction building and signing utilities
- **`algokit_utils`** - General-purpose utilities for Algorand development
- **`algod_client`** - TypeScript client for Algorand node (algod) API
- **`indexer_client`** - TypeScript client for Algorand indexer API

## Contributing

### Prerequisites

- Node.js 20+
- npm (comes with Node.js)

### Development Setup

1. **Install dependencies**:

   ```bash
   npm install
   ```

2. **Build all packages**:

   ```bash
   npm run build
   ```

3. **Run tests**:

   ```bash
   npm run test
   ```

4. **Development with watch mode**:

This command is helpful when making changes across packages, as it automatically ensures updates are available to dependant packages.

   ```bash
   npm run build-watch
   ```

### Development Workflow

1. **Start watch mode** to ensure changes are built and available to all dependant packages: `npm run build-watch`
2. **Make changes** to the relevant package(s) in their `src/` directories
3. **Test** your changes: `npm run test`
4. **Run pre-commit checks**: `npm run pre-commit`
