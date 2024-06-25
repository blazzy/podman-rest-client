# UpdateConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blkio_device_read_bps** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> |  | [optional]
**blkio_device_read_i_ops** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> |  | [optional]
**blkio_device_write_bps** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> |  | [optional]
**blkio_device_write_i_ops** | Option<[**Vec<models::ThrottleDevice>**](ThrottleDevice.md)> |  | [optional]
**blkio_weight** | Option<**i32**> |  | [optional]
**blkio_weight_device** | Option<[**Vec<models::WeightDevice>**](WeightDevice.md)> |  | [optional]
**cgroup_parent** | Option<**String**> | Applicable to UNIX platforms | [optional]
**cpu_count** | Option<**i64**> | Applicable to Windows | [optional]
**cpu_percent** | Option<**i64**> |  | [optional]
**cpu_period** | Option<**i64**> |  | [optional]
**cpu_quota** | Option<**i64**> |  | [optional]
**cpu_realtime_period** | Option<**i64**> |  | [optional]
**cpu_realtime_runtime** | Option<**i64**> |  | [optional]
**cpu_shares** | Option<**i64**> | Applicable to all platforms | [optional]
**cpuset_cpus** | Option<**String**> |  | [optional]
**cpuset_mems** | Option<**String**> |  | [optional]
**device_cgroup_rules** | Option<**Vec<String>**> |  | [optional]
**device_requests** | Option<[**Vec<models::DeviceRequest>**](DeviceRequest.md)> |  | [optional]
**devices** | Option<[**Vec<models::DeviceMapping>**](DeviceMapping.md)> |  | [optional]
**io_maximum_bandwidth** | Option<**i32**> |  | [optional]
**io_maximum_i_ops** | Option<**i32**> |  | [optional]
**kernel_memory** | Option<**i64**> | KernelMemory specifies the kernel memory limit (in bytes) for the container. Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes. | [optional]
**kernel_memory_tcp** | Option<**i64**> |  | [optional]
**memory** | Option<**i64**> |  | [optional]
**memory_reservation** | Option<**i64**> |  | [optional]
**memory_swap** | Option<**i64**> |  | [optional]
**memory_swappiness** | Option<**i64**> |  | [optional]
**nano_cpus** | Option<**i64**> |  | [optional]
**oom_kill_disable** | Option<**bool**> |  | [optional]
**pids_limit** | Option<**i64**> |  | [optional]
**restart_policy** | Option<[**models::RestartPolicy**](RestartPolicy.md)> |  | [optional]
**ulimits** | Option<[**Vec<models::Ulimit>**](Ulimit.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


