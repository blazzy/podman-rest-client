# PodCreateOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cgroup_parent** | Option<**String**> |  | [optional]
**container_command** | Option<**String**> |  | [optional]
**container_conmon_pidfile** | Option<**String**> |  | [optional]
**container_name** | Option<**String**> |  | [optional]
**cpus** | Option<**f64**> |  | [optional]
**cpuset_cpus** | Option<**String**> |  | [optional]
**create_command** | Option<**Vec<String>**> |  | [optional]
**device_read_bps** | Option<**Vec<String>**> |  | [optional]
**devices** | Option<**Vec<String>**> |  | [optional]
**exit_policy** | Option<**String**> |  | [optional]
**hostname** | Option<**String**> |  | [optional]
**infra** | Option<**bool**> |  | [optional]
**infra_image** | Option<**String**> |  | [optional]
**ipc** | Option<**String**> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**net** | Option<[**models::NetOptions**](NetOptions.md)> |  | [optional]
**pid** | Option<**String**> |  | [optional]
**restart** | Option<**String**> |  | [optional]
**security_opt** | Option<**Vec<String>**> |  | [optional]
**share** | Option<**Vec<String>**> |  | [optional]
**share_parent** | Option<**bool**> |  | [optional]
**sysctl** | Option<**Vec<String>**> |  | [optional]
**uts** | Option<**String**> |  | [optional]
**volume** | Option<**Vec<String>**> |  | [optional]
**volumes_from** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


