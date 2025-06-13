# Genesis


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alloc** | [**List[GenesisAllocation]**](GenesisAllocation.md) |  | 
**comment** | **str** |  | [optional] 
**devmode** | **bool** |  | [optional] 
**fees** | **str** |  | 
**id** | **str** |  | 
**network** | **str** |  | 
**proto** | **str** |  | 
**rwd** | **str** |  | 
**timestamp** | **int** |  | 

## Example

```python
from algokit_algod_api.models.genesis import Genesis

# TODO update the JSON string below
json = "{}"
# create an instance of Genesis from a JSON string
genesis_instance = Genesis.from_json(json)
# print the JSON string representation of the object
print(Genesis.to_json())

# convert the object into a dict
genesis_dict = genesis_instance.to_dict()
# create an instance of Genesis from a dict
genesis_from_dict = Genesis.from_dict(genesis_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


