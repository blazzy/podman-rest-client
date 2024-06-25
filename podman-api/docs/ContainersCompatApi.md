# \ContainersCompatApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**container_archive**](ContainersCompatApi.md#container_archive) | **Get** /containers/{name}/archive | Get files from a container
[**container_archive_libpod**](ContainersCompatApi.md#container_archive_libpod) | **Get** /libpod/containers/{name}/archive | Copy files from a container
[**container_attach**](ContainersCompatApi.md#container_attach) | **Post** /containers/{name}/attach | Attach to a container
[**container_changes_libpod**](ContainersCompatApi.md#container_changes_libpod) | **Get** /libpod/containers/{name}/changes | Report on changes to container's filesystem; adds, deletes or modifications.
[**container_create**](ContainersCompatApi.md#container_create) | **Post** /containers/create | Create a container
[**container_delete**](ContainersCompatApi.md#container_delete) | **Delete** /containers/{name} | Remove a container
[**container_export**](ContainersCompatApi.md#container_export) | **Get** /containers/{name}/export | Export a container
[**container_inspect**](ContainersCompatApi.md#container_inspect) | **Get** /containers/{name}/json | Inspect container
[**container_kill**](ContainersCompatApi.md#container_kill) | **Post** /containers/{name}/kill | Kill container
[**container_list**](ContainersCompatApi.md#container_list) | **Get** /containers/json | List containers
[**container_logs**](ContainersCompatApi.md#container_logs) | **Get** /containers/{name}/logs | Get container logs
[**container_pause**](ContainersCompatApi.md#container_pause) | **Post** /containers/{name}/pause | Pause container
[**container_prune**](ContainersCompatApi.md#container_prune) | **Post** /containers/prune | Delete stopped containers
[**container_rename**](ContainersCompatApi.md#container_rename) | **Post** /containers/{name}/rename | Rename an existing container
[**container_resize**](ContainersCompatApi.md#container_resize) | **Post** /containers/{name}/resize | Resize a container's TTY
[**container_restart**](ContainersCompatApi.md#container_restart) | **Post** /containers/{name}/restart | Restart container
[**container_start**](ContainersCompatApi.md#container_start) | **Post** /containers/{name}/start | Start a container
[**container_stats**](ContainersCompatApi.md#container_stats) | **Get** /containers/{name}/stats | Get stats for a container
[**container_stop**](ContainersCompatApi.md#container_stop) | **Post** /containers/{name}/stop | Stop a container
[**container_top**](ContainersCompatApi.md#container_top) | **Get** /containers/{name}/top | List processes running inside a container
[**container_unpause**](ContainersCompatApi.md#container_unpause) | **Post** /containers/{name}/unpause | Unpause container
[**container_update**](ContainersCompatApi.md#container_update) | **Post** /containers/{name}/update | Update configuration of an existing container
[**container_wait**](ContainersCompatApi.md#container_wait) | **Post** /containers/{name}/wait | Wait on a container
[**image_commit**](ContainersCompatApi.md#image_commit) | **Post** /commit | New Image
[**put_container_archive**](ContainersCompatApi.md#put_container_archive) | **Put** /containers/{name}/archive | Put files into a container



## container_archive

> std::path::PathBuf container_archive(name, path)
Get files from a container

Get a tar archive of files from a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | container name or id | [required] |
**path** | **String** | Path to a directory in the container to extract | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_archive_libpod

> std::path::PathBuf container_archive_libpod(name, path, rename)
Copy files from a container

Copy a tar archive of files from a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | container name or id | [required] |
**path** | **String** | Path to a directory in the container to extract | [required] |
**rename** | Option<**String**> | JSON encoded map[string]string to translate paths |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_attach

> container_attach(name, detach_keys, logs, stream, stdout, stderr, stdin)
Attach to a container

Attach to a container to read its output or send it input. You can attach to the same container multiple times and you can reattach to containers that have been detached.  It uses the same stream format as docker, see the libpod attach endpoint for a description of the format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**detach_keys** | Option<**String**> | keys to use for detaching from the container |  |
**logs** | Option<**bool**> | Stream all logs from the container across the connection. Happens before streaming attach (if requested). At least one of logs or stream must be set |  |
**stream** | Option<**bool**> | Attach to the container. If unset, and logs is set, only the container's logs will be sent. At least one of stream or logs must be set |  |[default to true]
**stdout** | Option<**bool**> | Attach to container STDOUT |  |
**stderr** | Option<**bool**> | Attach to container STDERR |  |
**stdin** | Option<**bool**> | Attach to container STDIN |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_changes_libpod

> container_changes_libpod(name, parent, diff_type)
Report on changes to container's filesystem; adds, deletes or modifications.

Returns which files in a container's filesystem have been added, deleted, or modified. The Kind of modification can be one of:  0: Modified 1: Added 2: Deleted 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or id of the container | [required] |
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


## container_create

> models::ContainerCreateResponse container_create(body, name)
Create a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateContainerConfig**](CreateContainerConfig.md) | Container to create | [required] |
**name** | Option<**String**> | container name |  |

### Return type

[**models::ContainerCreateResponse**](ContainerCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_delete

> container_delete(name, force, v, link)
Remove a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**force** | Option<**bool**> | If the container is running, kill it before removing it. |  |[default to false]
**v** | Option<**bool**> | Remove the volumes associated with the container. |  |[default to false]
**link** | Option<**bool**> | not supported |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_export

> container_export(name)
Export a container

Export the contents of a container as a tarball.

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


## container_inspect

> models::ContainerJson container_inspect(name, size)
Inspect container

Return low-level information about a container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or id of the container | [required] |
**size** | Option<**bool**> | include the size of the container |  |[default to false]

### Return type

[**models::ContainerJson**](ContainerJSON.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_kill

> container_kill(name, all, signal)
Kill container

Signal to send to the container as an integer or string (e.g. SIGINT)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**all** | Option<**bool**> | Send kill signal to all containers |  |[default to false]
**signal** | Option<**String**> | signal to be sent to container |  |[default to SIGKILL]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_list

> Vec<models::Container> container_list(all, external, limit, size, filters)
List containers

Returns a list of containers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Return all containers. By default, only running containers are shown |  |[default to false]
**external** | Option<**bool**> | Return containers in storage not controlled by Podman |  |[default to false]
**limit** | Option<**i32**> | Return this number of most recently created containers, including non-running ones. |  |
**size** | Option<**bool**> | Return the size of container as fields SizeRw and SizeRootFs. |  |[default to false]
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the containers list. Available filters: - `ancestor`=(`<image-name>[:<tag>]`, `<image id>`, or `<image@digest>`) - `before`=(`<container id>` or `<container name>`) - `expose`=(`<port>[/<proto>]` or `<startport-endport>/[<proto>]`) - `exited=<int>` containers with exit code of `<int>` - `health`=(`starting`, `healthy`, `unhealthy` or `none`) - `id=<ID>` a container's ID - `is-task`=(`true` or `false`) - `label`=(`key` or `\"key=value\"`) of a container label - `name=<name>` a container's name - `network`=(`<network id>` or `<network name>`) - `publish`=(`<port>[/<proto>]` or `<startport-endport>/[<proto>]`) - `since`=(`<container id>` or `<container name>`) - `status`=(`created`, `restarting`, `running`, `removing`, `paused`, `exited` or `dead`) - `volume`=(`<volume name>` or `<mount point destination>`)  |  |

### Return type

[**Vec<models::Container>**](Container.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_logs

> container_logs(name, follow, stdout, stderr, since, until, timestamps, tail)
Get container logs

Get stdout and stderr logs from a container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**follow** | Option<**bool**> | Keep connection after returning logs. |  |
**stdout** | Option<**bool**> | Return logs from stdout |  |
**stderr** | Option<**bool**> | Return logs from stderr |  |
**since** | Option<**String**> | Only return logs since this time, as a UNIX timestamp |  |
**until** | Option<**String**> | Only return logs before this time, as a UNIX timestamp |  |
**timestamps** | Option<**bool**> | Add timestamps to every log line |  |[default to false]
**tail** | Option<**String**> | Only return this number of log lines from the end of the logs |  |[default to all]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_pause

> container_pause(name)
Pause container

Use the cgroups freezer to suspend all processes in a container.

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


## container_prune

> Vec<models::ContainersPruneReport> container_prune(filters)
Delete stopped containers

Remove containers not in use

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters:  - `until=<timestamp>` Prune containers created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.  - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune containers with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**Vec<models::ContainersPruneReport>**](ContainersPruneReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_rename

> container_rename(name, name2)
Rename an existing container

Change the name of an existing container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Full or partial ID or full name of the container to rename | [required] |
**name2** | **String** | New name for the container | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_resize

> serde_json::Value container_resize(name, h, w, running)
Resize a container's TTY

Resize the terminal attached to a container (for use with Attach).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**h** | Option<**i32**> | Height to set for the terminal, in characters |  |
**w** | Option<**i32**> | Width to set for the terminal, in characters |  |
**running** | Option<**bool**> | Ignore containers not running errors |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_restart

> container_restart(name, t)
Restart container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**t** | Option<**i32**> | timeout before sending kill signal to container |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_start

> container_start(name, detach_keys)
Start a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**detach_keys** | Option<**String**> | Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _. |  |[default to ctrl-p,ctrl-q]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_stats

> serde_json::Value container_stats(name, stream, one_shot)
Get stats for a container

This returns a live stream of a container’s resource usage statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**stream** | Option<**bool**> | Stream the output |  |[default to true]
**one_shot** | Option<**bool**> | Provide a one-shot response in which preCPU stats are blank, resulting in a single cycle return. |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_stop

> container_stop(name, t)
Stop a container

Stop a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**t** | Option<**i32**> | number of seconds to wait before killing container |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_top

> models::ContainerTopOkBody container_top(name, ps_args)
List processes running inside a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**ps_args** | Option<**String**> | arguments to pass to ps such as aux. |  |[default to -ef]

### Return type

[**models::ContainerTopOkBody**](ContainerTopOKBody.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_unpause

> container_unpause(name)
Unpause container

Resume a paused container

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


## container_update

> container_update(name, resources)
Update configuration of an existing container

Change configuration settings for an existing container without requiring recreation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Full or partial ID or full name of the container to rename | [required] |
**resources** | Option<**models::UpdateConfig**> | attributes for updating the container |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_wait

> models::ContainerWait200Response container_wait(name, condition, interval)
Wait on a container

Block until a container stops or given condition is met.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the container | [required] |
**condition** | Option<**String**> | wait until container is to a given condition. default is stopped. valid conditions are:   - configured   - created   - exited   - paused   - running   - stopped  |  |
**interval** | Option<**String**> | Time Interval to wait before polling for completion. |  |[default to 250ms]

### Return type

[**models::ContainerWait200Response**](ContainerWait_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_commit

> image_commit(container, repo, tag, comment, author, pause, changes, squash)
New Image

Create a new image from a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container** | Option<**String**> | the name or ID of a container |  |
**repo** | Option<**String**> | the repository name for the created image |  |
**tag** | Option<**String**> | tag name for the created image |  |
**comment** | Option<**String**> | commit message |  |
**author** | Option<**String**> | author of the image |  |
**pause** | Option<**bool**> | pause the container before committing it |  |
**changes** | Option<**String**> | instructions to apply while committing in Dockerfile format |  |
**squash** | Option<**bool**> | squash newly built layers into a single new layer |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_container_archive

> put_container_archive(name, path, no_overwrite_dir_non_dir, copy_uidgid, request)
Put files into a container

Put a tar archive of files into a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | container name or id | [required] |
**path** | **String** | Path to a directory in the container to extract | [required] |
**no_overwrite_dir_non_dir** | Option<**String**> | if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa (1 or true) |  |
**copy_uidgid** | Option<**String**> | copy UID/GID maps to the dest file or di (1 or true) |  |
**request** | Option<**String**> | tarfile of files to copy into the container |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

