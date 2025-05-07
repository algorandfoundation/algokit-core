# GetBlockTxids200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_txids** | **List[str]** | Block transaction IDs. | 

## Example

```python
from algokit_algod_api.models.get_block_txids200_response import GetBlockTxids200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetBlockTxids200Response from a JSON string
get_block_txids200_response_instance = GetBlockTxids200Response.from_json(json)
# print the JSON string representation of the object
print(GetBlockTxids200Response.to_json())

# convert the object into a dict
get_block_txids200_response_dict = get_block_txids200_response_instance.to_dict()
# create an instance of GetBlockTxids200Response from a dict
get_block_txids200_response_from_dict = GetBlockTxids200Response.from_dict(get_block_txids200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


