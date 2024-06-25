# \ExecApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**container_exec_libpod**](ExecApi.md#container_exec_libpod) | **Post** /libpod/containers/{name}/exec | Create an exec instance
[**exec_inspect_libpod**](ExecApi.md#exec_inspect_libpod) | **Get** /libpod/exec/{id}/json | Inspect an exec instance
[**exec_resize_libpod**](ExecApi.md#exec_resize_libpod) | **Post** /libpod/exec/{id}/resize | Resize an exec instance
[**exec_start_libpod**](ExecApi.md#exec_start_libpod) | **Post** /libpod/exec/{id}/start | Start an exec instance



## container_exec_libpod

> container_exec_libpod(name, control)
Create an exec instance

Create an exec session to run a command inside a running container. Exec sessions will be automatically removed 5 minutes after they exit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of container | [required] |
**control** | Option<[**ContainerExecRequest**](ContainerExecRequest.md)> | Attributes for create |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_inspect_libpod

> exec_inspect_libpod(id)
Inspect an exec instance

Return low-level information about an exec instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_resize_libpod

> exec_resize_libpod(id, h, w)
Resize an exec instance

Resize the TTY session used by an exec instance. This endpoint only works if tty was specified as part of creating and starting the exec instance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |
**h** | Option<**i32**> | Height of the TTY session in characters |  |
**w** | Option<**i32**> | Width of the TTY session in characters |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_start_libpod

> exec_start_libpod(id, control)
Start an exec instance

Starts a previously set up exec instance. If detach is true, this endpoint returns immediately after starting the command. Otherwise, it sets up an interactive session with the command. The stream format is the same as the attach endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |
**control** | Option<[**ExecStartLibpodRequest**](ExecStartLibpodRequest.md)> | Attributes for start |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

