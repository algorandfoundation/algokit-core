# SimulateRequestTransactionGroup

A transaction group to simulate.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txns** | **List[str]** | An atomic transaction group. | 

## Example

```python
from algokit_algod_api.models.simulate_request_transaction_group import SimulateRequestTransactionGroup

# TODO update the JSON string below
json = "{}"
# create an instance of SimulateRequestTransactionGroup from a JSON string
simulate_request_transaction_group_instance = SimulateRequestTransactionGroup.from_json(json)
# print the JSON string representation of the object
print(SimulateRequestTransactionGroup.to_json())

# convert the object into a dict
simulate_request_transaction_group_dict = simulate_request_transaction_group_instance.to_dict()
# create an instance of SimulateRequestTransactionGroup from a dict
simulate_request_transaction_group_from_dict = SimulateRequestTransactionGroup.from_dict(simulate_request_transaction_group_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


