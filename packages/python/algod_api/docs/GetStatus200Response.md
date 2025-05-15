# GetStatus200Response

NodeStatus contains the information about a node status

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catchup_time** | **int** | CatchupTime in nanoseconds | 
**last_round** | **int** | LastRound indicates the last round seen | 
**last_version** | **str** | LastVersion indicates the last consensus version supported | 
**next_version** | **str** | NextVersion of consensus protocol to use | 
**next_version_round** | **int** | NextVersionRound is the round at which the next consensus version will apply | 
**next_version_supported** | **bool** | NextVersionSupported indicates whether the next consensus version is supported by this node | 
**stopped_at_unsupported_round** | **bool** | StoppedAtUnsupportedRound indicates that the node does not support the new rounds and has stopped making progress | 
**time_since_last_round** | **int** | TimeSinceLastRound in nanoseconds | 
**last_catchpoint** | **str** | The last catchpoint seen by the node | [optional] 
**catchpoint** | **str** | The current catchpoint that is being caught up to | [optional] 
**catchpoint_total_accounts** | **int** | The total number of accounts included in the current catchpoint | [optional] 
**catchpoint_processed_accounts** | **int** | The number of accounts from the current catchpoint that have been processed so far as part of the catchup | [optional] 
**catchpoint_verified_accounts** | **int** | The number of accounts from the current catchpoint that have been verified so far as part of the catchup | [optional] 
**catchpoint_total_kvs** | **int** | The total number of key-values (KVs) included in the current catchpoint | [optional] 
**catchpoint_processed_kvs** | **int** | The number of key-values (KVs) from the current catchpoint that have been processed so far as part of the catchup | [optional] 
**catchpoint_verified_kvs** | **int** | The number of key-values (KVs) from the current catchpoint that have been verified so far as part of the catchup | [optional] 
**catchpoint_total_blocks** | **int** | The total number of blocks that are required to complete the current catchpoint catchup | [optional] 
**catchpoint_acquired_blocks** | **int** | The number of blocks that have already been obtained by the node as part of the catchup | [optional] 
**upgrade_delay** | **int** | Upgrade delay | [optional] 
**upgrade_node_vote** | **bool** | This node&#39;s upgrade vote | [optional] 
**upgrade_votes_required** | **int** | Yes votes required for consensus upgrade | [optional] 
**upgrade_votes** | **int** | Total votes cast for consensus upgrade | [optional] 
**upgrade_yes_votes** | **int** | Yes votes cast for consensus upgrade | [optional] 
**upgrade_no_votes** | **int** | No votes cast for consensus upgrade | [optional] 
**upgrade_next_protocol_vote_before** | **int** | Next protocol round | [optional] 
**upgrade_vote_rounds** | **int** | Total voting rounds for current upgrade | [optional] 

## Example

```python
from algokit_algod_api.models.get_status200_response import GetStatus200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetStatus200Response from a JSON string
get_status200_response_instance = GetStatus200Response.from_json(json)
# print the JSON string representation of the object
print(GetStatus200Response.to_json())

# convert the object into a dict
get_status200_response_dict = get_status200_response_instance.to_dict()
# create an instance of GetStatus200Response from a dict
get_status200_response_from_dict = GetStatus200Response.from_dict(get_status200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


