# GetTransactionProof200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proof** | **bytearray** | Proof of transaction membership. | 
**stibhash** | **bytearray** | Hash of SignedTxnInBlock for verifying proof. | 
**treedepth** | **int** | Represents the depth of the tree that is being proven, i.e. the number of edges from a leaf to the root. | 
**idx** | **int** | Index of the transaction in the block&#39;s payset. | 
**hashtype** | **str** | The type of hash function used to create the proof, must be one of:  * sha512_256  * sha256 | 

## Example

```python
from algokit_algod_api.models.get_transaction_proof200_response import GetTransactionProof200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetTransactionProof200Response from a JSON string
get_transaction_proof200_response_instance = GetTransactionProof200Response.from_json(json)
# print the JSON string representation of the object
print(GetTransactionProof200Response.to_json())

# convert the object into a dict
get_transaction_proof200_response_dict = get_transaction_proof200_response_instance.to_dict()
# create an instance of GetTransactionProof200Response from a dict
get_transaction_proof200_response_from_dict = GetTransactionProof200Response.from_dict(get_transaction_proof200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


