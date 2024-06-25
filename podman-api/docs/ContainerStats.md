# ContainerStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avg_cpu** | Option<**f64**> |  | [optional]
**block_input** | Option<**i32**> |  | [optional]
**block_output** | Option<**i32**> |  | [optional]
**cpu** | Option<**f64**> |  | [optional]
**cpu_nano** | Option<**i32**> |  | [optional]
**cpu_system_nano** | Option<**i32**> |  | [optional]
**container_id** | Option<**String**> |  | [optional]
**duration** | Option<**i32**> |  | [optional]
**mem_limit** | Option<**i32**> |  | [optional]
**mem_perc** | Option<**f64**> |  | [optional]
**mem_usage** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**network** | Option<[**std::collections::HashMap<String, models::ContainerNetworkStats>**](ContainerNetworkStats.md)> | Map of interface name to network statistics for that interface. | [optional]
**pids** | Option<**i32**> |  | [optional]
**per_cpu** | Option<**Vec<i32>**> |  | [optional]
**system_nano** | Option<**i32**> |  | [optional]
**up_time** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


