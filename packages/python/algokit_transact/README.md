# AlgoKit Transact Python Bindings

Python bindings for the AlgoKit Transact Rust library.

## Installation

You can install the package directly from PyPI:

```bash
pip install algokit_transact
```

> [!NOTE] Not published to pypi yet

## Development Setup

This package uses [Poetry](https://python-poetry.org/) for dependency management and [Maturin](https://github.com/PyO3/maturin) for building and packaging the Rust extension.

1. Clone the repository:

```bash
git clone https://github.com/your-org/algorand-rust-ffis.git
cd algorand-rust-ffis/packages/python/algokit_transact
```

2. Create a virtual environment and install dependencies:

```bash
poetry install
```

3. Build and install the package in development mode:

```bash
poetry run maturin develop
```

## How It Works

This package uses [UniFFI](https://mozilla.github.io/uniffi-rs/) to generate Python bindings for the Rust core library. UniFFI generates the binding code using interface definition files, and Maturin handles the building and packaging of the Rust extension.

The project structure:

```
algokit_transact/         # Python package
  ├── __init__.py        # Main package imports
  ├── py.typed           # Marker file for type hints
  └── algokit_transact/  # Generated FFI code
      ├── __init__.py
      ├── algokit_transact_ffi.py
      └── libalgokit_transact.dylib
```

## Running Tests

```bash
poetry run pytest
```

## Building for Distribution

To build a wheel for distribution:

```bash
poetry run maturin build --release
```

Alternatively, you can build it via bun script at the root of the repository:

```bash
bun run scripts/build/index.ts algokit_transact python
```
