# CreateOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_volume_spec** | Option<[**models::ClusterVolumeSpec**](ClusterVolumeSpec.md)> |  | [optional]
**driver** | Option<**String**> | Name of the volume driver to use. | [optional]
**driver_opts** | Option<**std::collections::HashMap<String, String>**> | A mapping of driver options and values. These options are passed directly to the driver and are driver specific. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**name** | Option<**String**> | The new volume's name. If not specified, Docker generates a name. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


