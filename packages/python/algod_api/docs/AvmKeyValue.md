# AvmKeyValue

Represents an AVM key-value pair in an application store.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **bytearray** |  | 
**value** | [**AvmValue**](AvmValue.md) |  | 

## Example

```python
from algokit_algod_api.models.avm_key_value import AvmKeyValue

# TODO update the JSON string below
json = "{}"
# create an instance of AvmKeyValue from a JSON string
avm_key_value_instance = AvmKeyValue.from_json(json)
# print the JSON string representation of the object
print(AvmKeyValue.to_json())

# convert the object into a dict
avm_key_value_dict = avm_key_value_instance.to_dict()
# create an instance of AvmKeyValue from a dict
avm_key_value_from_dict = AvmKeyValue.from_dict(avm_key_value_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


