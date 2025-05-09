# AccountAssetHolding

AccountAssetHolding describes the account's asset holding and asset parameters (if either exist) for a specific asset ID.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_holding** | [**AssetHolding**](AssetHolding.md) |  | 
**asset_params** | [**AssetParams**](AssetParams.md) |  | [optional] 

## Example

```python
from algokit_algod_api.models.account_asset_holding import AccountAssetHolding

# TODO update the JSON string below
json = "{}"
# create an instance of AccountAssetHolding from a JSON string
account_asset_holding_instance = AccountAssetHolding.from_json(json)
# print the JSON string representation of the object
print(AccountAssetHolding.to_json())

# convert the object into a dict
account_asset_holding_dict = account_asset_holding_instance.to_dict()
# create an instance of AccountAssetHolding from a dict
account_asset_holding_from_dict = AccountAssetHolding.from_dict(account_asset_holding_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


