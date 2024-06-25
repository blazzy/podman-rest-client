# \VolumesApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**volume_create_libpod**](VolumesApi.md#volume_create_libpod) | **Post** /libpod/volumes/create | Create a volume
[**volume_delete_libpod**](VolumesApi.md#volume_delete_libpod) | **Delete** /libpod/volumes/{name} | Remove volume
[**volume_exists_libpod**](VolumesApi.md#volume_exists_libpod) | **Get** /libpod/volumes/{name}/exists | Volume exists
[**volume_inspect_libpod**](VolumesApi.md#volume_inspect_libpod) | **Get** /libpod/volumes/{name}/json | Inspect volume
[**volume_list_libpod**](VolumesApi.md#volume_list_libpod) | **Get** /libpod/volumes/json | List volumes
[**volume_prune_libpod**](VolumesApi.md#volume_prune_libpod) | **Post** /libpod/volumes/prune | Prune volumes



## volume_create_libpod

> models::VolumeConfigResponse volume_create_libpod(create)
Create a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<[**VolumeCreateOptions**](VolumeCreateOptions.md)> | attributes for creating a volume |  |

### Return type

[**models::VolumeConfigResponse**](VolumeConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_delete_libpod

> volume_delete_libpod(name, force)
Remove volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the volume | [required] |
**force** | Option<**bool**> | force removal |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_exists_libpod

> volume_exists_libpod(name)
Volume exists

Check if a volume exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name of the volume | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_inspect_libpod

> models::VolumeConfigResponse volume_inspect_libpod(name)
Inspect volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the volume | [required] |

### Return type

[**models::VolumeConfigResponse**](VolumeConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_list_libpod

> Vec<models::VolumeConfigResponse> volume_list_libpod(filters)
List volumes

Returns a list of volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a map[string][]string) to process on the volumes list. Available filters:   - driver=<volume-driver-name> Matches volumes based on their driver.   - label=<key> or label=<key>:<value> Matches volumes based on the presence of a label alone or a label and a value.   - name=<volume-name> Matches all of volume name.   - opt=<driver-option> Matches a storage driver options   - `until=<timestamp>` List volumes created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.  |  |

### Return type

[**Vec<models::VolumeConfigResponse>**](VolumeConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_prune_libpod

> Vec<models::PruneReport> volume_prune_libpod(filters)
Prune volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of filters (a map[string][]string) to match volumes against before pruning. Available filters:   - `until=<timestamp>` Prune volumes created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune volumes with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**Vec<models::PruneReport>**](PruneReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

