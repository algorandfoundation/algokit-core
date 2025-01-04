# ApplicationStateOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | Option<**String**> | For local state changes, the address of the account associated with the local state. | [optional]
**app_state_type** | **String** | Type of application state. Value `g` is **global state**, `l` is **local state**, `b` is **boxes**. | 
**key** | **String** | The key (name) of the global/local/box state. | 
**new_value** | Option<[**models::AvmValue**](AvmValue.md)> |  | [optional]
**operation** | **String** | Operation type. Value `w` is **write**, `d` is **delete**. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


