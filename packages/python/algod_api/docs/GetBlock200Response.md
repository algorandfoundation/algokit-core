# GetBlock200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block** | **object** | Block header data. | 
**cert** | **object** | Optional certificate object. This is only included when the format is set to message pack. | [optional] 

## Example

```python
from algokit_algod_api.models.get_block200_response import GetBlock200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetBlock200Response from a JSON string
get_block200_response_instance = GetBlock200Response.from_json(json)
# print the JSON string representation of the object
print(GetBlock200Response.to_json())

# convert the object into a dict
get_block200_response_dict = get_block200_response_instance.to_dict()
# create an instance of GetBlock200Response from a dict
get_block200_response_from_dict = GetBlock200Response.from_dict(get_block200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


