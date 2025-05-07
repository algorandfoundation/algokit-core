# ApplicationInitialStates

An application's initial global/local/box states that were accessed during simulation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **int** | Application index. | 
**app_locals** | [**List[ApplicationKVStorage]**](ApplicationKVStorage.md) | An application&#39;s initial local states tied to different accounts. | [optional] 
**app_globals** | [**ApplicationKVStorage**](ApplicationKVStorage.md) |  | [optional] 
**app_boxes** | [**ApplicationKVStorage**](ApplicationKVStorage.md) |  | [optional] 

## Example

```python
from algokit_algod_api.models.application_initial_states import ApplicationInitialStates

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationInitialStates from a JSON string
application_initial_states_instance = ApplicationInitialStates.from_json(json)
# print the JSON string representation of the object
print(ApplicationInitialStates.to_json())

# convert the object into a dict
application_initial_states_dict = application_initial_states_instance.to_dict()
# create an instance of ApplicationInitialStates from a dict
application_initial_states_from_dict = ApplicationInitialStates.from_dict(application_initial_states_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


