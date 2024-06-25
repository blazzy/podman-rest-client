# AccessMode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_volume** | Option<[**serde_json::Value**](.md)> | Intentionally empty. | [optional]
**mount_volume** | Option<[**models::TypeMount**](TypeMount.md)> |  | [optional]
**scope** | Option<**String**> | Scope defines the Scope of a Cluster Volume. This is how many nodes a Volume can be accessed simultaneously on. | [optional]
**sharing** | Option<**String**> | SharingMode defines the Sharing of a Cluster Volume. This is how Tasks using a Volume at the same time can use it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


