# ApplicationStateOperation

An operation against an application's global/local/box state.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation** | **str** | Operation type. Value &#x60;w&#x60; is **write**, &#x60;d&#x60; is **delete**. | 
**app_state_type** | **str** | Type of application state. Value &#x60;g&#x60; is **global state**, &#x60;l&#x60; is **local state**, &#x60;b&#x60; is **boxes**. | 
**key** | **bytearray** | The key (name) of the global/local/box state. | 
**new_value** | [**AvmValue**](AvmValue.md) |  | [optional] 
**account** | **str** | For local state changes, the address of the account associated with the local state. | [optional] 

## Example

```python
from algokit_algod_api.models.application_state_operation import ApplicationStateOperation

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationStateOperation from a JSON string
application_state_operation_instance = ApplicationStateOperation.from_json(json)
# print the JSON string representation of the object
print(ApplicationStateOperation.to_json())

# convert the object into a dict
application_state_operation_dict = application_state_operation_instance.to_dict()
# create an instance of ApplicationStateOperation from a dict
application_state_operation_from_dict = ApplicationStateOperation.from_dict(application_state_operation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


