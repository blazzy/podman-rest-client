# NamedVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dest** | Option<**String**> | Destination to mount the named volume within the container. Must be an absolute path. Path will be created if it does not exist. | [optional]
**is_anonymous** | Option<**bool**> | IsAnonymous sets the named volume as anonymous even if it has a name This is used for emptyDir volumes from a kube yaml | [optional]
**name** | Option<**String**> | Name is the name of the named volume to be mounted. May be empty. If empty, a new named volume with a pseudorandomly generated name will be mounted at the given destination. | [optional]
**options** | Option<**Vec<String>**> | Options are options that the named volume will be mounted with. | [optional]
**sub_path** | Option<**String**> | SubPath stores the sub directory of the named volume to be mounted in the container | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


