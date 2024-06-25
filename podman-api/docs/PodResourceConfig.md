# PodResourceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_period** | Option<**i32**> | CPU period of the cpuset, determined by --cpus | [optional]
**cpu_quota** | Option<**i64**> | CPU quota of the cpuset, determined by --cpus | [optional]
**resource_limits** | Option<[**models::LinuxResources**](LinuxResources.md)> |  | [optional]
**throttle_read_bps_device** | Option<[**std::collections::HashMap<String, models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | ThrottleReadBpsDevice contains the rate at which the devices in the pod can be read from/accessed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


