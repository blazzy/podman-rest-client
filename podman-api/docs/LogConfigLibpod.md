# LogConfigLibpod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**driver** | Option<**String**> | LogDriver is the container's log driver. Optional. | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | A set of options to accompany the log driver. Optional. | [optional]
**path** | Option<**String**> | LogPath is the path the container's logs will be stored at. Only available if LogDriver is set to \"json-file\" or \"k8s-file\". Optional. | [optional]
**size** | Option<**i64**> | Size is the maximum size of the log file Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


