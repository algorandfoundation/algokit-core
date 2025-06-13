# GenesisAllocation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addr** | **str** |  | 
**comment** | **str** |  | 
**state** | [**GenesisAllocationState**](GenesisAllocationState.md) |  | 

## Example

```python
from algokit_algod_api.models.genesis_allocation import GenesisAllocation

# TODO update the JSON string below
json = "{}"
# create an instance of GenesisAllocation from a JSON string
genesis_allocation_instance = GenesisAllocation.from_json(json)
# print the JSON string representation of the object
print(GenesisAllocation.to_json())

# convert the object into a dict
genesis_allocation_dict = genesis_allocation_instance.to_dict()
# create an instance of GenesisAllocation from a dict
genesis_allocation_from_dict = GenesisAllocation.from_dict(genesis_allocation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


