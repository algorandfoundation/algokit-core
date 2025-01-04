# SimulateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_empty_signatures** | Option<**bool**> | Allows transactions without signatures to be simulated as if they had correct signatures. | [optional]
**allow_more_logging** | Option<**bool**> | Lifts limits on log opcode usage during simulation. | [optional]
**allow_unnamed_resources** | Option<**bool**> | Allows access to unnamed resources during simulation. | [optional]
**exec_trace_config** | Option<[**models::SimulateTraceConfig**](SimulateTraceConfig.md)> |  | [optional]
**extra_opcode_budget** | Option<**i32**> | Applies extra opcode budget during simulation for each transaction group. | [optional]
**fix_signers** | Option<**bool**> | If true, signers for transactions that are missing signatures will be fixed during evaluation. | [optional]
**round** | Option<**i32**> | If provided, specifies the round preceding the simulation. State changes through this round will be used to run this simulation. Usually only the 4 most recent rounds will be available (controlled by the node config value MaxAcctLookback). If not specified, defaults to the latest available round. | [optional]
**txn_groups** | [**Vec<models::SimulateRequestTransactionGroup>**](SimulateRequestTransactionGroup.md) | The transaction groups to simulate. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


