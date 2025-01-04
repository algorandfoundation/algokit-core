# SimulationOpcodeTraceUnit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pc** | **i32** | The program counter of the current opcode being evaluated. | 
**scratch_changes** | Option<[**Vec<models::ScratchChange>**](ScratchChange.md)> | The writes into scratch slots. | [optional]
**spawned_inners** | Option<**Vec<i32>**> | The indexes of the traces for inner transactions spawned by this opcode, if any. | [optional]
**stack_additions** | Option<[**Vec<models::AvmValue>**](AvmValue.md)> | The values added by this opcode to the stack. | [optional]
**stack_pop_count** | Option<**i32**> | The number of deleted stack values by this opcode. | [optional]
**state_changes** | Option<[**Vec<models::ApplicationStateOperation>**](ApplicationStateOperation.md)> | The operations against the current application's states. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


