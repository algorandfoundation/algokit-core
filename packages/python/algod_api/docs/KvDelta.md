# KvDelta

A single Delta containing the key, the previous value and the current value for a single round.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **bytearray** | The key, base64 encoded. | [optional] 
**value** | **bytearray** | The new value of the KV store entry, base64 encoded. | [optional] 

## Example

```python
from algokit_algod_api.models.kv_delta import KvDelta

# TODO update the JSON string below
json = "{}"
# create an instance of KvDelta from a JSON string
kv_delta_instance = KvDelta.from_json(json)
# print the JSON string representation of the object
print(KvDelta.to_json())

# convert the object into a dict
kv_delta_dict = kv_delta_instance.to_dict()
# create an instance of KvDelta from a dict
kv_delta_from_dict = KvDelta.from_dict(kv_delta_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


