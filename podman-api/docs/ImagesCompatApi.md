# \ImagesCompatApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**image_build**](ImagesCompatApi.md#image_build) | **Post** /build | Create image
[**image_create**](ImagesCompatApi.md#image_create) | **Post** /images/create | Create an image
[**image_delete**](ImagesCompatApi.md#image_delete) | **Delete** /images/{name} | Remove Image
[**image_get**](ImagesCompatApi.md#image_get) | **Get** /images/{name}/get | Export an image
[**image_get_all**](ImagesCompatApi.md#image_get_all) | **Get** /images/get | Export several images
[**image_history**](ImagesCompatApi.md#image_history) | **Get** /images/{name}/history | History of an image
[**image_inspect**](ImagesCompatApi.md#image_inspect) | **Get** /images/{name}/json | Inspect an image
[**image_list**](ImagesCompatApi.md#image_list) | **Get** /images/json | List Images
[**image_load**](ImagesCompatApi.md#image_load) | **Post** /images/load | Import image
[**image_prune**](ImagesCompatApi.md#image_prune) | **Post** /images/prune | Prune unused images
[**image_push**](ImagesCompatApi.md#image_push) | **Post** /images/{name}/push | Push Image
[**image_search**](ImagesCompatApi.md#image_search) | **Get** /images/search | Search images
[**image_tag**](ImagesCompatApi.md#image_tag) | **Post** /images/{name}/tag | Tag an image



## image_build

> models::ImageBuild200Response image_build(content_type, x_registry_config, dockerfile, t, extrahosts, remote, q, nocache, cachefrom, pull, rm, forcerm, memory, memswap, cpushares, cpusetcpus, cpuperiod, cpuquota, buildargs, shmsize, squash, labels, networkmode, platform, target, outputs, input_stream)
Create image

Build an image from the given Dockerfile(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | Option<**String**> |  |  |[default to application/x-tar]
**x_registry_config** | Option<**String**> |  |  |
**dockerfile** | Option<**String**> | Path within the build context to the `Dockerfile`. This is ignored if remote is specified and points to an external `Dockerfile`.  |  |[default to Dockerfile]
**t** | Option<**String**> | A name and optional tag to apply to the image in the `name:tag` format. If you omit the tag, the default latest value is assumed. You can provide several t parameters. |  |[default to latest]
**extrahosts** | Option<**String**> | TBD Extra hosts to add to /etc/hosts (As of version 1.xx)  |  |
**remote** | Option<**String**> | A Git repository URI or HTTP/HTTPS context URI. If the URI points to a single text file, the file’s contents are placed into a file called Dockerfile and the image is built from that file. If the URI points to a tarball, the file is downloaded by the daemon and the contents therein used as the context for the build. If the URI points to a tarball and the dockerfile parameter is also specified, there must be a file with the corresponding path inside the tarball. (As of version 1.xx)  |  |
**q** | Option<**bool**> | Suppress verbose build output  |  |[default to false]
**nocache** | Option<**bool**> | Do not use the cache when building the image (As of version 1.xx)  |  |[default to false]
**cachefrom** | Option<**String**> | JSON array of images used to build cache resolution (As of version 1.xx)  |  |
**pull** | Option<**bool**> | Attempt to pull the image even if an older image exists locally (As of version 1.xx)  |  |[default to false]
**rm** | Option<**bool**> | Remove intermediate containers after a successful build (As of version 1.xx)  |  |[default to true]
**forcerm** | Option<**bool**> | Always remove intermediate containers, even upon failure (As of version 1.xx)  |  |[default to false]
**memory** | Option<**i32**> | Memory is the upper limit (in bytes) on how much memory running containers can use (As of version 1.xx)  |  |
**memswap** | Option<**i32**> | MemorySwap limits the amount of memory and swap together (As of version 1.xx)  |  |
**cpushares** | Option<**i32**> | CPUShares (relative weight (As of version 1.xx)  |  |
**cpusetcpus** | Option<**String**> | CPUSetCPUs in which to allow execution (0-3, 0,1) (As of version 1.xx)  |  |
**cpuperiod** | Option<**i32**> | CPUPeriod limits the CPU CFS (Completely Fair Scheduler) period (As of version 1.xx)  |  |
**cpuquota** | Option<**i32**> | CPUQuota limits the CPU CFS (Completely Fair Scheduler) quota (As of version 1.xx)  |  |
**buildargs** | Option<**String**> | JSON map of string pairs denoting build-time variables. For example, the build argument `Foo` with the value of `bar` would be encoded in JSON as `[\"Foo\":\"bar\"]`.  For example, buildargs={\"Foo\":\"bar\"}.  Note(s): * This should not be used to pass secrets. * The value of buildargs should be URI component encoded before being passed to the API.  (As of version 1.xx)  |  |
**shmsize** | Option<**i32**> | ShmSize is the \"size\" value to use when mounting an shmfs on the container's /dev/shm directory. Default is 64MB (As of version 1.xx)  |  |[default to 67108864]
**squash** | Option<**bool**> | Silently ignored. Squash the resulting images layers into a single layer (As of version 1.xx)  |  |[default to false]
**labels** | Option<**String**> | JSON map of key, value pairs to set as labels on the new image (As of version 1.xx)  |  |
**networkmode** | Option<**String**> | Sets the networking mode for the run commands during build. Supported standard values are:   * `bridge` limited to containers within a single host, port mapping required for external access   * `host` no isolation between host and containers on this network   * `none` disable all networking for this container   * container:<nameOrID> share networking with given container   ---All other values are assumed to be a custom network's name (As of version 1.xx)  |  |[default to bridge]
**platform** | Option<**String**> | Platform format os[/arch[/variant]] (As of version 1.xx)  |  |
**target** | Option<**String**> | Target build stage (As of version 1.xx)  |  |
**outputs** | Option<**String**> | output configuration TBD (As of version 1.xx)  |  |
**input_stream** | Option<**std::path::PathBuf**> | A tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz.  |  |

### Return type

[**models::ImageBuild200Response**](ImageBuild_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_create

> std::path::PathBuf image_create(x_registry_auth, from_image, from_src, repo, tag, message, platform, input_image)
Create an image

Create an image by either pulling it from a registry or importing it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_registry_auth** | Option<**String**> | A base64-encoded auth configuration. |  |
**from_image** | Option<**String**> | Name of the image to pull. The name may include a tag or digest. This parameter may only be used when pulling an image. The pull is cancelled if the HTTP connection is closed. |  |
**from_src** | Option<**String**> | Source to import. The value may be a URL from which the image can be retrieved or - to read the image from the request body. This parameter may only be used when importing an image |  |
**repo** | Option<**String**> | Repository name given to an image when it is imported. The repo may include a tag. This parameter may only be used when importing an image. |  |
**tag** | Option<**String**> | Tag or digest. If empty when pulling an image, this causes all tags for the given image to be pulled. |  |
**message** | Option<**String**> | Set commit message for imported image. |  |
**platform** | Option<**String**> | Platform in the format os[/arch[/variant]] |  |
**input_image** | Option<**std::path::PathBuf**> | Image content if fromSrc parameter was used |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain, application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_delete

> Vec<models::ImageDelete200ResponseInner> image_delete(name, force, noprune)
Remove Image

Delete an image from local storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name or ID of image to delete | [required] |
**force** | Option<**bool**> | remove the image even if used by containers or has other tags |  |
**noprune** | Option<**bool**> | do not remove dangling parent images |  |

### Return type

[**Vec<models::ImageDelete200ResponseInner>**](ImageDelete_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_get

> std::path::PathBuf image_get(name)
Export an image

Export an image in tarball format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-tar

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_get_all

> std::path::PathBuf image_get_all(names)
Export several images

Get a tarball containing all images and metadata for several image repositories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**names** | **String** | one or more image names or IDs comma separated | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_history

> models::HistoryResponse image_history(name)
History of an image

Return parent layers of an image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |

### Return type

[**models::HistoryResponse**](HistoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_inspect

> models::ImageInspect image_inspect(name)
Inspect an image

Return low-level information about an image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |

### Return type

[**models::ImageInspect**](ImageInspect.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_list

> Vec<models::Summary> image_list(all, filters, digests)
List Images

Returns a list of images on the server. Note that it uses a different, smaller representation of an image than inspecting a single image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Show all images. Only images from a final layer (no children) are shown by default. |  |[default to false]
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters: - `before`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`) - `dangling=true` - `label=key` or `label=\"key=value\"` of an image label - `reference`=(`<image-name>[:<tag>]`) - `since`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`)  |  |
**digests** | Option<**bool**> | Not supported |  |[default to false]

### Return type

[**Vec<models::Summary>**](Summary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_load

> image_load(quiet, request)
Import image

Load a set of images and tags into a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quiet** | Option<**bool**> | not supported |  |
**request** | Option<**String**> | tarball of container image |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_prune

> Vec<models::ImageDelete200ResponseInner> image_prune(filters)
Prune unused images

Remove images from local storage that are not being used by a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | filters to apply to image pruning, encoded as JSON (map[string][]string). Available filters:   - `dangling=<boolean>` When set to `true` (or `1`), prune only      unused *and* untagged images. When set to `false`      (or `0`), all unused images are pruned.   - `until=<string>` Prune images created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune images with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**Vec<models::ImageDelete200ResponseInner>**](ImageDelete_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_push

> std::path::PathBuf image_push(name, tag, all, compress, destination, x_registry_auth)
Push Image

Push an image to a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of image to push. | [required] |
**tag** | Option<**String**> | The tag to associate with the image on the registry. |  |
**all** | Option<**bool**> | All indicates whether to push all images related to the image list |  |
**compress** | Option<**bool**> | use compression on image |  |
**destination** | Option<**String**> | destination name for the image being pushed |  |
**x_registry_auth** | Option<**String**> | A base64-encoded auth configuration. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_search

> models::ImageSearch200Response image_search(term, limit, filters, tls_verify, list_tags)
Search images

Search registries for an image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | Option<**String**> | term to search |  |
**limit** | Option<**i32**> | maximum number of results |  |[default to 25]
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters: - `is-automated=(true|false)` - `is-official=(true|false)` - `stars=<number>` Matches images that have at least 'number' stars.  |  |
**tls_verify** | Option<**bool**> | Require HTTPS and verify signatures when contacting registries. |  |[default to true]
**list_tags** | Option<**bool**> | list the available tags in the repository |  |

### Return type

[**models::ImageSearch200Response**](ImageSearch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_tag

> image_tag(name, repo, tag)
Tag an image

Tag an image so that it becomes part of a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**repo** | Option<**String**> | the repository to tag in |  |
**tag** | Option<**String**> | the name of the new tag |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

