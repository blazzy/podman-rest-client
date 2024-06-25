# LinuxCpu

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**burst** | Option<**i32**> | CPU hardcap burst limit (in usecs). Allowed accumulated cpu time additionally for burst in a given period. | [optional]
**cpus** | Option<**String**> | CPUs to use within the cpuset. Default is to use any CPU available. | [optional]
**idle** | Option<**i64**> | cgroups are configured with minimum weight, 0: default behavior, 1: SCHED_IDLE. | [optional]
**mems** | Option<**String**> | List of memory nodes in the cpuset. Default is to use any available memory node. | [optional]
**period** | Option<**i32**> | CPU period to be used for hardcapping (in usecs). | [optional]
**quota** | Option<**i64**> | CPU hardcap limit (in usecs). Allowed cpu time in a given period. | [optional]
**realtime_period** | Option<**i32**> | CPU period to be used for realtime scheduling (in usecs). | [optional]
**realtime_runtime** | Option<**i64**> | How much time realtime scheduling may use (in usecs). | [optional]
**shares** | Option<**i32**> | CPU shares (relative weight (ratio) vs. other cgroups with cpu shares). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


