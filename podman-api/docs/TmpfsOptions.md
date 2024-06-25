# TmpfsOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<**i32**> | The bits have the same definition on all systems, so that information about files can be moved from one system to another portably. Not all bits apply to all systems. The only required bit is [ModeDir] for directories. | [optional]
**size_bytes** | Option<**i64**> | Size sets the size of the tmpfs, in bytes.  This will be converted to an operating system specific value depending on the host. For example, on linux, it will be converted to use a 'k', 'm' or 'g' syntax. BSD, though not widely supported with docker, uses a straight byte value.  Percentages are not supported. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


