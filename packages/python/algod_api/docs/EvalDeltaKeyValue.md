# EvalDeltaKeyValue

Key-value pairs for StateDelta.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** |  | 
**value** | [**EvalDelta**](EvalDelta.md) |  | 

## Example

```python
from algokit_algod_api.models.eval_delta_key_value import EvalDeltaKeyValue

# TODO update the JSON string below
json = "{}"
# create an instance of EvalDeltaKeyValue from a JSON string
eval_delta_key_value_instance = EvalDeltaKeyValue.from_json(json)
# print the JSON string representation of the object
print(EvalDeltaKeyValue.to_json())

# convert the object into a dict
eval_delta_key_value_dict = eval_delta_key_value_instance.to_dict()
# create an instance of EvalDeltaKeyValue from a dict
eval_delta_key_value_from_dict = EvalDeltaKeyValue.from_dict(eval_delta_key_value_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


