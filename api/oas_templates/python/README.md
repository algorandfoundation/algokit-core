# Custom OpenAPI Generator Templates for Algorand Python Client

These templates customize the generated Python client to use the `algo_models` package instead of generating new models.

## Features

- Uses existing `algo_models` Python bindings instead of generating new models
- Removes Pydantic dependency
- Uses either uv or poetry for packaging (configurable)
- Removes GitLab CI configuration

## Usage

Run the generator script:

```bash
./scripts/generate_algod_client.sh
```

This will generate a Python client in `packages/python/algod_client`.

## Configuration

Edit `api/oas_templates/python/config.yaml` to:

1. Modify package details
2. Toggle between uv and poetry
3. Add import mappings for additional models

## Import Mappings

The important part of the configuration is correctly mapping OpenAPI schema models to your `algo_models` classes. Add all necessary models to the `importMappings` section in the config file.

Example:

```yaml
importMappings:
  Account: "algo_models.account.Account"
  Address: "algo_models.address.Address"
  Transaction: "algo_models.transaction.Transaction"
```

## Custom Templates

The key customized templates are:

- `model.mustache` - Modified to import from `algo_models` instead of generating models
- `model_generic.mustache` - Replaced with a stub
- `requirements.mustache` - Removed Pydantic dependency
- `pyproject.mustache` - Updated to use poetry/uv and include algo_models
