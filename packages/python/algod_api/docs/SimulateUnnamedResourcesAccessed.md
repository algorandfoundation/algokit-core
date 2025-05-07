# SimulateUnnamedResourcesAccessed

These are resources that were accessed by this group that would normally have caused failure, but were allowed in simulation. Depending on where this object is in the response, the unnamed resources it contains may or may not qualify for group resource sharing. If this is a field in SimulateTransactionGroupResult, the resources do qualify, but if this is a field in SimulateTransactionResult, they do not qualify. In order to make this group valid for actual submission, resources that qualify for group sharing can be made available by any transaction of the group; otherwise, resources must be placed in the same transaction which accessed them.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | **List[str]** | The unnamed accounts that were referenced. The order of this array is arbitrary. | [optional] 
**assets** | **List[int]** | The unnamed assets that were referenced. The order of this array is arbitrary. | [optional] 
**apps** | **List[int]** | The unnamed applications that were referenced. The order of this array is arbitrary. | [optional] 
**boxes** | [**List[BoxReference]**](BoxReference.md) | The unnamed boxes that were referenced. The order of this array is arbitrary. | [optional] 
**extra_box_refs** | **int** | The number of extra box references used to increase the IO budget. This is in addition to the references defined in the input transaction group and any referenced to unnamed boxes. | [optional] 
**asset_holdings** | [**List[AssetHoldingReference]**](AssetHoldingReference.md) | The unnamed asset holdings that were referenced. The order of this array is arbitrary. | [optional] 
**app_locals** | [**List[ApplicationLocalReference]**](ApplicationLocalReference.md) | The unnamed application local states that were referenced. The order of this array is arbitrary. | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_unnamed_resources_accessed import SimulateUnnamedResourcesAccessed

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateUnnamedResourcesAccessed from a JSON string
simulate_unnamed_resources_accessed_instance = SimulateUnnamedResourcesAccessed.from_json(json)
# print the JSON string representation of the object
print(SimulateUnnamedResourcesAccessed.to_json())

# convert the object into a dict
simulate_unnamed_resources_accessed_dict = simulate_unnamed_resources_accessed_instance.to_dict()
# create an instance of SimulateUnnamedResourcesAccessed from a dict
simulate_unnamed_resources_accessed_from_dict = SimulateUnnamedResourcesAccessed.from_dict(simulate_unnamed_resources_accessed_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


