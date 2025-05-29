# SimulateTransaction200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **i32** | The version of this response object. | 
**last_round** | **i32** | The round immediately preceding this simulation. State changes through this round were used to run this simulation. | 
**txn_groups** | [**Vec<models::SimulateTransactionGroupResult>**](SimulateTransactionGroupResult.md) | A result object for each transaction group that was simulated. | 
**eval_overrides** | Option<[**models::SimulationEvalOverrides**](SimulationEvalOverrides.md)> |  | [optional]
**exec_trace_config** | Option<[**models::SimulateTraceConfig**](SimulateTraceConfig.md)> |  | [optional]
**initial_states** | Option<[**models::SimulateInitialStates**](SimulateInitialStates.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


