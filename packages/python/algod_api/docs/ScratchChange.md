# ScratchChange

A write operation into a scratch slot.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**slot** | **int** | The scratch slot written. | 
**new_value** | [**AvmValue**](AvmValue.md) |  | 

## Example

```python
from algokit_algod_api.models.scratch_change import ScratchChange

# TODO update the JSON string below
json = "{}"
# create an instance of ScratchChange from a JSON string
scratch_change_instance = ScratchChange.from_json(json)
# print the JSON string representation of the object
print(ScratchChange.to_json())

# convert the object into a dict
scratch_change_dict = scratch_change_instance.to_dict()
# create an instance of ScratchChange from a dict
scratch_change_from_dict = ScratchChange.from_dict(scratch_change_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


