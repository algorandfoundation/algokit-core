# ApplicationStateSchema

Specifies maximums on the number of each type that may be stored.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num_uint** | **int** | \\[nui\\] num of uints. | 
**num_byte_slice** | **int** | \\[nbs\\] num of byte slices. | 

## Example

```python
from algokit_algod_api.models.application_state_schema import ApplicationStateSchema

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationStateSchema from a JSON string
application_state_schema_instance = ApplicationStateSchema.from_json(json)
# print the JSON string representation of the object
print(ApplicationStateSchema.to_json())

# convert the object into a dict
application_state_schema_dict = application_state_schema_instance.to_dict()
# create an instance of ApplicationStateSchema from a dict
application_state_schema_from_dict = ApplicationStateSchema.from_dict(application_state_schema_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


