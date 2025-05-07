# AbortCatchup200Response

An catchpoint abort response.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catchup_message** | **str** | Catchup abort response string | 

## Example

```python
from algokit_algod_api.models.abort_catchup200_response import AbortCatchup200Response

# TODO update the JSON string below
json = "{}"
# create an instance of AbortCatchup200Response from a JSON string
abort_catchup200_response_instance = AbortCatchup200Response.from_json(json)
# print the JSON string representation of the object
print(AbortCatchup200Response.to_json())

# convert the object into a dict
abort_catchup200_response_dict = abort_catchup200_response_instance.to_dict()
# create an instance of AbortCatchup200Response from a dict
abort_catchup200_response_from_dict = AbortCatchup200Response.from_dict(abort_catchup200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


