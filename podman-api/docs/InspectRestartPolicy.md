# InspectRestartPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maximum_retry_count** | Option<**i32**> | MaximumRetryCount is the maximum number of retries allowed if the \"on-failure\" restart policy is in use. Not used if \"on-failure\" is not set. | [optional]
**name** | Option<**String**> | Name contains the container's restart policy. Allowable values are \"no\" or \"\" (take no action), \"on-failure\" (restart on non-zero exit code, with an optional max retry count), and \"always\" (always restart on container stop, unless explicitly requested by API). Note that this is NOT actually a name of any sort - the poor naming is for Docker compatibility. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


