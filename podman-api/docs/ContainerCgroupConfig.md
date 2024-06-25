# ContainerCgroupConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cgroup_parent** | Option<**String**> | CgroupParent is the container's Cgroup parent. If not set, the default for the current cgroup driver will be used. Optional. | [optional]
**cgroupns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**cgroups_mode** | Option<**String**> | CgroupsMode sets a policy for how cgroups will be created for the container, including the ability to disable creation entirely. Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


