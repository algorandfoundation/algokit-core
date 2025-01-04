# DebugSettingsProf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_rate** | Option<**i32**> | The rate of blocking events. The profiler aims to sample an average of one blocking event per rate nanoseconds spent blocked. To turn off profiling entirely, pass rate 0. | [optional]
**mutex_rate** | Option<**i32**> | The rate of mutex events. On average 1/rate events are reported. To turn off profiling entirely, pass rate 0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


