# \SecretsCompatApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secret_create**](SecretsCompatApi.md#secret_create) | **Post** /secrets/create | Create a secret
[**secret_delete**](SecretsCompatApi.md#secret_delete) | **Delete** /secrets/{name} | Remove secret
[**secret_inspect**](SecretsCompatApi.md#secret_inspect) | **Get** /secrets/{name} | Inspect secret
[**secret_list**](SecretsCompatApi.md#secret_list) | **Get** /secrets | List secrets



## secret_create

> models::SecretCreateLibpod201Response secret_create(create)
Create a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<[**SecretCreate**](SecretCreate.md)> | attributes for creating a secret  |  |

### Return type

[**models::SecretCreateLibpod201Response**](SecretCreateLibpod_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_delete

> secret_delete(name)
Remove secret

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


## secret_inspect

> models::SecretInfoReportCompat secret_inspect(name)
Inspect secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the secret | [required] |

### Return type

[**models::SecretInfoReportCompat**](SecretInfoReportCompat.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_list

> Vec<models::SecretInfoReportCompat> secret_list(filters)
List secrets

Returns a list of secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a `map[string][]string`) to process on the secrets list. Currently available filters:   - `name=[name]` Matches secrets name (accepts regex).   - `id=[id]` Matches for full or partial ID.  |  |

### Return type

[**Vec<models::SecretInfoReportCompat>**](SecretInfoReportCompat.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

