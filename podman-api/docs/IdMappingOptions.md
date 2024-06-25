# IdMappingOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_user_ns** | Option<**bool**> |  | [optional]
**auto_user_ns_opts** | Option<[**models::AutoUserNsOptions**](AutoUserNsOptions.md)> |  | [optional]
**gid_map** | Option<[**Vec<models::IdMap>**](IDMap.md)> |  | [optional]
**host_gid_mapping** | Option<**bool**> |  | [optional]
**host_uid_mapping** | Option<**bool**> | UIDMap and GIDMap are used for setting up a layer's root filesystem for use inside of a user namespace where ID mapping is being used. If HostUIDMapping/HostGIDMapping is true, no mapping of the respective type will be used.  Otherwise, if UIDMap and/or GIDMap contain at least one mapping, one or both will be used.  By default, if neither of those conditions apply, if the layer has a parent layer, the parent layer's mapping will be used, and if it does not have a parent layer, the mapping which was passed to the Store object when it was initialized will be used. | [optional]
**uid_map** | Option<[**Vec<models::IdMap>**](IDMap.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


