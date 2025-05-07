# AccountAssetsInformation200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**round** | **int** | The round for which this information is relevant. | 
**next_token** | **str** | Used for pagination, when making another request provide this token with the next parameter. | [optional] 
**asset_holdings** | [**List[AccountAssetHolding]**](AccountAssetHolding.md) |  | [optional] 

## Example

```python
from algokit_algod_api.models.account_assets_information200_response import AccountAssetsInformation200Response

# TODO update the JSON string below
json = "{}"
# create an instance of AccountAssetsInformation200Response from a JSON string
account_assets_information200_response_instance = AccountAssetsInformation200Response.from_json(json)
# print the JSON string representation of the object
print(AccountAssetsInformation200Response.to_json())

# convert the object into a dict
account_assets_information200_response_dict = account_assets_information200_response_instance.to_dict()
# create an instance of AccountAssetsInformation200Response from a dict
account_assets_information200_response_from_dict = AccountAssetsInformation200Response.from_dict(account_assets_information200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


