# ListContainer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_remove** | Option<**bool**> | AutoRemove | [optional]
**cid_file** | Option<**String**> | CIDFile specified at creation time. | [optional]
**command** | Option<**Vec<String>**> | Container command | [optional]
**created** | Option<**String**> | Container creation time | [optional]
**created_at** | Option<**String**> | Human-readable container creation time. | [optional]
**exit_code** | Option<**i32**> | If container has exited, the return code from the command | [optional]
**exited** | Option<**bool**> | If container has exited/stopped | [optional]
**exited_at** | Option<**i64**> | Time container exited | [optional]
**exposed_ports** | Option<[**serde_json::Value**](.md)> | ExposedPorts contains the ports that are exposed but not forwarded, see Ports for forwarded ports. The key is the port number and the string slice contains the protocols, i.e. \"tcp\", \"udp\" and \"sctp\". | [optional]
**id** | Option<**String**> | The unique identifier for the container | [optional]
**image** | Option<**String**> | Container image | [optional]
**image_id** | Option<**String**> | Container image ID | [optional]
**is_infra** | Option<**bool**> | If this container is a Pod infra container | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels for container | [optional]
**mounts** | Option<**Vec<String>**> | User volume mounts | [optional]
**names** | Option<**Vec<String>**> | The names assigned to the container | [optional]
**namespaces** | Option<[**models::ListContainerNamespaces**](ListContainerNamespaces.md)> |  | [optional]
**networks** | Option<**Vec<String>**> | The network names assigned to the container | [optional]
**pid** | Option<**i64**> | The process id of the container | [optional]
**pod** | Option<**String**> | If the container is part of Pod, the Pod ID. Requires the pod boolean to be set | [optional]
**pod_name** | Option<**String**> | If the container is part of Pod, the Pod name. Requires the pod boolean to be set | [optional]
**ports** | Option<[**Vec<models::PortMapping>**](PortMapping.md)> | Port mappings | [optional]
**restarts** | Option<**i32**> | Restarts is how many times the container was restarted by its restart policy. This is NOT incremented by normal container restarts (only by restart policy). | [optional]
**size** | Option<[**models::ContainerSize**](ContainerSize.md)> |  | [optional]
**started_at** | Option<**i64**> | Time when container started | [optional]
**state** | Option<**String**> | State of container | [optional]
**status** | Option<**String**> | Status is a human-readable approximation of a duration for json output | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


