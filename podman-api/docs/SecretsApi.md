# \SecretsApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secret_create_libpod**](SecretsApi.md#secret_create_libpod) | **Post** /libpod/secrets/create | Create a secret
[**secret_delete_libpod**](SecretsApi.md#secret_delete_libpod) | **Delete** /libpod/secrets/{name} | Remove secret
[**secret_exists_libpod**](SecretsApi.md#secret_exists_libpod) | **Get** /libpod/secrets/{name}/exists | Secret exists
[**secret_inspect_libpod**](SecretsApi.md#secret_inspect_libpod) | **Get** /libpod/secrets/{name}/json | Inspect secret
[**secret_list_libpod**](SecretsApi.md#secret_list_libpod) | **Get** /libpod/secrets/json | List secrets



## secret_create_libpod

> models::SecretCreateLibpod201Response secret_create_libpod(name, driver, driveropts, labels, request)
Create a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | User-defined name of the secret. | [required] |
**driver** | Option<**String**> | Secret driver |  |[default to file]
**driveropts** | Option<**String**> | Secret driver options |  |
**labels** | Option<**String**> | Labels on the secret |  |
**request** | Option<**String**> | Secret |  |

### Return type

[**models::SecretCreateLibpod201Response**](SecretCreateLibpod_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_delete_libpod

> secret_delete_libpod(name, all)
Remove secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the secret | [required] |
**all** | Option<**bool**> | Remove all secrets |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_exists_libpod

> secret_exists_libpod(name)
Secret exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the secret | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_inspect_libpod

> models::SecretInfoReport secret_inspect_libpod(name, showsecret)
Inspect secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the secret | [required] |
**showsecret** | Option<**bool**> | Display Secret |  |[default to false]

### Return type

[**models::SecretInfoReport**](SecretInfoReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_list_libpod

> Vec<models::SecretInfoReport> secret_list_libpod(filters)
List secrets

Returns a list of secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a `map[string][]string`) to process on the secrets list. Currently available filters:   - `name=[name]` Matches secrets name (accepts regex).   - `id=[id]` Matches for full or partial ID.  |  |

### Return type

[**Vec<models::SecretInfoReport>**](SecretInfoReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

