# TealDryrun200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txns** | [**List[DryrunTxnResult]**](DryrunTxnResult.md) |  | 
**error** | **str** |  | 
**protocol_version** | **str** | Protocol version is the protocol version Dryrun was operated under. | 

## Example

```python
from algokit_algod_api.models.teal_dryrun200_response import TealDryrun200Response

# TODO update the JSON string below
json = "{}"
# create an instance of TealDryrun200Response from a JSON string
teal_dryrun200_response_instance = TealDryrun200Response.from_json(json)
# print the JSON string representation of the object
print(TealDryrun200Response.to_json())

# convert the object into a dict
teal_dryrun200_response_dict = teal_dryrun200_response_instance.to_dict()
# create an instance of TealDryrun200Response from a dict
teal_dryrun200_response_from_dict = TealDryrun200Response.from_dict(teal_dryrun200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


