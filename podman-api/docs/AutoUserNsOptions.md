# AutoUserNsOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_gid_mappings** | Option<[**Vec<models::IdMap>**](IDMap.md)> | AdditionalGIDMappings specified additional GID mappings to include in the generated user namespace. | [optional]
**additional_uid_mappings** | Option<[**Vec<models::IdMap>**](IDMap.md)> | AdditionalUIDMappings specified additional UID mappings to include in the generated user namespace. | [optional]
**group_file** | Option<**String**> | GroupFile to use if the container uses a volume. | [optional]
**initial_size** | Option<**i32**> | InitialSize defines the minimum size for the user namespace. The created user namespace will have at least this size. | [optional]
**passwd_file** | Option<**String**> | PasswdFile to use if the container uses a volume. | [optional]
**size** | Option<**i32**> | Size defines the size for the user namespace.  If it is set to a value bigger than 0, the user namespace will have exactly this size. If it is not set, some heuristics will be used to find its size. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


