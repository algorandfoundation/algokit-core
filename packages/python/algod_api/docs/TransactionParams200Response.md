# TransactionParams200Response

TransactionParams contains the parameters that help a client construct a new transaction.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**consensus_version** | **str** | ConsensusVersion indicates the consensus protocol version as of LastRound. | 
**fee** | **int** | Fee is the suggested transaction fee Fee is in units of micro-Algos per byte. Fee may fall to zero but transactions must still have a fee of at least MinTxnFee for the current network protocol. | 
**genesis_hash** | **bytearray** | GenesisHash is the hash of the genesis block. | 
**genesis_id** | **str** | GenesisID is an ID listed in the genesis block. | 
**last_round** | **int** | LastRound indicates the last round seen | 
**min_fee** | **int** | The minimum transaction fee (not per byte) required for the txn to validate for the current network protocol. | 

## Example

```python
from algokit_algod_api.models.transaction_params200_response import TransactionParams200Response

# TODO update the JSON string below
json = "{}"
# create an instance of TransactionParams200Response from a JSON string
transaction_params200_response_instance = TransactionParams200Response.from_json(json)
# print the JSON string representation of the object
print(TransactionParams200Response.to_json())

# convert the object into a dict
transaction_params200_response_dict = transaction_params200_response_instance.to_dict()
# create an instance of TransactionParams200Response from a dict
transaction_params200_response_from_dict = TransactionParams200Response.from_dict(transaction_params200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


