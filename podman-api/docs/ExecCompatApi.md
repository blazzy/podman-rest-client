# \ExecCompatApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**container_exec**](ExecCompatApi.md#container_exec) | **Post** /containers/{name}/exec | Create an exec instance
[**exec_inspect**](ExecCompatApi.md#exec_inspect) | **Get** /exec/{id}/json | Inspect an exec instance
[**exec_resize**](ExecCompatApi.md#exec_resize) | **Post** /exec/{id}/resize | Resize an exec instance
[**exec_start**](ExecCompatApi.md#exec_start) | **Post** /exec/{id}/start | Start an exec instance



## container_exec

> container_exec(name, control)
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


## exec_inspect

> models::InspectExecSession exec_inspect(id)
Inspect an exec instance

Return low-level information about an exec instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |

### Return type

[**models::InspectExecSession**](InspectExecSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_resize

> exec_resize(id, h, w, running)
Resize an exec instance

Resize the TTY session used by an exec instance. This endpoint only works if tty was specified as part of creating and starting the exec instance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |
**h** | Option<**i32**> | Height of the TTY session in characters |  |
**w** | Option<**i32**> | Width of the TTY session in characters |  |
**running** | Option<**bool**> | Ignore containers not running errors |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_start

> exec_start(id, control)
Start an exec instance

Starts a previously set up exec instance. If detach is true, this endpoint returns immediately after starting the command. Otherwise, it sets up an interactive session with the command.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Exec instance ID | [required] |
**control** | Option<[**ExecStartRequest**](ExecStartRequest.md)> | Attributes for start |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

