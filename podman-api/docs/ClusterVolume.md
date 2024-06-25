# ClusterVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional]
**id** | Option<**String**> | ID is the Swarm ID of the volume. Because cluster volumes are Swarm objects, they have an ID, unlike non-cluster volumes, which only have a Name. This ID can be used to refer to the cluster volume. | [optional]
**info** | Option<[**models::Info**](Info.md)> |  | [optional]
**publish_status** | Option<[**Vec<models::PublishStatus>**](PublishStatus.md)> | PublishStatus contains the status of the volume as it pertains to its publishing on Nodes. | [optional]
**spec** | Option<[**models::ClusterVolumeSpec**](ClusterVolumeSpec.md)> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**version** | Option<[**models::Version**](Version.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


