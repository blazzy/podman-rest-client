# CreateContainerConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args_escaped** | Option<**bool**> |  | [optional]
**attach_stderr** | Option<**bool**> |  | [optional]
**attach_stdin** | Option<**bool**> |  | [optional]
**attach_stdout** | Option<**bool**> |  | [optional]
**cmd** | Option<**Vec<String>**> | We need to override the json decoder to accept both options. | [optional]
**domainname** | Option<**String**> |  | [optional]
**entrypoint** | Option<**Vec<String>**> | We need to override the json decoder to accept both options. | [optional]
**env** | Option<**Vec<String>**> |  | [optional]
**env_merge** | Option<**Vec<String>**> |  | [optional]
**exposed_ports** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | PortSet is a collection of structs indexed by Port | [optional]
**healthcheck** | Option<[**models::HealthcheckConfig**](HealthcheckConfig.md)> |  | [optional]
**host_config** | Option<[**models::HostConfig**](HostConfig.md)> |  | [optional]
**hostname** | Option<**String**> |  | [optional]
**image** | Option<**String**> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**mac_address** | Option<**String**> | Mac Address of the container.  Deprecated: this field is deprecated since API v1.44. Use EndpointSettings.MacAddress instead. | [optional]
**name** | Option<**String**> |  | [optional]
**network_disabled** | Option<**bool**> |  | [optional]
**networking_config** | Option<[**models::NetworkingConfig**](NetworkingConfig.md)> |  | [optional]
**on_build** | Option<**Vec<String>**> |  | [optional]
**open_stdin** | Option<**bool**> |  | [optional]
**shell** | Option<**Vec<String>**> | We need to override the json decoder to accept both options. | [optional]
**stdin_once** | Option<**bool**> |  | [optional]
**stop_signal** | Option<**String**> |  | [optional]
**stop_timeout** | Option<**i64**> |  | [optional]
**tty** | Option<**bool**> |  | [optional]
**unset_env** | Option<**Vec<String>**> |  | [optional]
**unset_env_all** | Option<**bool**> |  | [optional]
**user** | Option<**String**> |  | [optional]
**volumes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**working_dir** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


