# LibpodImageSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arch** | Option<**String**> | Podman extensions | [optional]
**containers** | Option<**i64**> |  | [optional]
**created** | Option<**i64**> |  | [optional]
**dangling** | Option<**bool**> |  | [optional]
**digest** | Option<**String**> |  | [optional]
**history** | Option<**Vec<String>**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**is_manifest_list** | Option<**bool**> | IsManifestList is a ptr so we can distinguish between a true json empty response and false.  the docker compat side needs to return empty; where as the libpod side needs a value of true or false | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**names** | Option<**Vec<String>**> |  | [optional]
**os** | Option<**String**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**read_only** | Option<**bool**> |  | [optional]
**repo_digests** | Option<**Vec<String>**> |  | [optional]
**repo_tags** | Option<**Vec<String>**> |  | [optional]
**shared_size** | Option<**i64**> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**virtual_size** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


