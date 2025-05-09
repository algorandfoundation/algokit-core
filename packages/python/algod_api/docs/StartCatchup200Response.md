# StartCatchup200Response

An catchpoint start response.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catchup_message** | **str** | Catchup start response string | 

## Example

```python
from algokit_algod_api.models.start_catchup200_response import StartCatchup200Response

# TODO update the JSON string below
json = "{}"
# create an instance of StartCatchup200Response from a JSON string
start_catchup200_response_instance = StartCatchup200Response.from_json(json)
# print the JSON string representation of the object
print(StartCatchup200Response.to_json())

# convert the object into a dict
start_catchup200_response_dict = start_catchup200_response_instance.to_dict()
# create an instance of StartCatchup200Response from a dict
start_catchup200_response_from_dict = StartCatchup200Response.from_dict(start_catchup200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


