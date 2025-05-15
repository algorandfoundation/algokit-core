# AssetHolding

Describes an asset held by an account.  Definition: data/basics/userBalance.go : AssetHolding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **int** | \\[a\\] number of units held. | 
**asset_id** | **int** | Asset ID of the holding. | 
**is_frozen** | **bool** | \\[f\\] whether or not the holding is frozen. | 

## Example

```python
from algokit_algod_api.models.asset_holding import AssetHolding

# TODO update the JSON string below
json = "{}"
# create an instance of AssetHolding from a JSON string
asset_holding_instance = AssetHolding.from_json(json)
# print the JSON string representation of the object
print(AssetHolding.to_json())

# convert the object into a dict
asset_holding_dict = asset_holding_instance.to_dict()
# create an instance of AssetHolding from a dict
asset_holding_from_dict = AssetHolding.from_dict(asset_holding_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


