# LinuxMemory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**check_before_update** | Option<**bool**> | CheckBeforeUpdate enables checking if a new memory limit is lower than the current usage during update, and if so, rejecting the new limit. | [optional]
**disable_oom_killer** | Option<**bool**> | DisableOOMKiller disables the OOM killer for out of memory conditions | [optional]
**kernel** | Option<**i64**> | Kernel memory limit (in bytes).  Deprecated: kernel-memory limits are not supported in cgroups v2, and were obsoleted in [kernel v5.4]. This field should no longer be used, as it may be ignored by runtimes.  [kernel v5.4]: https://github.com/torvalds/linux/commit/0158115f702b0ba208ab0 | [optional]
**kernel_tcp** | Option<**i64**> | Kernel memory limit for tcp (in bytes) | [optional]
**limit** | Option<**i64**> | Memory limit (in bytes). | [optional]
**reservation** | Option<**i64**> | Memory reservation or soft_limit (in bytes). | [optional]
**swap** | Option<**i64**> | Total memory limit (memory + swap). | [optional]
**swappiness** | Option<**i32**> | How aggressive the kernel will swap memory pages. | [optional]
**use_hierarchy** | Option<**bool**> | Enables hierarchical memory accounting | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


