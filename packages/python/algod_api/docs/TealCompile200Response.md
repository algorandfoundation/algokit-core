# TealCompile200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **str** | base32 SHA512_256 of program bytes (Address style) | 
**result** | **str** | base64 encoded program bytes | 
**sourcemap** | **object** | JSON of the source map | [optional] 

## Example

```python
from algokit_algod_api.models.teal_compile200_response import TealCompile200Response

# TODO update the JSON string below
json = "{}"
# create an instance of TealCompile200Response from a JSON string
teal_compile200_response_instance = TealCompile200Response.from_json(json)
# print the JSON string representation of the object
print(TealCompile200Response.to_json())

# convert the object into a dict
teal_compile200_response_dict = teal_compile200_response_instance.to_dict()
# create an instance of TealCompile200Response from a dict
teal_compile200_response_from_dict = TealCompile200Response.from_dict(teal_compile200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


