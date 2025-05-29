# ParticipationKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The key's ParticipationID. | 
**address** | **String** | Address the key was generated for. | 
**effective_first_valid** | Option<**i32**> | When registered, this is the first round it may be used. | [optional]
**effective_last_valid** | Option<**i32**> | When registered, this is the last round it may be used. | [optional]
**last_vote** | Option<**i32**> | Round when this key was last used to vote. | [optional]
**last_block_proposal** | Option<**i32**> | Round when this key was last used to propose a block. | [optional]
**last_state_proof** | Option<**i32**> | Round when this key was last used to generate a state proof. | [optional]
**key** | [**models::AccountParticipation**](AccountParticipation.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


