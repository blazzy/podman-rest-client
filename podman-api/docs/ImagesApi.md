# \ImagesApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**image_build_libpod**](ImagesApi.md#image_build_libpod) | **Post** /libpod/build | Create image
[**image_changes_libpod**](ImagesApi.md#image_changes_libpod) | **Get** /libpod/images/{name}/changes | Report on changes to images's filesystem; adds, deletes or modifications.
[**image_delete_all_libpod**](ImagesApi.md#image_delete_all_libpod) | **Delete** /libpod/images/remove | Remove one or more images from the storage.
[**image_delete_libpod**](ImagesApi.md#image_delete_libpod) | **Delete** /libpod/images/{name} | Remove an image from the local storage.
[**image_exists_libpod**](ImagesApi.md#image_exists_libpod) | **Get** /libpod/images/{name}/exists | Image exists
[**image_export_libpod**](ImagesApi.md#image_export_libpod) | **Get** /libpod/images/export | Export multiple images
[**image_get_libpod**](ImagesApi.md#image_get_libpod) | **Get** /libpod/images/{name}/get | Export an image
[**image_history_libpod**](ImagesApi.md#image_history_libpod) | **Get** /libpod/images/{name}/history | History of an image
[**image_import_libpod**](ImagesApi.md#image_import_libpod) | **Post** /libpod/images/import | Import image
[**image_inspect_libpod**](ImagesApi.md#image_inspect_libpod) | **Get** /libpod/images/{name}/json | Inspect an image
[**image_list_libpod**](ImagesApi.md#image_list_libpod) | **Get** /libpod/images/json | List Images
[**image_load_libpod**](ImagesApi.md#image_load_libpod) | **Post** /libpod/images/load | Load image
[**image_prune_libpod**](ImagesApi.md#image_prune_libpod) | **Post** /libpod/images/prune | Prune unused images
[**image_pull_libpod**](ImagesApi.md#image_pull_libpod) | **Post** /libpod/images/pull | Pull images
[**image_push_libpod**](ImagesApi.md#image_push_libpod) | **Post** /libpod/images/{name}/push | Push Image
[**image_resolve_libpod**](ImagesApi.md#image_resolve_libpod) | **Get** /libpod/images/{name}/resolve | Resolve an image (short) name
[**image_scp_libpod**](ImagesApi.md#image_scp_libpod) | **Post** /libpod/images/scp/{name} | Copy an image from one host to another
[**image_search_libpod**](ImagesApi.md#image_search_libpod) | **Get** /libpod/images/search | Search images
[**image_tag_libpod**](ImagesApi.md#image_tag_libpod) | **Post** /libpod/images/{name}/tag | Tag an image
[**image_tree_libpod**](ImagesApi.md#image_tree_libpod) | **Get** /libpod/images/{name}/tree | Image tree
[**image_untag_libpod**](ImagesApi.md#image_untag_libpod) | **Post** /libpod/images/{name}/untag | Untag an image



## image_build_libpod

> models::ImageBuildLibpod200Response image_build_libpod(dockerfile, t, allplatforms, extrahosts, remote, q, nocache, cachefrom, pull, rm, forcerm, memory, memswap, cpushares, cpusetcpus, cpuperiod, cpuquota, buildargs, shmsize, squash, labels, layer_label, layers, networkmode, platform, target, outputs, httpproxy, unsetenv, unsetlabel, volume)
Create image

Build an image from the given Dockerfile(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dockerfile** | Option<**String**> | Path within the build context to the `Dockerfile`. This is ignored if remote is specified and points to an external `Dockerfile`.  |  |[default to Dockerfile]
**t** | Option<**String**> | A name and optional tag to apply to the image in the `name:tag` format.  If you omit the tag, the default latest value is assumed. You can provide several t parameters. |  |[default to latest]
**allplatforms** | Option<**bool**> | Instead of building for a set of platforms specified using the platform option, inspect the build's base images, and build for all of the platforms that are available.  Stages that use *scratch* as a starting point can not be inspected, so at least one non-*scratch* stage must be present for detection to work usefully.  |  |[default to false]
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
**layer_label** | Option<[**Vec<String>**](String.md)> | Add an intermediate image *label* (e.g. label=*value*) to the intermediate image metadata. |  |
**layers** | Option<**bool**> | Cache intermediate layers during build. (As of version 1.xx)  |  |[default to true]
**networkmode** | Option<**String**> | Sets the networking mode for the run commands during build. Supported standard values are:   * `bridge` limited to containers within a single host, port mapping required for external access   * `host` no isolation between host and containers on this network   * `none` disable all networking for this container   * container:<nameOrID> share networking with given container   ---All other values are assumed to be a custom network's name (As of version 1.xx)  |  |[default to bridge]
**platform** | Option<**String**> | Platform format os[/arch[/variant]] (As of version 1.xx)  |  |
**target** | Option<**String**> | Target build stage (As of version 1.xx)  |  |
**outputs** | Option<**String**> | output configuration TBD (As of version 1.xx)  |  |
**httpproxy** | Option<**bool**> | Inject http proxy environment variables into container (As of version 2.0.0)  |  |
**unsetenv** | Option<[**Vec<String>**](String.md)> | Unset environment variables from the final image. |  |
**unsetlabel** | Option<[**Vec<String>**](String.md)> | Unset the image label, causing the label not to be inherited from the base image. |  |
**volume** | Option<[**Vec<String>**](String.md)> | Extra volumes that should be mounted in the build container. |  |

### Return type

[**models::ImageBuildLibpod200Response**](ImageBuildLibpod_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_changes_libpod

> image_changes_libpod(name, parent, diff_type)
Report on changes to images's filesystem; adds, deletes or modifications.

Returns which files in an image's filesystem have been added, deleted, or modified. The Kind of modification can be one of:  0: Modified 1: Added 2: Deleted 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or id of the image | [required] |
**parent** | Option<**String**> | specify a second layer which is used to compare against it instead of the parent layer |  |
**diff_type** | Option<**String**> | select what you want to match, default is all |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_delete_all_libpod

> models::LibpodImagesRemoveReport image_delete_all_libpod(images, all, force, ignore, lookup_manifest)
Remove one or more images from the storage.

Remove one or more images from the storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**images** | Option<[**Vec<String>**](String.md)> | Images IDs or names to remove. |  |
**all** | Option<**bool**> | Remove all images. |  |[default to true]
**force** | Option<**bool**> | Force image removal (including containers using the images). |  |
**ignore** | Option<**bool**> | Ignore if a specified image does not exist and do not throw an error. |  |
**lookup_manifest** | Option<**bool**> | Resolves to manifest list instead of image. |  |

### Return type

[**models::LibpodImagesRemoveReport**](LibpodImagesRemoveReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_delete_libpod

> models::LibpodImagesRemoveReport image_delete_libpod(name, force)
Remove an image from the local storage.

Remove an image from the local storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name or ID of image to remove | [required] |
**force** | Option<**bool**> | remove the image even if used by containers or has other tags |  |

### Return type

[**models::LibpodImagesRemoveReport**](LibpodImagesRemoveReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_exists_libpod

> image_exists_libpod(name)
Image exists

Check if image exists in local store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_export_libpod

> std::path::PathBuf image_export_libpod(format, references, compress, oci_accept_uncompressed_layers)
Export multiple images

Export multiple images into a single object. Only `docker-archive` is currently supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | format for exported image (only docker-archive is supported) |  |
**references** | Option<[**Vec<String>**](String.md)> | references to images to export |  |
**compress** | Option<**bool**> | use compression on image |  |
**oci_accept_uncompressed_layers** | Option<**bool**> | accept uncompressed layers when copying OCI images |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_get_libpod

> std::path::PathBuf image_get_libpod(name, format, compress)
Export an image

Export an image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**format** | Option<**String**> | format for exported image |  |
**compress** | Option<**bool**> | use compression on image |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-tar

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_history_libpod

> models::HistoryResponse image_history_libpod(name)
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


## image_import_libpod

> models::ImageImportReport image_import_libpod(upload, content_type, changes, message, reference, url)
Import image

Import a previously exported tarball as an image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload** | **std::path::PathBuf** | tarball for imported image | [required] |
**content_type** | Option<**String**> |  |  |[default to application/x-tar]
**changes** | Option<[**Vec<String>**](String.md)> | Apply the following possible instructions to the created image: CMD | ENTRYPOINT | ENV | EXPOSE | LABEL | STOPSIGNAL | USER | VOLUME | WORKDIR.  JSON encoded string |  |
**message** | Option<**String**> | Set commit message for imported image |  |
**reference** | Option<**String**> | Optional Name[:TAG] for the image |  |
**url** | Option<**String**> | Load image from the specified URL |  |

### Return type

[**models::ImageImportReport**](ImageImportReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_inspect_libpod

> models::ImageData image_inspect_libpod(name)
Inspect an image

Obtain low-level information about an image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |

### Return type

[**models::ImageData**](ImageData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_list_libpod

> Vec<models::LibpodImageSummary> image_list_libpod(all, filters)
List Images

Returns a list of images on the server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Show all images. Only images from a final layer (no children) are shown by default. |  |[default to false]
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters: - `before`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`) - `dangling=true` - `label=key` or `label=\"key=value\"` of an image label - `reference`=(`<image-name>[:<tag>]`) - `id`=(`<image-id>`) - `since`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`)  |  |

### Return type

[**Vec<models::LibpodImageSummary>**](LibpodImageSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_load_libpod

> models::ImageLoadReport image_load_libpod(upload)
Load image

Load an image (oci-archive or docker-archive) stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload** | **String** | tarball of container image | [required] |

### Return type

[**models::ImageLoadReport**](ImageLoadReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_prune_libpod

> Vec<models::PruneReport> image_prune_libpod(all, external, filters)
Prune unused images

Remove images that are not being used by a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Remove all images not in use by containers, not just dangling ones  |  |[default to false]
**external** | Option<**bool**> | Remove images even when they are used by external containers (e.g, by build containers)  |  |[default to false]
**filters** | Option<**String**> | filters to apply to image pruning, encoded as JSON (map[string][]string). Available filters:   - `dangling=<boolean>` When set to `true` (or `1`), prune only      unused *and* untagged images. When set to `false`      (or `0`), all unused images are pruned.   - `until=<string>` Prune images created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune images with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**Vec<models::PruneReport>**](PruneReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_pull_libpod

> models::LibpodImagesPullReport image_pull_libpod(reference, quiet, compat_mode, arch, os, variant, policy, tls_verify, all_tags, x_registry_auth)
Pull images

Pull one or more images from a container registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference** | Option<**String**> | Mandatory reference to the image (e.g., quay.io/image/name:tag) |  |
**quiet** | Option<**bool**> | silences extra stream data on pull |  |[default to false]
**compat_mode** | Option<**bool**> | Return the same JSON payload as the Docker-compat endpoint. |  |[default to false]
**arch** | Option<**String**> | Pull image for the specified architecture. |  |
**os** | Option<**String**> | Pull image for the specified operating system. |  |
**variant** | Option<**String**> | Pull image for the specified variant. |  |
**policy** | Option<**String**> | Pull policy, \"always\" (default), \"missing\", \"newer\", \"never\". |  |
**tls_verify** | Option<**bool**> | Require TLS verification. |  |[default to true]
**all_tags** | Option<**bool**> | Pull all tagged images in the repository. |  |
**x_registry_auth** | Option<**String**> | base-64 encoded auth config. Must include the following four values: username, password, email and server address OR simply just an identity token. |  |

### Return type

[**models::LibpodImagesPullReport**](LibpodImagesPullReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_push_libpod

> std::path::PathBuf image_push_libpod(name, destination, force_compression_format, tls_verify, quiet, x_registry_auth)
Push Image

Push an image to a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of image to push. | [required] |
**destination** | Option<**String**> | Allows for pushing the image to a different destination than the image refers to. |  |
**force_compression_format** | Option<**bool**> | Enforce compressing the layers with the specified --compression and do not reuse differently compressed blobs on the registry. |  |[default to false]
**tls_verify** | Option<**bool**> | Require TLS verification. |  |[default to true]
**quiet** | Option<**bool**> | silences extra stream data on push |  |[default to true]
**x_registry_auth** | Option<**String**> | A base64-encoded auth configuration. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_resolve_libpod

> image_resolve_libpod(name)
Resolve an image (short) name

Resolve the passed image name to a list of fully-qualified images referring to container registries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the (short) name to resolve | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_scp_libpod

> models::ScpReport image_scp_libpod(name, destination, quiet)
Copy an image from one host to another

Copy an image from one host to another

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | source connection/image | [required] |
**destination** | Option<**String**> | dest connection/image |  |
**quiet** | Option<**bool**> | quiet output |  |[default to false]

### Return type

[**models::ScpReport**](ScpReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_search_libpod

> models::ImageSearch200Response image_search_libpod(term, limit, filters, tls_verify, list_tags)
Search images

Search registries for images

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | Option<**String**> | term to search |  |
**limit** | Option<**i32**> | maximum number of results |  |[default to 25]
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters: - `is-automated=(true|false)` - `is-official=(true|false)` - `stars=<number>` Matches images that have at least 'number' stars.  |  |
**tls_verify** | Option<**bool**> | Require HTTPS and verify signatures when contacting registries. |  |[default to true]
**list_tags** | Option<**bool**> | list the available tags in the repository |  |[default to false]

### Return type

[**models::ImageSearch200Response**](ImageSearch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_tag_libpod

> image_tag_libpod(name, repo, tag)
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


## image_tree_libpod

> models::ImageTreeReport image_tree_libpod(name, whatrequires)
Image tree

Retrieve the image tree for the provided image name or ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**whatrequires** | Option<**bool**> | show all child images and layers of the specified image |  |

### Return type

[**models::ImageTreeReport**](ImageTreeReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_untag_libpod

> image_untag_libpod(name, repo, tag)
Untag an image

Untag an image. If not repo and tag are specified, all tags are removed from the image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**repo** | Option<**String**> | the repository to untag |  |
**tag** | Option<**String**> | the name of the tag to untag |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

