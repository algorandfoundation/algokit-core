# AssetHoldingReference

References an asset held by an account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | **str** | Address of the account holding the asset. | 
**asset** | **int** | Asset ID of the holding. | 

## Example

```python
from algokit_algod_api.models.asset_holding_reference import AssetHoldingReference

# TODO update the JSON string below
json = "{}"
# create an instance of AssetHoldingReference from a JSON string
asset_holding_reference_instance = AssetHoldingReference.from_json(json)
# print the JSON string representation of the object
print(AssetHoldingReference.to_json())

# convert the object into a dict
asset_holding_reference_dict = asset_holding_reference_instance.to_dict()
# create an instance of AssetHoldingReference from a dict
asset_holding_reference_from_dict = AssetHoldingReference.from_dict(asset_holding_reference_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


