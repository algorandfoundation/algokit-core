# SimulateTransactionResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_budget_consumed** | Option<**i32**> | Budget used during execution of an app call transaction. This value includes budged used by inner app calls spawned by this transaction. | [optional]
**exec_trace** | Option<[**models::SimulationTransactionExecTrace**](SimulationTransactionExecTrace.md)> |  | [optional]
**fixed_signer** | Option<**String**> | The account that needed to sign this transaction when no signature was provided and the provided signer was incorrect. | [optional]
**logic_sig_budget_consumed** | Option<**i32**> | Budget used during execution of a logic sig transaction. | [optional]
**txn_result** | [**models::PendingTransactionResponse**](PendingTransactionResponse.md) |  | 
**unnamed_resources_accessed** | Option<[**models::SimulateUnnamedResourcesAccessed**](SimulateUnnamedResourcesAccessed.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


