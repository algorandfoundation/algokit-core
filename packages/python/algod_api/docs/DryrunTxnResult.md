# DryrunTxnResult

DryrunTxnResult contains any LogicSig or ApplicationCall program debug information and state updates from a dryrun.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disassembly** | **List[str]** | Disassembled program line by line. | 
**logic_sig_disassembly** | **List[str]** | Disassembled lsig program line by line. | [optional] 
**logic_sig_trace** | [**List[DryrunState]**](DryrunState.md) |  | [optional] 
**logic_sig_messages** | **List[str]** |  | [optional] 
**app_call_trace** | [**List[DryrunState]**](DryrunState.md) |  | [optional] 
**app_call_messages** | **List[str]** |  | [optional] 
**global_delta** | [**List[EvalDeltaKeyValue]**](EvalDeltaKeyValue.md) | Application state delta. | [optional] 
**local_deltas** | [**List[AccountStateDelta]**](AccountStateDelta.md) |  | [optional] 
**logs** | **List[bytearray]** |  | [optional] 
**budget_added** | **int** | Budget added during execution of app call transaction. | [optional] 
**budget_consumed** | **int** | Budget consumed during execution of app call transaction. | [optional] 

## Example

```python
from algokit_algod_api.models.dryrun_txn_result import DryrunTxnResult

# TODO update the JSON string below
json = "{}"
# create an instance of DryrunTxnResult from a JSON string
dryrun_txn_result_instance = DryrunTxnResult.from_json(json)
# print the JSON string representation of the object
print(DryrunTxnResult.to_json())

# convert the object into a dict
dryrun_txn_result_dict = dryrun_txn_result_instance.to_dict()
# create an instance of DryrunTxnResult from a dict
dryrun_txn_result_from_dict = DryrunTxnResult.from_dict(dryrun_txn_result_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


