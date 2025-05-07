# OpenAPI Generators for Algorand APIs

## Overview

The `/api` directory is central codebase for generating API clients for Algorand APIs across multiple languages. This document outlines the architecture, structure, and workflows implemented in this directory.

## OpenAPI Generator Approach

[OpenAPI generators](https://openapi-generator.tech/) was chosen to automate the creation of API clients in various languages (like TypeScript, Python, and Rust) from a single OpenAPI specification. This approach guarantees consistency across different language clients and significantly reduces the effort needed for maintenance. Instead of manually updating clients for each language, changes made to the API specification automatically propagate to all generated clients. The generator tool supports a wide range of languages and is a mature, actively maintained open-source project.

## Structure

The `/api` directory is organized as follows:

- `specs/`: Contains OpenAPI specifications (both v2 and v3 formats).
- `api_clients/`: Holds the generated client code for different languages.
- `oas_templates/`: Stores custom Mustache templates used for code generation.
- `scripts/`: Includes utility scripts for generation and maintenance tasks.
- Configuration files:
  - `openapitools.json`: Configures the OpenAPI generator.
  - `package.json`: Manages NPM dependencies and scripts.
  - `tsconfig.json`: Contains TypeScript configuration settings.

## OpenAPI v3 vs v2

We primarily use OpenAPI v3 specifications for client generation because it offers a more generator-friendly experience.

The original algod oas spec from go algorand repository was in v2, but it also provides an auto-converted v3 specification in YAML format. We opted for JSON for consistency with the original v2 spec's format. To minimize duplication, only the original v2 spec was copied into the `api/specs/` directory; the v3 JSON spec is automatically generated using the same tool leveraged in the Go Algorand repository, ensuring we stay aligned while using our preferred format.

## Improvements Over Original Specification

Modifications to the original specification have mainly focused on improving documentation clarity, formatting consistency, and ensuring compatibility with the generator tools, rather than making significant changes to the API itself. Minor known issues, like problems with non-ASCII characters in JSON responses on certain endpoints, by further refining the spec as we develop test suites for each generated client language.

Notable changes include:

- Adding consistent formatting and filling in missing descriptions.
- Removing `format: 'byte'` from the `AvmValue` schema. This change prevents an overly complex type definition in the generated Python client that previously caused recursion errors when instantiating the auto-generated `pydantic` v2 base model.

## Client Implementation

Currently, the auto-generated clients cover the transaction endpoints and leverage:

- **algokit-utils-ts** and **algokit-utils-py**: Used within the TypeScript and Python test suites.
- **FFI binding packages**: Utilized as test utilities. Transactions are constructed using the FFI bindings packages.

Our initial implementation work focuses on core endpoints critical for basic integration in the algokit-utils packages:

- Retrieving pending transactions
- Fetching transaction parameters
- Posting raw transactions

This focused approach allows us to build a baseline before expanding coverage to more complex endpoints.

## Testing Approach

Custom tests were written instead of modifying the auto-generated ones for several reasons:

1. **Setup Complexity**: Many endpoints require intricate setup and teardown procedures that are hard to automate generically.
2. **Environment Dependencies**: Tests often rely on specific environment configurations (like Sandbox).
3. **Edge Case Coverage**: Custom tests provide better flexibility for covering specific edge cases and scenarios.
4. **Maintainability**: Following the OpenAPI generator's recommendation, extending functionality via custom tests is more maintainable than altering the generated test code directly.

This approach aligns with best practices for using OpenAPI generators, emphasizing extension over modification.

## Template System

The OpenAPI generator uses Mustache templates for code generation. We utilize custom templates stored in the `oas_templates/` directory to:

1. Extend the base templates provided by the OpenAPI generator.
2. Tailor the generated code to better suit our project's specific needs and conventions.
3. Inject additional utilities and helper functions where needed.
4. Implement language-specific optimizations or adjustments.

### Template Lookup Order

OpenAPI Generator looks for templates in the following order (simplified from the [oas generator docs](https://openapi-generator.tech/docs/templating#retrieving-templates)):

```ascii
+--------------------------+ (api/oas_templates/{lang}/)
| 1. User Lib Path         | (custom/.../libraries/...)
+------------+-------------+
              | (Not found fetch from next level up)
              v
+--------------------------+
| 2. Base Template content
| and other embedded files | (base template files in oas generators)
+------------+-------------+
```

Which means that if custom template that was extended from the base does not contain files that exist in base template, the generator will look in the next level up. To explicitly prevent this from happening, we can use the `.openapi-generator-ignore` files are placed in generation output directory prior to invocation of the generator. This logic is handled inside `scripts/generate-api-clients.ts` script.

## Configuration and Ignore Files

The core configuration for the OpenAPI generator resides in `openapitools.json`. This file defines:

- Generator versions to use.

Actual invocation of the generator is done via `scripts/generate-api-clients.ts` which is a wrapper around the generator tool that is invoked via `bun run`.

Additionally, `.openapi-generator-ignore` within each template folder are used to control the regeneration process, allowing us to:

- Protect manually customized files from being overwritten during generation runs.
- Skip the generation of files that are not needed.
- Maintain custom implementations alongside the automatically generated code.

Lastly, each template folder defined `openapi-config.yaml` file to control the generation process. This allows us to inject custom files that are not part of base templates as well as control the templatised properties offered by each language generator.

## Future Work

1. **Expanded Endpoint Coverage and integration into algokit-utils**: Develop custom Mustache test templates for each target language to ensure robust testing across more API endpoints. As new categories of endpoints are covered, we can perform integration into algokit-utils in parallel.
2. **CI/CD Integration & Versioning**: Implement a proper CI/CD pipeline for automated client generation and introduce semantic versioning. This includes integrating the generated clients into version control and adding output stability tests to track changes easily when specifications are updated.
3. **Monorepo Management**: Explore tools like Bazel or NX to improve the management of our multi-language codebase, potentially offering benefits like incremental builds and better dependency handling.
4. **Documentation Generation**: Automate the generation of client-specific documentation. The default generator already creates basic Markdown files, but we aim to enhance this.

### Furthe notes

Adopting a monorepo structure with tools like Bazel or NX could offer advantages such as:

- Faster, incremental builds.
- Coordinated versioning across packages.
- Simplified cross-language dependency management.
- Consistent development tooling.
