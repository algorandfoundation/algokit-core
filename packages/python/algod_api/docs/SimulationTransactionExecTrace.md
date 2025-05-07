# SimulationTransactionExecTrace

The execution trace of calling an app or a logic sig, containing the inner app call trace in a recursive way.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approval_program_trace** | [**List[SimulationOpcodeTraceUnit]**](SimulationOpcodeTraceUnit.md) | Program trace that contains a trace of opcode effects in an approval program. | [optional] 
**approval_program_hash** | **bytearray** | SHA512_256 hash digest of the approval program executed in transaction. | [optional] 
**clear_state_program_trace** | [**List[SimulationOpcodeTraceUnit]**](SimulationOpcodeTraceUnit.md) | Program trace that contains a trace of opcode effects in a clear state program. | [optional] 
**clear_state_program_hash** | **bytearray** | SHA512_256 hash digest of the clear state program executed in transaction. | [optional] 
**clear_state_rollback** | **bool** | If true, indicates that the clear state program failed and any persistent state changes it produced should be reverted once the program exits. | [optional] 
**clear_state_rollback_error** | **str** | The error message explaining why the clear state program failed. This field will only be populated if clear-state-rollback is true and the failure was due to an execution error. | [optional] 
**logic_sig_trace** | [**List[SimulationOpcodeTraceUnit]**](SimulationOpcodeTraceUnit.md) | Program trace that contains a trace of opcode effects in a logic sig. | [optional] 
**logic_sig_hash** | **bytearray** | SHA512_256 hash digest of the logic sig executed in transaction. | [optional] 
**inner_trace** | [**List[SimulationTransactionExecTrace]**](SimulationTransactionExecTrace.md) | An array of SimulationTransactionExecTrace representing the execution trace of any inner transactions executed. | [optional] 

## Example

```python
from algokit_algod_api.models.simulation_transaction_exec_trace import SimulationTransactionExecTrace

# TODO update the JSON string below
json = "{}"
# create an instance of SimulationTransactionExecTrace from a JSON string
simulation_transaction_exec_trace_instance = SimulationTransactionExecTrace.from_json(json)
# print the JSON string representation of the object
print(SimulationTransactionExecTrace.to_json())

# convert the object into a dict
simulation_transaction_exec_trace_dict = simulation_transaction_exec_trace_instance.to_dict()
# create an instance of SimulationTransactionExecTrace from a dict
simulation_transaction_exec_trace_from_dict = SimulationTransactionExecTrace.from_dict(simulation_transaction_exec_trace_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


