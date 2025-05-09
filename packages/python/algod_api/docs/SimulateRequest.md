# SimulateRequest

Request type for simulation endpoint.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txn_groups** | [**List[SimulateRequestTransactionGroup]**](SimulateRequestTransactionGroup.md) | The transaction groups to simulate. | 
**round** | **int** | If provided, specifies the round preceding the simulation. State changes through this round will be used to run this simulation. Usually only the 4 most recent rounds will be available (controlled by the node config value MaxAcctLookback). If not specified, defaults to the latest available round. | [optional] 
**allow_empty_signatures** | **bool** | Allows transactions without signatures to be simulated as if they had correct signatures. | [optional] 
**allow_more_logging** | **bool** | Lifts limits on log opcode usage during simulation. | [optional] 
**allow_unnamed_resources** | **bool** | Allows access to unnamed resources during simulation. | [optional] 
**extra_opcode_budget** | **int** | Applies extra opcode budget during simulation for each transaction group. | [optional] 
**exec_trace_config** | [**SimulateTraceConfig**](SimulateTraceConfig.md) |  | [optional] 
**fix_signers** | **bool** | If true, signers for transactions that are missing signatures will be fixed during evaluation. | [optional] 

## Example

```python
from algokit_algod_api.models.simulate_request import SimulateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateRequest from a JSON string
simulate_request_instance = SimulateRequest.from_json(json)
# print the JSON string representation of the object
print(SimulateRequest.to_json())

# convert the object into a dict
simulate_request_dict = simulate_request_instance.to_dict()
# create an instance of SimulateRequest from a dict
simulate_request_from_dict = SimulateRequest.from_dict(simulate_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


