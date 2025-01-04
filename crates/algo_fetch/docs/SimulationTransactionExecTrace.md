# SimulationTransactionExecTrace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approval_program_hash** | Option<**String**> | SHA512_256 hash digest of the approval program executed in transaction. | [optional]
**approval_program_trace** | Option<[**Vec<models::SimulationOpcodeTraceUnit>**](SimulationOpcodeTraceUnit.md)> | Program trace that contains a trace of opcode effects in an approval program. | [optional]
**clear_state_program_hash** | Option<**String**> | SHA512_256 hash digest of the clear state program executed in transaction. | [optional]
**clear_state_program_trace** | Option<[**Vec<models::SimulationOpcodeTraceUnit>**](SimulationOpcodeTraceUnit.md)> | Program trace that contains a trace of opcode effects in a clear state program. | [optional]
**clear_state_rollback** | Option<**bool**> | If true, indicates that the clear state program failed and any persistent state changes it produced should be reverted once the program exits. | [optional]
**clear_state_rollback_error** | Option<**String**> | The error message explaining why the clear state program failed. This field will only be populated if clear-state-rollback is true and the failure was due to an execution error. | [optional]
**inner_trace** | Option<[**Vec<models::SimulationTransactionExecTrace>**](SimulationTransactionExecTrace.md)> | An array of SimulationTransactionExecTrace representing the execution trace of any inner transactions executed. | [optional]
**logic_sig_hash** | Option<**String**> | SHA512_256 hash digest of the logic sig executed in transaction. | [optional]
**logic_sig_trace** | Option<[**Vec<models::SimulationOpcodeTraceUnit>**](SimulationOpcodeTraceUnit.md)> | Program trace that contains a trace of opcode effects in a logic sig. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


