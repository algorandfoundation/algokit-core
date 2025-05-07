# StateProofMessage

Represents the message that the state proofs are attesting to.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_headers_commitment** | **bytearray** | The vector commitment root on all light block headers within a state proof interval. | 
**voters_commitment** | **bytearray** | The vector commitment root of the top N accounts to sign the next StateProof. | 
**ln_proven_weight** | **int** | An integer value representing the natural log of the proven weight with 16 bits of precision. This value would be used to verify the next state proof. | 
**first_attested_round** | **int** | The first round the message attests to. | 
**last_attested_round** | **int** | The last round the message attests to. | 

## Example

```python
from algokit_algod_api.models.state_proof_message import StateProofMessage

# TODO update the JSON string below
json = "{}"
# create an instance of StateProofMessage from a JSON string
state_proof_message_instance = StateProofMessage.from_json(json)
# print the JSON string representation of the object
print(StateProofMessage.to_json())

# convert the object into a dict
state_proof_message_dict = state_proof_message_instance.to_dict()
# create an instance of StateProofMessage from a dict
state_proof_message_from_dict = StateProofMessage.from_dict(state_proof_message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


