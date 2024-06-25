# LinuxResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_io** | Option<[**models::LinuxBlockIo**](LinuxBlockIO.md)> |  | [optional]
**cpu** | Option<[**models::LinuxCpu**](LinuxCPU.md)> |  | [optional]
**devices** | Option<[**Vec<models::LinuxDeviceCgroup>**](LinuxDeviceCgroup.md)> | Devices configures the device allowlist. | [optional]
**hugepage_limits** | Option<[**Vec<models::LinuxHugepageLimit>**](LinuxHugepageLimit.md)> | Hugetlb limits (in bytes). Default to reservation limits if supported. | [optional]
**memory** | Option<[**models::LinuxMemory**](LinuxMemory.md)> |  | [optional]
**network** | Option<[**models::LinuxNetwork**](LinuxNetwork.md)> |  | [optional]
**pids** | Option<[**models::LinuxPids**](LinuxPids.md)> |  | [optional]
**rdma** | Option<[**std::collections::HashMap<String, models::LinuxRdma>**](LinuxRdma.md)> | Rdma resource restriction configuration. Limits are a set of key value pairs that define RDMA resource limits, where the key is device name and value is resource limits. | [optional]
**unified** | Option<**std::collections::HashMap<String, String>**> | Unified resources. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


