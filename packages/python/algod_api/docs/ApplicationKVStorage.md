# ApplicationKVStorage

An application's global/local/box state.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kvs** | [**List[AvmKeyValue]**](AvmKeyValue.md) | Key-Value pairs representing application states. | 
**account** | **str** | The address of the account associated with the local state. | [optional] 

## Example

```python
from algokit_algod_api.models.application_kv_storage import ApplicationKVStorage

# TODO update the JSON string below
json = "{}"
# create an instance of ApplicationKVStorage from a JSON string
application_kv_storage_instance = ApplicationKVStorage.from_json(json)
# print the JSON string representation of the object
print(ApplicationKVStorage.to_json())

# convert the object into a dict
application_kv_storage_dict = application_kv_storage_instance.to_dict()
# create an instance of ApplicationKVStorage from a dict
application_kv_storage_from_dict = ApplicationKVStorage.from_dict(application_kv_storage_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


