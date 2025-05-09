# TealKeyValue

Represents a key-value pair in an application store.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** |  | 
**value** | [**TealValue**](TealValue.md) |  | 

## Example

```python
from algokit_algod_api.models.teal_key_value import TealKeyValue

# TODO update the JSON string below
json = "{}"
# create an instance of TealKeyValue from a JSON string
teal_key_value_instance = TealKeyValue.from_json(json)
# print the JSON string representation of the object
print(TealKeyValue.to_json())

# convert the object into a dict
teal_key_value_dict = teal_key_value_instance.to_dict()
# create an instance of TealKeyValue from a dict
teal_key_value_from_dict = TealKeyValue.from_dict(teal_key_value_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


