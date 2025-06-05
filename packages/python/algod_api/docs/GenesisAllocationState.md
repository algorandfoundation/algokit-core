# GenesisAllocationState


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algo** | **int** |  | 
**onl** | **int** |  | [optional] 
**sel** | **str** |  | [optional] 
**stprf** | **str** |  | [optional] 
**vote** | **str** |  | [optional] 
**vote_kd** | **int** |  | [optional] 
**vote_fst** | **int** |  | [optional] 
**vote_lst** | **int** |  | [optional] 

## Example

```python
from algokit_algod_api.models.genesis_allocation_state import GenesisAllocationState

# TODO update the JSON string below
json = "{}"
# create an instance of GenesisAllocationState from a JSON string
genesis_allocation_state_instance = GenesisAllocationState.from_json(json)
# print the JSON string representation of the object
print(GenesisAllocationState.to_json())

# convert the object into a dict
genesis_allocation_state_dict = genesis_allocation_state_instance.to_dict()
# create an instance of GenesisAllocationState from a dict
genesis_allocation_state_from_dict = GenesisAllocationState.from_dict(genesis_allocation_state_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


