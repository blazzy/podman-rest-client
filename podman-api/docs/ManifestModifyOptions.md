# ManifestModifyOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**all** | Option<**bool**> | True when operating on a list to include all images | [optional]
**annotation** | Option<**Vec<String>**> | Annotation to add to the item in the manifest list | [optional]
**annotations** | Option<**std::collections::HashMap<String, String>**> | Annotations to add to the item in the manifest list by a map which is preferred over Annotation | [optional]
**arch** | Option<**String**> | Arch overrides the architecture for the item in the manifest list | [optional]
**artifact_annotations** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**artifact_config** | Option<**String**> |  | [optional]
**artifact_config_type** | Option<**String**> |  | [optional]
**artifact_exclude_titles** | Option<**bool**> |  | [optional]
**artifact_files** | Option<**Vec<String>**> |  | [optional]
**artifact_layer_type** | Option<**String**> |  | [optional]
**artifact_subject** | Option<**String**> |  | [optional]
**artifact_type** | Option<**String**> | The following are all of the fields from ManifestAddArtifactOptions. We can't just embed the whole structure because it embeds a ManifestAnnotateOptions, which would conflict with the one that ManifestAddOptions embeds. | [optional]
**features** | Option<**Vec<String>**> | Feature list for the item in the manifest list | [optional]
**images** | Option<**Vec<String>**> | Images is an optional list of image references to add to manifest list | [optional]
**index_annotation** | Option<**Vec<String>**> | IndexAnnotation is a slice of key=value annotations to add to the manifest list itself | [optional]
**index_annotations** | Option<**std::collections::HashMap<String, String>**> | IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation | [optional]
**operation** | Option<**String**> |  | [optional]
**os** | Option<**String**> | OS overrides the operating system for the item in the manifest list | [optional]
**os_features** | Option<**Vec<String>**> | OS features for the item in the manifest list | [optional]
**os_version** | Option<**String**> | OSVersion overrides the operating system for the item in the manifest list | [optional]
**subject** | Option<**String**> | IndexSubject is a subject value to set in the manifest list itself | [optional]
**variant** | Option<**String**> | Variant for the item in the manifest list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


