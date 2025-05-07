# AvmValue

Represents an AVM value.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **int** | value type. Value &#x60;1&#x60; refers to **bytes**, value &#x60;2&#x60; refers to **uint64** | 
**bytes** | **str** | bytes value. | [optional] 
**uint** | **int** | uint value. | [optional] 

## Example

```python
from algokit_algod_api.models.avm_value import AvmValue

# TODO update the JSON string below
json = "{}"
# create an instance of AvmValue from a JSON string
avm_value_instance = AvmValue.from_json(json)
# print the JSON string representation of the object
print(AvmValue.to_json())

# convert the object into a dict
avm_value_dict = avm_value_instance.to_dict()
# create an instance of AvmValue from a dict
avm_value_from_dict = AvmValue.from_dict(avm_value_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


