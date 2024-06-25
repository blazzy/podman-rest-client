# HostInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arch** | Option<**String**> |  | [optional]
**buildah_version** | Option<**String**> |  | [optional]
**cgroup_controllers** | Option<**Vec<String>**> |  | [optional]
**cgroup_manager** | Option<**String**> |  | [optional]
**cgroup_version** | Option<**String**> |  | [optional]
**conmon** | Option<[**models::ConmonInfo**](ConmonInfo.md)> |  | [optional]
**cpu_utilization** | Option<[**models::CpuUsage**](CPUUsage.md)> |  | [optional]
**cpus** | Option<**i64**> |  | [optional]
**database_backend** | Option<**String**> |  | [optional]
**distribution** | Option<[**models::DistributionInfo**](DistributionInfo.md)> |  | [optional]
**event_logger** | Option<**String**> |  | [optional]
**free_locks** | Option<**i32**> |  | [optional]
**hostname** | Option<**String**> |  | [optional]
**id_mappings** | Option<[**models::IdMappings**](IDMappings.md)> |  | [optional]
**kernel** | Option<**String**> |  | [optional]
**linkmode** | Option<**String**> |  | [optional]
**log_driver** | Option<**String**> |  | [optional]
**mem_free** | Option<**i64**> |  | [optional]
**mem_total** | Option<**i64**> |  | [optional]
**network_backend** | Option<**String**> |  | [optional]
**network_backend_info** | Option<[**models::NetworkInfo**](NetworkInfo.md)> |  | [optional]
**oci_runtime** | Option<[**models::OciRuntimeInfo**](OCIRuntimeInfo.md)> |  | [optional]
**os** | Option<**String**> |  | [optional]
**pasta** | Option<[**models::PastaInfo**](PastaInfo.md)> |  | [optional]
**remote_socket** | Option<[**models::RemoteSocket**](RemoteSocket.md)> |  | [optional]
**rootless_network_cmd** | Option<**String**> | RootlessNetworkCmd returns the default rootless network command (slirp4netns or pasta) | [optional]
**runtime_info** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**security** | Option<[**models::SecurityInfo**](SecurityInfo.md)> |  | [optional]
**service_is_remote** | Option<**bool**> | ServiceIsRemote is true when the podman/libpod service is remote to the client | [optional]
**slirp4netns** | Option<[**models::SlirpInfo**](SlirpInfo.md)> |  | [optional]
**swap_free** | Option<**i64**> |  | [optional]
**swap_total** | Option<**i64**> |  | [optional]
**uptime** | Option<**String**> |  | [optional]
**variant** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


