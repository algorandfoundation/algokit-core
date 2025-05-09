# GetPendingTransactionsByAddress200Response

PendingTransactions is an array of signed transactions exactly as they were submitted.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**top_transactions** | **List[object]** | An array of signed transaction objects. | 
**total_transactions** | **int** | Total number of transactions in the pool. | 

## Example

```python
from algokit_algod_api.models.get_pending_transactions_by_address200_response import GetPendingTransactionsByAddress200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetPendingTransactionsByAddress200Response from a JSON string
get_pending_transactions_by_address200_response_instance = GetPendingTransactionsByAddress200Response.from_json(json)
# print the JSON string representation of the object
print(GetPendingTransactionsByAddress200Response.to_json())

# convert the object into a dict
get_pending_transactions_by_address200_response_dict = get_pending_transactions_by_address200_response_instance.to_dict()
# create an instance of GetPendingTransactionsByAddress200Response from a dict
get_pending_transactions_by_address200_response_from_dict = GetPendingTransactionsByAddress200Response.from_dict(get_pending_transactions_by_address200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


