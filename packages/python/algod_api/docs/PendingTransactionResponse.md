# PendingTransactionResponse

Details about a pending transaction. If the transaction was recently confirmed, includes confirmation details like the round and reward details.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_index** | **int** | The asset index if the transaction was found and it created an asset. | [optional] 
**application_index** | **int** | The application index if the transaction was found and it created an application. | [optional] 
**close_rewards** | **int** | Rewards in microalgos applied to the close remainder to account. | [optional] 
**closing_amount** | **int** | Closing amount for the transaction. | [optional] 
**asset_closing_amount** | **int** | The number of the asset&#39;s unit that were transferred to the close-to address. | [optional] 
**confirmed_round** | **int** | The round where this transaction was confirmed, if present. | [optional] 
**pool_error** | **str** | Indicates that the transaction was kicked out of this node&#39;s transaction pool (and specifies why that happened).  An empty string indicates the transaction wasn&#39;t kicked out of this node&#39;s txpool due to an error.  | 
**receiver_rewards** | **int** | Rewards in microalgos applied to the receiver account. | [optional] 
**sender_rewards** | **int** | Rewards in microalgos applied to the sender account. | [optional] 
**local_state_delta** | [**List[AccountStateDelta]**](AccountStateDelta.md) | Local state key/value changes for the application being executed by this transaction. | [optional] 
**global_state_delta** | [**List[EvalDeltaKeyValue]**](EvalDeltaKeyValue.md) | Application state delta. | [optional] 
**logs** | **List[bytearray]** | Logs for the application being executed by this transaction. | [optional] 
**inner_txns** | [**List[PendingTransactionResponse]**](PendingTransactionResponse.md) | Inner transactions produced by application execution. | [optional] 
**txn** | **object** | The raw signed transaction. | 

## Example

```python
from algokit_algod_api.models.pending_transaction_response import PendingTransactionResponse

# TODO update the JSON string below
json = "{}"
# create an instance of PendingTransactionResponse from a JSON string
pending_transaction_response_instance = PendingTransactionResponse.from_json(json)
# print the JSON string representation of the object
print(PendingTransactionResponse.to_json())

# convert the object into a dict
pending_transaction_response_dict = pending_transaction_response_instance.to_dict()
# create an instance of PendingTransactionResponse from a dict
pending_transaction_response_from_dict = PendingTransactionResponse.from_dict(pending_transaction_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


