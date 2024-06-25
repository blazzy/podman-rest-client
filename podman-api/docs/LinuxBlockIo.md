# LinuxBlockIo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**leaf_weight** | Option<**i32**> | Specifies tasks' weight in the given cgroup while competing with the cgroup's child cgroups, CFQ scheduler only | [optional]
**throttle_read_bps_device** | Option<[**Vec<models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO read rate limit per cgroup per device, bytes per second | [optional]
**throttle_read_iops_device** | Option<[**Vec<models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO read rate limit per cgroup per device, IO per second | [optional]
**throttle_write_bps_device** | Option<[**Vec<models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO write rate limit per cgroup per device, bytes per second | [optional]
**throttle_write_iops_device** | Option<[**Vec<models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO write rate limit per cgroup per device, IO per second | [optional]
**weight** | Option<**i32**> | Specifies per cgroup weight | [optional]
**weight_device** | Option<[**Vec<models::LinuxWeightDevice>**](LinuxWeightDevice.md)> | Weight per cgroup per device, can override BlkioWeight | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


