# ContainerResourceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intel_rdt** | Option<[**models::LinuxIntelRdt**](LinuxIntelRdt.md)> |  | [optional]
**oom_score_adj** | Option<**i64**> | OOMScoreAdj adjusts the score used by the OOM killer to determine processes to kill for the container's process. Optional. | [optional]
**r_limits** | Option<[**Vec<models::PosixRlimit>**](POSIXRlimit.md)> | Rlimits are POSIX rlimits to apply to the container. Optional. | [optional]
**resource_limits** | Option<[**models::LinuxResources**](LinuxResources.md)> |  | [optional]
**throttle_read_bps_device** | Option<[**std::collections::HashMap<String, models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO read rate limit per cgroup per device, bytes per second | [optional]
**throttle_read_iops_device** | Option<[**std::collections::HashMap<String, models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO read rate limit per cgroup per device, IO per second | [optional]
**throttle_write_bps_device** | Option<[**std::collections::HashMap<String, models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO write rate limit per cgroup per device, bytes per second | [optional]
**throttle_write_iops_device** | Option<[**std::collections::HashMap<String, models::LinuxThrottleDevice>**](LinuxThrottleDevice.md)> | IO write rate limit per cgroup per device, IO per second | [optional]
**unified** | Option<**std::collections::HashMap<String, String>**> | CgroupConf are key-value options passed into the container runtime that are used to configure cgroup v2. Optional. | [optional]
**weight_device** | Option<[**std::collections::HashMap<String, models::LinuxWeightDevice>**](LinuxWeightDevice.md)> | Weight per cgroup per device, can override BlkioWeight | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


