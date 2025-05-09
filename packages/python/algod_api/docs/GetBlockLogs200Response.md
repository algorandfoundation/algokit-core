# GetBlockLogs200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**logs** | [**List[AppCallLogs]**](AppCallLogs.md) |  | 

## Example

```python
from algokit_algod_api.models.get_block_logs200_response import GetBlockLogs200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetBlockLogs200Response from a JSON string
get_block_logs200_response_instance = GetBlockLogs200Response.from_json(json)
# print the JSON string representation of the object
print(GetBlockLogs200Response.to_json())

# convert the object into a dict
get_block_logs200_response_dict = get_block_logs200_response_instance.to_dict()
# create an instance of GetBlockLogs200Response from a dict
get_block_logs200_response_from_dict = GetBlockLogs200Response.from_dict(get_block_logs200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


