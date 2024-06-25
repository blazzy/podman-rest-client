# \NetworksCompatApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_connect**](NetworksCompatApi.md#network_connect) | **Post** /networks/{name}/connect | Connect container to network
[**network_create**](NetworksCompatApi.md#network_create) | **Post** /networks/create | Create network
[**network_delete**](NetworksCompatApi.md#network_delete) | **Delete** /networks/{name} | Remove a network
[**network_disconnect**](NetworksCompatApi.md#network_disconnect) | **Post** /networks/{name}/disconnect | Disconnect container from network
[**network_inspect**](NetworksCompatApi.md#network_inspect) | **Get** /networks/{name} | Inspect a network
[**network_list**](NetworksCompatApi.md#network_list) | **Get** /networks | List networks
[**network_prune**](NetworksCompatApi.md#network_prune) | **Post** /networks/prune | Delete unused networks



## network_connect

> network_connect(name, create)
Connect container to network

Connect a container to a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |
**create** | Option<**models::NetworkConnect**> | attributes for connecting a container to a network |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_create

> models::NetworkCreate201Response network_create(create)
Create network

Create a network configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<**models::NetworkCreateRequest**> | attributes for creating a network |  |

### Return type

[**models::NetworkCreate201Response**](NetworkCreate_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_delete

> network_delete(name)
Remove a network

Remove a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_disconnect

> network_disconnect(name, create)
Disconnect container from network

Disconnect a container from a network

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


## network_inspect

> models::NetworkResource network_inspect(name, verbose, scope)
Inspect a network

Display low level configuration network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the network | [required] |
**verbose** | Option<**bool**> | Detailed inspect output for troubleshooting |  |
**scope** | Option<**String**> | Filter the network by scope (swarm, global, or local) |  |

### Return type

[**models::NetworkResource**](NetworkResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_list

> Vec<models::NetworkResource> network_list(filters)
List networks

Display summary of network configurations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a `map[string][]string`) to process on the network list. Currently available filters:   - `name=[name]` Matches network name (accepts regex).   - `id=[id]` Matches for full or partial ID.   - `driver=[driver]` Only bridge is supported.   - `label=[key]` or `label=[key=value]` Matches networks based on the presence of a label alone or a label and a value.  |  |

### Return type

[**Vec<models::NetworkResource>**](NetworkResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_prune

> models::NetworkPrune200Response network_prune(filters)
Delete unused networks

Remove networks that do not have containers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a map[string][]string). Available filters:   - `until=<timestamp>` Prune networks created before this timestamp. The <timestamp> can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune networks with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**models::NetworkPrune200Response**](NetworkPrune_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

