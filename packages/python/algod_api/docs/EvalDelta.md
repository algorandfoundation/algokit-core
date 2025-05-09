# EvalDelta

Represents a TEAL value delta.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **int** | \\[at\\] delta action. | 
**bytes** | **str** | \\[bs\\] bytes value. | [optional] 
**uint** | **int** | \\[ui\\] uint value. | [optional] 

## Example

```python
from algokit_algod_api.models.eval_delta import EvalDelta

# TODO update the JSON string below
json = "{}"
# create an instance of EvalDelta from a JSON string
eval_delta_instance = EvalDelta.from_json(json)
# print the JSON string representation of the object
print(EvalDelta.to_json())

# convert the object into a dict
eval_delta_dict = eval_delta_instance.to_dict()
# create an instance of EvalDelta from a dict
eval_delta_from_dict = EvalDelta.from_dict(eval_delta_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


