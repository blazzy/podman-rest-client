# InspectMount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<**String**> | The destination directory for the volume. Specified as a path within the container, as it would be passed into the OCI runtime. | [optional]
**driver** | Option<**String**> | The driver used for the named volume. Empty for bind mounts. | [optional]
**mode** | Option<**String**> | Contains SELinux :z/:Z mount options. Unclear what, if anything, else goes in here. | [optional]
**name** | Option<**String**> | The name of the volume. Empty for bind mounts. | [optional]
**options** | Option<**Vec<String>**> | All remaining mount options. Additional data, not present in the original output. | [optional]
**propagation** | Option<**String**> | Mount propagation for the mount. Can be empty if not specified, but is always printed - no omitempty. | [optional]
**rw** | Option<**bool**> | Whether the volume is read-write | [optional]
**source** | Option<**String**> | The source directory for the volume. | [optional]
**r#type** | Option<**String**> | Whether the mount is a volume or bind mount. Allowed values are \"volume\" and \"bind\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


