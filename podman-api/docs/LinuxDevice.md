# LinuxDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_mode** | Option<**i32**> | The bits have the same definition on all systems, so that information about files can be moved from one system to another portably. Not all bits apply to all systems. The only required bit is [ModeDir] for directories. | [optional]
**gid** | Option<**i32**> | Gid of the device. | [optional]
**major** | Option<**i64**> | Major is the device's major number. | [optional]
**minor** | Option<**i64**> | Minor is the device's minor number. | [optional]
**path** | Option<**String**> | Path to the device. | [optional]
**r#type** | Option<**String**> | Device type, block, char, etc. | [optional]
**uid** | Option<**i32**> | UID of the device. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


