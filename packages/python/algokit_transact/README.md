# AlgoKit Transact Python

This library provides the core primitives for transaction management, including: creation, grouping, fee calculation, signature attachment, and encoding. Once the transactions have been formed, they can be sent to the network using your chosen algod HTTP client.

## Installation

> [!NOTE] This library is not yet published to PyPI.

You can install the package by [following these instructions](../../README.md#python).

## Key Management

This library doesn't contain any abstractions for keypair creation or transaction signing because Algorand uses standard ED25519 key pairs. As a result, you can use any cryptographic library that supports ED25519 alongside this library.

In the below examples, we use `PyNaCl`. Using this library, you can create a keypair and obtain the Algorand address like below:

```python
from nacl.signing import SigningKey
from algokit_transact import address_from_pub_key

# Generate a new signing key
my_signing_key = SigningKey.generate()

# Get the public key and Algorand address
my_public_key = my_signing_key.verify_key.__bytes__()
my_algorand_address = address_from_pub_key(my_public_key)
```

## Examples

Below is a collection of examples that'll help you formulate transactions that can be sent to the network.

### Create a Payment

```python
import base64
from nacl.signing import SigningKey
from algokit_transact import (
    account_from_address,
    account_from_pub_key,
    assign_fee,
    FeeParams,
    Transaction,
    TransactionType,
    SignedTransaction,
    PaymentTransactionFields,
    encode_transaction,
    encode_signed_transaction,
)

# Get the sender and reciever addresses
alice_keypair = SigningKey.generate()
alice = account_from_pub_key(alice_keypair.verify_key.__bytes__())
bob = account_from_address("B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y")

# Build the base payment transaction
base_tx = Transaction(
    transaction_type=TransactionType.PAYMENT,
    sender=alice.address,
    first_valid=1337,
    last_valid=1347,
    genesis_hash=base64.b64decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI="),
    genesis_id="testnet-v1.0",
    payment=PaymentTransactionFields(amount=1337, receiver=bob.address),
)

tx = assign_fee(base_tx, FeeParams(fee_per_byte=0, min_fee=1000, max_fee=2000))

# Encode the transaction for signing
encoded_tx = encode_transaction(tx)

# Sign the transaction using PyNaCl
tx_sig = alice_keypair.sign(encoded_tx).signature

# Create an encoded signed transaction ready for sending to the algod api
signed_tx = SignedTransaction(
    transaction=tx,
    signature=tx_sig,
)
encoded_signed_tx = encode_signed_transaction(signed_tx)
```

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

> NOTE: You have to clear out the maturin build target directory if you are rebuilding the release wheel.
