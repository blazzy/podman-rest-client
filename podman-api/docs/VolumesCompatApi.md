# \VolumesCompatApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**volume_create**](VolumesCompatApi.md#volume_create) | **Post** /volumes/create | Create a volume
[**volume_delete**](VolumesCompatApi.md#volume_delete) | **Delete** /volumes/{name} | Remove volume
[**volume_inspect**](VolumesCompatApi.md#volume_inspect) | **Get** /volumes/{name} | Inspect volume
[**volume_list**](VolumesCompatApi.md#volume_list) | **Get** /volumes | List volumes
[**volume_prune**](VolumesCompatApi.md#volume_prune) | **Post** /volumes/prune | Prune volumes



## volume_create

> models::Volume volume_create(create)
Create a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<[**VolumeCreate**](VolumeCreate.md)> | attributes for creating a volume. Note: If a volume by the same name exists, a 201 response with that volume's information will be generated.  |  |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_delete

> volume_delete(name, force)
Remove volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the volume | [required] |
**force** | Option<**bool**> | Force removal of the volume. This actually only causes errors due to the names volume not being found to be suppressed, which is the behaviour Docker implements.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_inspect

> models::Volume volume_inspect(name)
Inspect volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the volume | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_list

> models::ListResponse volume_list(filters)
List volumes

Returns a list of volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a map[string][]string) to process on the volumes list. Available filters:   - driver=<volume-driver-name> Matches volumes based on their driver.   - label=<key> or label=<key>:<value> Matches volumes based on the presence of a label alone or a label and a value.   - name=<volume-name> Matches all of volume name.   - `until=<timestamp>` List volumes created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.  Note:   The boolean `dangling` filter is not yet implemented for this endpoint.  |  |

### Return type

[**models::ListResponse**](ListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_prune

> models::VolumesPruneReport volume_prune(filters)
Prune volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of filters (a map[string][]string) to match volumes against before pruning. Available filters:   - `until=<timestamp>` Prune volumes created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune volumes with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**models::VolumesPruneReport**](VolumesPruneReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

