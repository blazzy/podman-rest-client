# PlayKubeReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exit_code** | Option<**i32**> | If set, exit with the specified exit code. | [optional]
**pods** | Option<[**Vec<models::PlayKubePod>**](PlayKubePod.md)> | Pods - pods created by play kube. | [optional]
**rm_report** | Option<[**Vec<models::PodRmReport>**](PodRmReport.md)> |  | [optional]
**secret_rm_report** | Option<[**Vec<models::SecretRmReport>**](SecretRmReport.md)> |  | [optional]
**secrets** | Option<[**Vec<models::PlaySecret>**](PlaySecret.md)> | Secrets - secrets created by play kube | [optional]
**service_container_id** | Option<**String**> | ServiceContainerID - ID of the service container if one is created | [optional]
**stop_report** | Option<[**Vec<models::PodStopReport>**](PodStopReport.md)> |  | [optional]
**volume_rm_report** | Option<[**Vec<models::VolumeRmReport>**](VolumeRmReport.md)> |  | [optional]
**volumes** | Option<[**Vec<models::PlayKubeVolume>**](PlayKubeVolume.md)> | Volumes - volumes created by play kube. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


