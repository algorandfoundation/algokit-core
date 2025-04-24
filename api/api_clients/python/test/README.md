# Testing the algorand_algod_client Client

This directory contains tests for the algorand_algod_client client, which provides a Python interface to the Algorand API.

## Prerequisites

Before running tests, ensure you have:

1. A local Algorand node running (e.g., using [sandbox](https://github.com/algorand/sandbox))
2. The node should be accessible at `http://localhost:4001` with the token `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`

You can customize the node URL and token by setting environment variables:
- `ALGORAND_HOST` - The URL of your Algorand node (default: `http://localhost:4001`)
- `ALGORAND_API_TOKEN` - Your Algorand API token (default: `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`)

## Running Tests

To run the tests, use the following command from the root directory:

```bash
# Run all tests
pytest

# Run a specific test file
pytest test/test_public_api.py

# Run a specific test
pytest test/test_public_api.py::TestPublicApi::test_transaction_params
```

## Test Structure

The tests are organized as follows:

- `conftest.py` - Contains common fixtures and test setup
- `test_utils.py` - Utility functions for generating test data
- `data/` - Contains dummy data files used for testing
- `test_*_api.py` - Tests for API endpoints
- `test_model_*.py` - Tests for data models

## Converting from unittest to pytest

If you have existing tests written using unittest, you can convert them to pytest using the `pytestify.py` script:

```bash
python pytestify.py test/
```

Options:
- `--keep-method-casing` - Don't convert camelCase method names to snake_case
- `--with-count-equal` - Convert assertCountEqual to sorted comparison 
