# \NetworksApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_connect_libpod**](NetworksApi.md#network_connect_libpod) | **Post** /libpod/networks/{name}/connect | Connect container to network
[**network_create_libpod**](NetworksApi.md#network_create_libpod) | **Post** /libpod/networks/create | Create network
[**network_delete_libpod**](NetworksApi.md#network_delete_libpod) | **Delete** /libpod/networks/{name} | Remove a network
[**network_disconnect_libpod**](NetworksApi.md#network_disconnect_libpod) | **Post** /libpod/networks/{name}/disconnect | Disconnect container from network
[**network_exists_libpod**](NetworksApi.md#network_exists_libpod) | **Get** /libpod/networks/{name}/exists | Network exists
[**network_inspect_libpod**](NetworksApi.md#network_inspect_libpod) | **Get** /libpod/networks/{name}/json | Inspect a network
[**network_list_libpod**](NetworksApi.md#network_list_libpod) | **Get** /libpod/networks/json | List networks
[**network_prune_libpod**](NetworksApi.md#network_prune_libpod) | **Post** /libpod/networks/prune | Delete unused networks
[**network_update_libpod**](NetworksApi.md#network_update_libpod) | **Post** /libpod/networks/{name}/update | Update existing podman network



## network_connect_libpod

> network_connect_libpod(name, create)
Connect container to network

Connect a container to a network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |
**create** | Option<**models::NetworkConnectOptions**> | attributes for connecting a container to a network |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_create_libpod

> models::Network network_create_libpod(create)
Create network

Create a new network configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<[**NetworkCreateLibpod**](NetworkCreateLibpod.md)> | attributes for creating a network |  |

### Return type

[**models::Network**](Network.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_delete_libpod

> Vec<models::NetworkRmReport> network_delete_libpod(name, force)
Remove a network

Remove a configured network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |
**force** | Option<**bool**> | remove containers associated with network |  |

### Return type

[**Vec<models::NetworkRmReport>**](NetworkRmReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_disconnect_libpod

> network_disconnect_libpod(name, create)
Disconnect container from network

Disconnect a container from a network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |
**create** | Option<**models::NetworkDisconnect**> | attributes for disconnecting a container from a network |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_exists_libpod

> network_exists_libpod(name)
Network exists

Check if network exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the network | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_inspect_libpod

> models::NetworkInspectReport network_inspect_libpod(name)
Inspect a network

Display configuration for a network. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |

### Return type

[**models::NetworkInspectReport**](NetworkInspectReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_list_libpod

> Vec<models::Network> network_list_libpod(filters)
List networks

Display summary of network configurations.   - In a 200 response, all of the fields named Bytes are returned as a Base64 encoded string. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a `map[string][]string`) to process on the network list. Available filters:   - `name=[name]` Matches network name (accepts regex).   - `id=[id]` Matches for full or partial ID.   - `driver=[driver]` Only bridge is supported.   - `label=[key]` or `label=[key=value]` Matches networks based on the presence of a label alone or a label and a value.   - `until=[timestamp]` Matches all networks that were created before the given timestamp.  |  |

### Return type

[**Vec<models::Network>**](Network.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_prune_libpod

> Vec<models::NetworkPruneReport> network_prune_libpod(filters)
Delete unused networks

Remove networks that do not have containers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a `map[string][]string`). Available filters:   - `until=<timestamp>` Prune networks created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune networks with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**Vec<models::NetworkPruneReport>**](NetworkPruneReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_update_libpod

> network_update_libpod(name, update)
Update existing podman network

Update existing podman network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the network | [required] |
**update** | Option<**models::NetworkUpdateOptions**> | attributes for updating a netavark network |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

