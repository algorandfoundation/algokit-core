# Language Specific Packages

This section of the monorepo contains the language specific published packages which may or may not be built from shared rust code.

## Installation

Whilst we are building out the capabilities, we will likely be making a lot of breaking changes.
As such we will only be releasing alpha versions of the packages for integration with internal tooling.
At this point anything is open to change, so because of that we'll only publish these packages to internal package sources.

See below for the installation instructions for each supported language:

### Python

The Python packages are published as wheels on the GitHub release. These can be installed directly.

For a binary wheel with multiple target platforms like [algokit_transact](https://github.com/algorandfoundation/algokit-core/releases/tag/python%2Falgokit_transact%401.0.0-alpha.1), you can use the below config.

```toml
[tool.poetry.dependencies]
algokit_transact = [
    { url = "https://github.com/algorandfoundation/algokit-core/releases/download/python%2Falgokit_transact%401.0.0-alpha.1/algokit_transact-1.0.0a1-py3-none-macosx_10_12_x86_64.whl", markers = "sys_platform == 'darwin' and platform_machine == 'x86_64'" },
    { url = "https://github.com/algorandfoundation/algokit-core/releases/download/python%2Falgokit_transact%401.0.0-alpha.1/algokit_transact-1.0.0a1-py3-none-macosx_11_0_arm64.whl", markers = "sys_platform == 'darwin' and platform_machine == 'arm64'" },
    { url = "https://github.com/algorandfoundation/algokit-core/releases/download/python%2Falgokit_transact%401.0.0-alpha.1/algokit_transact-1.0.0a1-py3-none-manylinux_2_17_x86_64.manylinux2014_x86_64.whl", markers = "sys_platform == 'linux' and platform_machine == 'x86_64'" },
    { url = "https://github.com/algorandfoundation/algokit-core/releases/download/python%2Falgokit_transact%401.0.0-alpha.1/algokit_transact-1.0.0a1-py3-none-manylinux_2_17_aarch64.manylinux2014_aarch64.whl", markers = "sys_platform == 'linux' and platform_machine == 'aarch64'" },
    { url = "https://github.com/algorandfoundation/algokit-core/releases/download/python%2Falgokit_transact%401.0.0-alpha.1/algokit_transact-1.0.0a1-py3-none-win_amd64.whl", markers = "sys_platform == 'win32' and platform_machine == 'AMD64'" }
]
```

For a non binary wheel like [algod_api](https://github.com/algorandfoundation/algokit-core/releases/tag/python%2Falgod_api%401.0.0-alpha.2), you can use the below config.

```toml
[tool.poetry.dependencies]
algokit_algod_api = { url = "https://github.com/algorandfoundation/algokit-core/releases/download/python%2Falgod_api%401.0.0-alpha.2/algokit_algod_api-1.0.0a2-py3-none-any.whl" }
```

### TypeScript

The TypeScript packages are published to the GitHub packages NPM repository.

For example [algokit_transact](https://github.com/algorandfoundation/algokit-core/pkgs/npm/algokit-transact).

GitHub packages (the product) has some constaints, namely:

- It requires authentication to install a package
- It enforce packages scopes, which must match the GitHub username (`algorandfoundation` in our case).

Unfortunately for us, the package scope in GitHub packages matches the official NPM registry and NPM itself does not support configuring multiple registries for a single package scope, so we need to be a little crafty when installing/updating the alpha packages we're installing from GitHub packages.

To install, follow the below:

1. Create a new [personal access token](https://github.com/settings/tokens/new) with `read:packages` permission and your preference of expiration time.
2. Add the following to `~/.npmrc`:

   ```
   @algorandfoundation:registry=https://npm.pkg.github.com
   //npm.pkg.github.com/:_authToken={TOKEN}
   ```

3. Install the package hosted in GitHub packages, which will update your NPM package lock file.

   ```
   npm install @algorandfoundation/algokit-transact@1.0.0-alpha.4
   ```

4. Remove `@algorandfoundation:registry=https://npm.pkg.github.com` from `~/.npmrc`, so NPM hosted `@algorandfoundation` packages can be resolved.

Note: You will need to perform steps 2-4 each time your either install a package for the first time or update a GitHub package hosted dependency. Installing from the lock file only requires the token.

## Per-model DTO codecs (TypeScript clients)

The generated TypeScript API clients now emit per-model codecs alongside each TypeScript model type. For a given model `Foo` you will get:

- `type Foo`: domain model with camelCase keys and rich types (e.g. `Uint8Array`, `bigint`)
- `type FooDto`: JSON-serializable shape with wire keys (e.g. `kebab-case`) and JSON-safe primitives
- `Foo.toDto(value: Foo): FooDto` and `Foo.fromDto(dto: FooDto): Foo`
- `Foo.encodeMsgpack(value: Foo): Uint8Array` and `Foo.decodeMsgpack(bytes: Uint8Array): Foo`
- `Foo.encodeJson(value: Foo): unknown` and `Foo.decodeJson(raw: unknown): Foo`

Behavioral rules:

- Key rename: camelCase in `Foo` is mapped to wire keys in `FooDto`
- Bytes fields: represented as `Uint8Array` in `Foo`; JSON uses base64 strings; msgpack passes through bytes
- Big integers: represented as `bigint` in `Foo`; JSON encodes as strings and decodes back to `bigint`
- Nested refs/arrays: codecs recurse into child model codecs automatically
- Signed transactions: encoded/decoded using `@algorandfoundation/algokit-transact`; for JSON arrays (e.g. simulate) raw object forms are preserved when already in DTO shape

Services call these codecs automatically:

- Request bodies: JSON uses `Model.encodeJson`, msgpack uses `Model.encodeMsgpack` (or passes `Uint8Array` through)
- Responses: JSON uses `Model.decodeJson`, msgpack uses `Model.decodeMsgpack`

This replaces centralized transformer/rename maps; only small runtime helpers remain (e.g. `json.ts` base64 and `msgpack.ts`).
