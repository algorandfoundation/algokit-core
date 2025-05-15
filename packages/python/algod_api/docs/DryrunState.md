# DryrunState

Stores the TEAL eval step data

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line** | **int** | Line number | 
**pc** | **int** | Program counter | 
**stack** | [**List[TealValue]**](TealValue.md) |  | 
**scratch** | [**List[TealValue]**](TealValue.md) |  | [optional] 
**error** | **str** | Evaluation error if any | [optional] 

## Example

```python
from algokit_algod_api.models.dryrun_state import DryrunState

# TODO update the JSON string below
json = "{}"
# create an instance of DryrunState from a JSON string
dryrun_state_instance = DryrunState.from_json(json)
# print the JSON string representation of the object
print(DryrunState.to_json())

# convert the object into a dict
dryrun_state_dict = dryrun_state_instance.to_dict()
# create an instance of DryrunState from a dict
dryrun_state_from_dict = DryrunState.from_dict(dryrun_state_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


