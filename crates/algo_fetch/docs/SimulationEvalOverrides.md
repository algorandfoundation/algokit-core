# SimulationEvalOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_empty_signatures** | Option<**bool**> | If true, transactions without signatures are allowed and simulated as if they were properly signed. | [optional]
**allow_unnamed_resources** | Option<**bool**> | If true, allows access to unnamed resources during simulation. | [optional]
**extra_opcode_budget** | Option<**i32**> | The extra opcode budget added to each transaction group during simulation | [optional]
**fix_signers** | Option<**bool**> | If true, signers for transactions that are missing signatures will be fixed during evaluation. | [optional]
**max_log_calls** | Option<**i32**> | The maximum log calls one can make during simulation | [optional]
**max_log_size** | Option<**i32**> | The maximum byte number to log during simulation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


