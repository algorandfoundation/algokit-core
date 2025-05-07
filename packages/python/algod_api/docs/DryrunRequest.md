# DryrunRequest

Request data type for dryrun endpoint. Given the Transactions and simulated ledger state upload, run TEAL scripts and return debugging information.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txns** | **List[str]** |  | 
**accounts** | [**List[Account]**](Account.md) |  | 
**apps** | [**List[Application]**](Application.md) |  | 
**protocol_version** | **str** | ProtocolVersion specifies a specific version string to operate under, otherwise whatever the current protocol of the network this algod is running in. | 
**round** | **int** | Round is available to some TEAL scripts. Defaults to the current round on the network this algod is attached to. | 
**latest_timestamp** | **int** | LatestTimestamp is available to some TEAL scripts. Defaults to the latest confirmed timestamp this algod is attached to. | 
**sources** | [**List[DryrunSource]**](DryrunSource.md) |  | 

## Example

```python
from algokit_algod_api.models.dryrun_request import DryrunRequest

# TODO update the JSON string below
json = "{}"
# create an instance of DryrunRequest from a JSON string
dryrun_request_instance = DryrunRequest.from_json(json)
# print the JSON string representation of the object
print(DryrunRequest.to_json())

# convert the object into a dict
dryrun_request_dict = dryrun_request_instance.to_dict()
# create an instance of DryrunRequest from a dict
dryrun_request_from_dict = DryrunRequest.from_dict(dryrun_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


