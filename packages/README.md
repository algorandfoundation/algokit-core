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
