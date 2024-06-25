# ImageData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**annotations** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**architecture** | Option<**String**> |  | [optional]
**author** | Option<**String**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**config** | Option<[**models::ImageConfig**](ImageConfig.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**digest** | Option<**String**> | The following is an example of the contents of Digest types:  sha256:7173b809ca12ec5dee4506cd86be934c4596dd234ee82c0662eac04a8c2c71dc  This allows to abstract the digest behind this type and work only in those terms. | [optional]
**graph_driver** | Option<[**models::DriverData**](DriverData.md)> |  | [optional]
**healthcheck** | Option<[**models::Schema2HealthConfig**](Schema2HealthConfig.md)> |  | [optional]
**history** | Option<[**Vec<models::History>**](History.md)> |  | [optional]
**id** | Option<**String**> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**manifest_type** | Option<**String**> |  | [optional]
**names_history** | Option<**Vec<String>**> |  | [optional]
**os** | Option<**String**> |  | [optional]
**parent** | Option<**String**> |  | [optional]
**repo_digests** | Option<**Vec<String>**> |  | [optional]
**repo_tags** | Option<**Vec<String>**> |  | [optional]
**root_fs** | Option<[**models::RootFs**](RootFS.md)> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**user** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**virtual_size** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


