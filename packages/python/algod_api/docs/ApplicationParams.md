# ApplicationParams

Stores the global information associated with an application.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creator** | **str** | The address that created this application. This is the address where the parameters and global state for this application can be found. | 
**approval_program** | **bytearray** | \\[approv\\] approval program. | 
**clear_state_program** | **bytearray** | \\[clearp\\] approval program. | 
**extra_program_pages** | **int** | \\[epp\\] the amount of extra program pages available to this app. | [optional] 
**local_state_schema** | [**ApplicationStateSchema**](ApplicationStateSchema.md) |  | [optional] 
**global_state_schema** | [**ApplicationStateSchema**](ApplicationStateSchema.md) |  | [optional] 
**global_state** | [**List[TealKeyValue]**](TealKeyValue.md) | Represents a key-value store for use in an application. | [optional] 
**version** | **int** | \\[v\\] the number of updates to the application programs | [optional] 

## Example

```python
from algokit_algod_api.models.application_params import ApplicationParams

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationParams from a JSON string
application_params_instance = ApplicationParams.from_json(json)
# print the JSON string representation of the object
print(ApplicationParams.to_json())

# convert the object into a dict
application_params_dict = application_params_instance.to_dict()
# create an instance of ApplicationParams from a dict
application_params_from_dict = ApplicationParams.from_dict(application_params_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


