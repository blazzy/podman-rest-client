# PlayKubePod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_errors** | Option<**Vec<String>**> | ContainerErrors - any errors that occurred while starting containers in the pod. | [optional]
**containers** | Option<**Vec<String>**> | Containers - the IDs of the containers running in the created pod. | [optional]
**id** | Option<**String**> | ID - ID of the pod created as a result of play kube. | [optional]
**init_containers** | Option<**Vec<String>**> | InitContainers - the IDs of the init containers to be run in the created pod. | [optional]
**logs** | Option<**Vec<String>**> | Logs - non-fatal errors and log messages while processing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


