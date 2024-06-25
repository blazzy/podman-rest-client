# Mount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bind_options** | Option<[**models::BindOptions**](BindOptions.md)> |  | [optional]
**cluster_options** | Option<[**serde_json::Value**](.md)> |  | [optional]
**consistency** | Option<**String**> |  | [optional]
**read_only** | Option<**bool**> |  | [optional]
**source** | Option<**String**> | Source specifies the name of the mount. Depending on mount type, this may be a volume name or a host path, or even ignored. Source is not supported for tmpfs (must be an empty value) | [optional]
**target** | Option<**String**> |  | [optional]
**tmpfs_options** | Option<[**models::TmpfsOptions**](TmpfsOptions.md)> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**volume_options** | Option<[**models::VolumeOptions**](VolumeOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


