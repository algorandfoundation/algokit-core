# GetSyncRound200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**round** | **int** | The minimum sync round for the ledger. | 

## Example

```python
from algokit_algod_api.models.get_sync_round200_response import GetSyncRound200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetSyncRound200Response from a JSON string
get_sync_round200_response_instance = GetSyncRound200Response.from_json(json)
# print the JSON string representation of the object
print(GetSyncRound200Response.to_json())

# convert the object into a dict
get_sync_round200_response_dict = get_sync_round200_response_instance.to_dict()
# create an instance of GetSyncRound200Response from a dict
get_sync_round200_response_from_dict = GetSyncRound200Response.from_dict(get_sync_round200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


