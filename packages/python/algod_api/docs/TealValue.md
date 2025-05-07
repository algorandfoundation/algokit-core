# TealValue

Represents a TEAL value.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **int** | \\[tt\\] value type. Value &#x60;1&#x60; refers to **bytes**, value &#x60;2&#x60; refers to **uint** | 
**bytes** | **str** | \\[tb\\] bytes value. | 
**uint** | **int** | \\[ui\\] uint value. | 

## Example

```python
from algokit_algod_api.models.teal_value import TealValue

# TODO update the JSON string below
json = "{}"
# create an instance of TealValue from a JSON string
teal_value_instance = TealValue.from_json(json)
# print the JSON string representation of the object
print(TealValue.to_json())

# convert the object into a dict
teal_value_dict = teal_value_instance.to_dict()
# create an instance of TealValue from a dict
teal_value_from_dict = TealValue.from_dict(teal_value_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


