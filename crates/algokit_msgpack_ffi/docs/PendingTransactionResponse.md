# PendingTransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_index** | Option<**i32**> | The asset index if the transaction was found and it created an asset. | [optional]
**application_index** | Option<**i32**> | The application index if the transaction was found and it created an application. | [optional]
**close_rewards** | Option<**i32**> | Rewards in microalgos applied to the close remainder to account. | [optional]
**closing_amount** | Option<**i32**> | Closing amount for the transaction. | [optional]
**asset_closing_amount** | Option<**i32**> | The number of the asset's unit that were transferred to the close-to address. | [optional]
**confirmed_round** | Option<**i32**> | The round where this transaction was confirmed, if present. | [optional]
**pool_error** | **String** | Indicates that the transaction was kicked out of this node's transaction pool (and specifies why that happened).  An empty string indicates the transaction wasn't kicked out of this node's txpool due to an error.  | 
**receiver_rewards** | Option<**i32**> | Rewards in microalgos applied to the receiver account. | [optional]
**sender_rewards** | Option<**i32**> | Rewards in microalgos applied to the sender account. | [optional]
**local_state_delta** | Option<[**Vec<models::AccountStateDelta>**](AccountStateDelta.md)> | Local state key/value changes for the application being executed by this transaction. | [optional]
**global_state_delta** | Option<[**Vec<models::EvalDeltaKeyValue>**](EvalDeltaKeyValue.md)> | Application state delta. | [optional]
**logs** | Option<**Vec<String>**> | Logs for the application being executed by this transaction. | [optional]
**inner_txns** | Option<[**Vec<models::PendingTransactionResponse>**](PendingTransactionResponse.md)> | Inner transactions produced by application execution. | [optional]
**txn** | **String** | The raw signed transaction. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


