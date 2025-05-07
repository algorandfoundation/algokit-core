# AccountApplicationInformation200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**round** | **int** | The round for which this information is relevant. | 
**app_local_state** | [**ApplicationLocalState**](ApplicationLocalState.md) |  | [optional] 
**created_app** | [**ApplicationParams**](ApplicationParams.md) |  | [optional] 

## Example

```python
from algokit_algod_api.models.account_application_information200_response import AccountApplicationInformation200Response

# TODO update the JSON string below
json = "{}"
# create an instance of AccountApplicationInformation200Response from a JSON string
account_application_information200_response_instance = AccountApplicationInformation200Response.from_json(json)
# print the JSON string representation of the object
print(AccountApplicationInformation200Response.to_json())

# convert the object into a dict
account_application_information200_response_dict = account_application_information200_response_instance.to_dict()
# create an instance of AccountApplicationInformation200Response from a dict
account_application_information200_response_from_dict = AccountApplicationInformation200Response.from_dict(account_application_information200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


