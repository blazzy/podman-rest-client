# ClusterVolumeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_mode** | Option<[**models::AccessMode**](AccessMode.md)> |  | [optional]
**accessibility_requirements** | Option<[**models::TopologyRequirement**](TopologyRequirement.md)> |  | [optional]
**availability** | Option<**String**> |  | [optional]
**capacity_range** | Option<[**models::CapacityRange**](CapacityRange.md)> |  | [optional]
**group** | Option<**String**> | Group defines the volume group of this volume. Volumes belonging to the same group can be referred to by group name when creating Services. Referring to a volume by group instructs swarm to treat volumes in that group interchangeably for the purpose of scheduling. Volumes with an empty string for a group technically all belong to the same, emptystring group. | [optional]
**secrets** | Option<[**Vec<models::Secret>**](Secret.md)> | Secrets defines Swarm Secrets that are passed to the CSI storage plugin when operating on this volume. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


