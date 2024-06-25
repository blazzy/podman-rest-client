# PluginConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args** | [**models::PluginConfigArgs**](PluginConfigArgs.md) |  | 
**description** | **String** | description | 
**docker_version** | Option<**String**> | Docker Version used to create the plugin | [optional]
**documentation** | **String** | documentation | 
**entrypoint** | **Vec<String>** | entrypoint | 
**env** | [**Vec<models::PluginEnv>**](PluginEnv.md) | env | 
**interface** | [**models::PluginConfigInterface**](PluginConfigInterface.md) |  | 
**ipc_host** | **bool** | ipc host | 
**linux** | [**models::PluginConfigLinux**](PluginConfigLinux.md) |  | 
**mounts** | [**Vec<models::PluginMount>**](PluginMount.md) | mounts | 
**network** | [**models::PluginConfigNetwork**](PluginConfigNetwork.md) |  | 
**pid_host** | **bool** | pid host | 
**propagated_mount** | **String** | propagated mount | 
**user** | Option<[**models::PluginConfigUser**](PluginConfigUser.md)> |  | [optional]
**work_dir** | **String** | work dir | 
**rootfs** | Option<[**models::PluginConfigRootfs**](PluginConfigRootfs.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


