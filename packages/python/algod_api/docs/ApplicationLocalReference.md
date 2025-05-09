# ApplicationLocalReference

References an account's local state for an application.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | **str** | Address of the account with the local state. | 
**app** | **int** | Application ID of the local state application. | 

## Example

```python
from algokit_algod_api.models.application_local_reference import ApplicationLocalReference

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationLocalReference from a JSON string
application_local_reference_instance = ApplicationLocalReference.from_json(json)
# print the JSON string representation of the object
print(ApplicationLocalReference.to_json())

# convert the object into a dict
application_local_reference_dict = application_local_reference_instance.to_dict()
# create an instance of ApplicationLocalReference from a dict
application_local_reference_from_dict = ApplicationLocalReference.from_dict(application_local_reference_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


