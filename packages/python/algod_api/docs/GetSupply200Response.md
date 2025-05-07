# GetSupply200Response

Supply represents the current supply of MicroAlgos in the system

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_round** | **int** | Round | 
**online_money** | **int** | OnlineMoney | 
**total_money** | **int** | TotalMoney | 

## Example

```python
from algokit_algod_api.models.get_supply200_response import GetSupply200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetSupply200Response from a JSON string
get_supply200_response_instance = GetSupply200Response.from_json(json)
# print the JSON string representation of the object
print(GetSupply200Response.to_json())

# convert the object into a dict
get_supply200_response_dict = get_supply200_response_instance.to_dict()
# create an instance of GetSupply200Response from a dict
get_supply200_response_from_dict = GetSupply200Response.from_dict(get_supply200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


