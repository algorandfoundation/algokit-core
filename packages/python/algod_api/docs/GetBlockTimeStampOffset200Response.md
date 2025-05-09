# GetBlockTimeStampOffset200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**offset** | **int** | Timestamp offset in seconds. | 

## Example

```python
from algokit_algod_api.models.get_block_time_stamp_offset200_response import GetBlockTimeStampOffset200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetBlockTimeStampOffset200Response from a JSON string
get_block_time_stamp_offset200_response_instance = GetBlockTimeStampOffset200Response.from_json(json)
# print the JSON string representation of the object
print(GetBlockTimeStampOffset200Response.to_json())

# convert the object into a dict
get_block_time_stamp_offset200_response_dict = get_block_time_stamp_offset200_response_instance.to_dict()
# create an instance of GetBlockTimeStampOffset200Response from a dict
get_block_time_stamp_offset200_response_from_dict = GetBlockTimeStampOffset200Response.from_dict(get_block_time_stamp_offset200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


