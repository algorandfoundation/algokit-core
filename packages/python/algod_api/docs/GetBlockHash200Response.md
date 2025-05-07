# GetBlockHash200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_hash** | **str** | Block header hash. | 

## Example

```python
from algokit_algod_api.models.get_block_hash200_response import GetBlockHash200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetBlockHash200Response from a JSON string
get_block_hash200_response_instance = GetBlockHash200Response.from_json(json)
# print the JSON string representation of the object
print(GetBlockHash200Response.to_json())

# convert the object into a dict
get_block_hash200_response_dict = get_block_hash200_response_instance.to_dict()
# create an instance of GetBlockHash200Response from a dict
get_block_hash200_response_from_dict = GetBlockHash200Response.from_dict(get_block_hash200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


