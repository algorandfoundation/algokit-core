# RawTransaction200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_id** | **str** | encoding of the transaction hash. | 

## Example

```python
from algokit_algod_api.models.raw_transaction200_response import RawTransaction200Response

# TODO update the JSON string below
json = "{}"
# create an instance of RawTransaction200Response from a JSON string
raw_transaction200_response_instance = RawTransaction200Response.from_json(json)
# print the JSON string representation of the object
print(RawTransaction200Response.to_json())

# convert the object into a dict
raw_transaction200_response_dict = raw_transaction200_response_instance.to_dict()
# create an instance of RawTransaction200Response from a dict
raw_transaction200_response_from_dict = RawTransaction200Response.from_dict(raw_transaction200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


