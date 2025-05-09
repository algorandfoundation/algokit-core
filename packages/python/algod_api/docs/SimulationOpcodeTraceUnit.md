# SimulationOpcodeTraceUnit

The set of trace information and effect from evaluating a single opcode.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pc** | **int** | The program counter of the current opcode being evaluated. | 
**scratch_changes** | [**List[ScratchChange]**](ScratchChange.md) | The writes into scratch slots. | [optional] 
**state_changes** | [**List[ApplicationStateOperation]**](ApplicationStateOperation.md) | The operations against the current application&#39;s states. | [optional] 
**spawned_inners** | **List[int]** | The indexes of the traces for inner transactions spawned by this opcode, if any. | [optional] 
**stack_pop_count** | **int** | The number of deleted stack values by this opcode. | [optional] 
**stack_additions** | [**List[AvmValue]**](AvmValue.md) | The values added by this opcode to the stack. | [optional] 

## Example

```python
from algokit_algod_api.models.simulation_opcode_trace_unit import SimulationOpcodeTraceUnit

# TODO update the JSON string below
json = "{}"
# create an instance of SimulationOpcodeTraceUnit from a JSON string
simulation_opcode_trace_unit_instance = SimulationOpcodeTraceUnit.from_json(json)
# print the JSON string representation of the object
print(SimulationOpcodeTraceUnit.to_json())

# convert the object into a dict
simulation_opcode_trace_unit_dict = simulation_opcode_trace_unit_instance.to_dict()
# create an instance of SimulationOpcodeTraceUnit from a dict
simulation_opcode_trace_unit_from_dict = SimulationOpcodeTraceUnit.from_dict(simulation_opcode_trace_unit_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


