# \ManifestsApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**manifest_add_libpod**](ManifestsApi.md#manifest_add_libpod) | **Post** /libpod/manifests/{name}/add | Add image
[**manifest_create_libpod**](ManifestsApi.md#manifest_create_libpod) | **Post** /libpod/manifests/{name} | Create
[**manifest_delete_libpod**](ManifestsApi.md#manifest_delete_libpod) | **Delete** /libpod/manifests/{name} | Delete manifest list
[**manifest_exists_libpod**](ManifestsApi.md#manifest_exists_libpod) | **Get** /libpod/manifests/{name}/exists | Exists
[**manifest_inspect_libpod**](ManifestsApi.md#manifest_inspect_libpod) | **Get** /libpod/manifests/{name}/json | Inspect
[**manifest_modify_libpod**](ManifestsApi.md#manifest_modify_libpod) | **Put** /libpod/manifests/{name} | Modify manifest list
[**manifest_push_libpod**](ManifestsApi.md#manifest_push_libpod) | **Post** /libpod/manifests/{name}/registry/{destination} | Push manifest list to registry
[**manifest_push_v3_libpod**](ManifestsApi.md#manifest_push_v3_libpod) | **Post** /libpod/manifests/{name}/push | Push manifest to registry



## manifest_add_libpod

> models::IdResponse manifest_add_libpod(name, options)
Add image

Add an image to a manifest list  Deprecated: As of 4.0.0 use ManifestModifyLibpod instead 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the manifest | [required] |
**options** | Option<[**ManifestAddOptions**](ManifestAddOptions.md)> | options for creating a manifest |  |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_create_libpod

> models::IdResponse manifest_create_libpod(name, images, all, amend, options)
Create

Create a manifest list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | manifest list or index name to create | [required] |
**images** | **String** | One or more names of an image or a manifest list. Repeat parameter as needed.  Support for multiple images, as of version 4.0.0 Alias of `image` is support for compatibility with < 4.0.0 Response status code is 200 with < 4.0.0 for compatibility  | [required] |
**all** | Option<**bool**> | add all contents if given list |  |
**amend** | Option<**bool**> | modify an existing list if one with the desired name already exists |  |
**options** | Option<[**ManifestModifyOptions**](ManifestModifyOptions.md)> | options for new manifest |  |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_delete_libpod

> models::LibpodImagesRemoveReport manifest_delete_libpod(name)
Delete manifest list

Delete named manifest list  As of v4.0.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name or ID of the  list to be deleted | [required] |

### Return type

[**models::LibpodImagesRemoveReport**](LibpodImagesRemoveReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_exists_libpod

> manifest_exists_libpod(name)
Exists

Check if manifest list exists  Note: There is no contract that the manifest list will exist for a follow-on operation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the manifest list | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_inspect_libpod

> models::Schema2ListPublic manifest_inspect_libpod(name, tls_verify)
Inspect

Display attributes of given manifest list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the manifest list | [required] |
**tls_verify** | Option<**bool**> | Require HTTPS and verify signatures when contacting registries. |  |[default to true]

### Return type

[**models::Schema2ListPublic**](Schema2ListPublic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_modify_libpod

> models::ManifestModifyReport manifest_modify_libpod(name, options, tls_verify)
Modify manifest list

Add/Remove an image(s) to a manifest list  Note: operations are not atomic when multiple Images are provided.  As of v4.0.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the manifest | [required] |
**options** | [**ManifestModifyOptions**](ManifestModifyOptions.md) | options for mutating a manifest | [required] |
**tls_verify** | Option<**bool**> | Require HTTPS and verify signatures when contacting registries. |  |[default to true]

### Return type

[**models::ManifestModifyReport**](ManifestModifyReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_push_libpod

> models::IdResponse manifest_push_libpod(name, destination, add_compression, force_compression_format, all, tls_verify, quiet)
Push manifest list to registry

Push a manifest list or image index to the named registry  As of v4.0.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the manifest list | [required] |
**destination** | **String** | the registry for the manifest list | [required] |
**add_compression** | Option<[**Vec<String>**](String.md)> | add existing instances with requested compression algorithms to manifest list |  |
**force_compression_format** | Option<**bool**> | Enforce compressing the layers with the specified --compression and do not reuse differently compressed blobs on the registry. |  |[default to false]
**all** | Option<**bool**> | push all images |  |[default to true]
**tls_verify** | Option<**bool**> | Require HTTPS and verify signatures when contacting registries. |  |[default to true]
**quiet** | Option<**bool**> | silences extra stream data on push |  |[default to true]

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest_push_v3_libpod

> models::IdResponse manifest_push_v3_libpod(name, destination, all)
Push manifest to registry

Push a manifest list or image index to a registry  Deprecated: As of 4.0.0 use ManifestPushLibpod instead 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the manifest | [required] |
**destination** | **String** | the destination for the manifest | [required] |
**all** | Option<**bool**> | push all images |  |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

