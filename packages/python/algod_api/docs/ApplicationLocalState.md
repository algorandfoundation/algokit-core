# ApplicationLocalState

Stores local state associated with an application.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **int** | The application which this local state is for. | 
**var_schema** | [**ApplicationStateSchema**](ApplicationStateSchema.md) |  | 
**key_value** | [**List[TealKeyValue]**](TealKeyValue.md) | Represents a key-value store for use in an application. | [optional] 

## Example

```python
from algokit_algod_api.models.application_local_state import ApplicationLocalState

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationLocalState from a JSON string
application_local_state_instance = ApplicationLocalState.from_json(json)
# print the JSON string representation of the object
print(ApplicationLocalState.to_json())

# convert the object into a dict
application_local_state_dict = application_local_state_instance.to_dict()
# create an instance of ApplicationLocalState from a dict
application_local_state_from_dict = ApplicationLocalState.from_dict(application_local_state_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


