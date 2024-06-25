# \PodsApi

All URIs are relative to *http://podman.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_kube_libpod**](PodsApi.md#generate_kube_libpod) | **Get** /libpod/generate/kube | Generate a Kubernetes YAML file.
[**generate_systemd_libpod**](PodsApi.md#generate_systemd_libpod) | **Get** /libpod/generate/{name}/systemd | Generate Systemd Units
[**kube_apply_libpod**](PodsApi.md#kube_apply_libpod) | **Post** /libpod/kube/apply | Apply a podman workload or Kubernetes YAML file.
[**play_kube_down_libpod**](PodsApi.md#play_kube_down_libpod) | **Delete** /libpod/play/kube | Remove resources created from kube play
[**play_kube_libpod**](PodsApi.md#play_kube_libpod) | **Post** /libpod/play/kube | Play a Kubernetes YAML file.
[**pod_create_libpod**](PodsApi.md#pod_create_libpod) | **Post** /libpod/pods/create | Create a pod
[**pod_delete_libpod**](PodsApi.md#pod_delete_libpod) | **Delete** /libpod/pods/{name} | Remove pod
[**pod_exists_libpod**](PodsApi.md#pod_exists_libpod) | **Get** /libpod/pods/{name}/exists | Pod exists
[**pod_inspect_libpod**](PodsApi.md#pod_inspect_libpod) | **Get** /libpod/pods/{name}/json | Inspect pod
[**pod_kill_libpod**](PodsApi.md#pod_kill_libpod) | **Post** /libpod/pods/{name}/kill | Kill a pod
[**pod_list_libpod**](PodsApi.md#pod_list_libpod) | **Get** /libpod/pods/json | List pods
[**pod_pause_libpod**](PodsApi.md#pod_pause_libpod) | **Post** /libpod/pods/{name}/pause | Pause a pod
[**pod_prune_libpod**](PodsApi.md#pod_prune_libpod) | **Post** /libpod/pods/prune | Prune unused pods
[**pod_restart_libpod**](PodsApi.md#pod_restart_libpod) | **Post** /libpod/pods/{name}/restart | Restart a pod
[**pod_start_libpod**](PodsApi.md#pod_start_libpod) | **Post** /libpod/pods/{name}/start | Start a pod
[**pod_stats_all_libpod**](PodsApi.md#pod_stats_all_libpod) | **Get** /libpod/pods/stats | Statistics for one or more pods
[**pod_stop_libpod**](PodsApi.md#pod_stop_libpod) | **Post** /libpod/pods/{name}/stop | Stop a pod
[**pod_top_libpod**](PodsApi.md#pod_top_libpod) | **Get** /libpod/pods/{name}/top | List processes
[**pod_unpause_libpod**](PodsApi.md#pod_unpause_libpod) | **Post** /libpod/pods/{name}/unpause | Unpause a pod



## generate_kube_libpod

> std::path::PathBuf generate_kube_libpod(names, service, r#type, replicas, no_trunc, podman_only)
Generate a Kubernetes YAML file.

Generate Kubernetes YAML based on a pod or container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**names** | [**Vec<String>**](String.md) | Name or ID of the container or pod. | [required] |
**service** | Option<**bool**> | Generate YAML for a Kubernetes service object. |  |[default to false]
**r#type** | Option<**String**> | Generate YAML for the given Kubernetes kind. |  |[default to pod]
**replicas** | Option<**i32**> | Set the replica number for Deployment kind. |  |[default to 0]
**no_trunc** | Option<**bool**> | don't truncate annotations to the Kubernetes maximum length of 63 characters |  |[default to false]
**podman_only** | Option<**bool**> | add podman-only reserved annotations in generated YAML file (cannot be used by Kubernetes) |  |[default to false]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/vnd.yaml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_systemd_libpod

> std::collections::HashMap<String, String> generate_systemd_libpod(name, use_name, new, no_header, start_timeout, stop_timeout, restart_policy, container_prefix, pod_prefix, separator, restart_sec, wants, after, requires, additional_env_variables)
Generate Systemd Units

Generate Systemd Units based on a pod or container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name or ID of the container or pod. | [required] |
**use_name** | Option<**bool**> | Use container/pod names instead of IDs. |  |[default to false]
**new** | Option<**bool**> | Create a new container instead of starting an existing one. |  |[default to false]
**no_header** | Option<**bool**> | Do not generate the header including the Podman version and the timestamp. |  |[default to false]
**start_timeout** | Option<**i32**> | Start timeout in seconds. |  |[default to 0]
**stop_timeout** | Option<**i32**> | Stop timeout in seconds. |  |[default to 10]
**restart_policy** | Option<**String**> | Systemd restart-policy. |  |[default to on-failure]
**container_prefix** | Option<**String**> | Systemd unit name prefix for containers. |  |[default to container]
**pod_prefix** | Option<**String**> | Systemd unit name prefix for pods. |  |[default to pod]
**separator** | Option<**String**> | Systemd unit name separator between name/id and prefix. |  |[default to -]
**restart_sec** | Option<**i32**> | Configures the time to sleep before restarting a service. |  |[default to 0]
**wants** | Option<[**Vec<String>**](String.md)> | Systemd Wants list for the container or pods. |  |[default to []]
**after** | Option<[**Vec<String>**](String.md)> | Systemd After list for the container or pods. |  |[default to []]
**requires** | Option<[**Vec<String>**](String.md)> | Systemd Requires list for the container or pods. |  |[default to []]
**additional_env_variables** | Option<[**Vec<String>**](String.md)> | Set environment variables to the systemd unit files. |  |[default to []]

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kube_apply_libpod

> std::path::PathBuf kube_apply_libpod(ca_cert_file, kube_config, namespace, service, file, request)
Apply a podman workload or Kubernetes YAML file.

Deploy a podman container, pod, volume, or Kubernetes yaml to a Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ca_cert_file** | Option<**String**> | Path to the CA cert file for the Kubernetes cluster. |  |
**kube_config** | Option<**String**> | Path to the kubeconfig file for the Kubernetes cluster. |  |
**namespace** | Option<**String**> | The namespace to deploy the workload to on the Kubernetes cluster. |  |
**service** | Option<**bool**> | Create a service object for the container being deployed. |  |
**file** | Option<**String**> | Path to the Kubernetes yaml file to deploy. |  |
**request** | Option<**String**> | Kubernetes YAML file. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## play_kube_down_libpod

> models::PlayKubeReport play_kube_down_libpod(force)
Remove resources created from kube play

Tears down pods, secrets, and volumes defined in a YAML file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option<**bool**> | Remove volumes. |  |[default to false]

### Return type

[**models::PlayKubeReport**](PlayKubeReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## play_kube_libpod

> models::PlayKubeReport play_kube_libpod(annotations, log_driver, log_options, network, no_hosts, no_trunc, publish_ports, publish_all_ports, replace, service_container, start, static_ips, static_macs, tls_verify, userns, wait, request)
Play a Kubernetes YAML file.

Create and run pods based on a Kubernetes YAML file (pod or service kind).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotations** | Option<**String**> | JSON encoded value of annotations (a map[string]string). |  |
**log_driver** | Option<**String**> | Logging driver for the containers in the pod. |  |
**log_options** | Option<[**Vec<String>**](String.md)> | logging driver options |  |
**network** | Option<[**Vec<String>**](String.md)> | USe the network mode or specify an array of networks. |  |
**no_hosts** | Option<**bool**> | do not setup /etc/hosts file in container |  |[default to false]
**no_trunc** | Option<**bool**> | use annotations that are not truncated to the Kubernetes maximum length of 63 characters |  |[default to false]
**publish_ports** | Option<[**Vec<String>**](String.md)> | publish a container's port, or a range of ports, to the host |  |
**publish_all_ports** | Option<**bool**> | Whether to publish all ports defined in the K8S YAML file (containerPort, hostPort), if false only hostPort will be published |  |
**replace** | Option<**bool**> | replace existing pods and containers |  |[default to false]
**service_container** | Option<**bool**> | Starts a service container before all pods. |  |[default to false]
**start** | Option<**bool**> | Start the pod after creating it. |  |[default to true]
**static_ips** | Option<[**Vec<String>**](String.md)> | Static IPs used for the pods. |  |
**static_macs** | Option<[**Vec<String>**](String.md)> | Static MACs used for the pods. |  |
**tls_verify** | Option<**bool**> | Require HTTPS and verify signatures when contacting registries. |  |[default to true]
**userns** | Option<**String**> | Set the user namespace mode for the pods. |  |
**wait** | Option<**bool**> | Clean up all objects created when a SIGTERM is received or pods exit. |  |[default to false]
**request** | Option<**String**> | Kubernetes YAML file. |  |

### Return type

[**models::PlayKubeReport**](PlayKubeReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_create_libpod

> models::IdResponse pod_create_libpod(create)
Create a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<[**PodSpecGenerator**](PodSpecGenerator.md)> | attributes for creating a pod |  |

### Return type

[**models::IdResponse**](IdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_delete_libpod

> models::PodRmReport pod_delete_libpod(name, force)
Remove pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |
**force** | Option<**bool**> | force removal of a running pod by first stopping all containers, then removing all containers in the pod |  |

### Return type

[**models::PodRmReport**](PodRmReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_exists_libpod

> pod_exists_libpod(name)
Pod exists

Check if a pod exists by name or ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_inspect_libpod

> models::InspectPodData pod_inspect_libpod(name)
Inspect pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |

### Return type

[**models::InspectPodData**](InspectPodData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_kill_libpod

> models::PodKillReport pod_kill_libpod(name, signal)
Kill a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |
**signal** | Option<**String**> | signal to be sent to pod |  |[default to SIGKILL]

### Return type

[**models::PodKillReport**](PodKillReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_list_libpod

> Vec<models::ListPodsReport> pod_list_libpod(filters)
List pods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON encoded value of the filters (a map[string][]string) to process on the pods list. Available filters:   - `id=<pod-id>` Matches all of pod id.   - `label=<key>` or `label=<key>:<value>` Matches pods based on the presence of a label alone or a label and a value.   - `name=<pod-name>` Matches all of pod name.   - `until=<timestamp>` List pods created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.   - `status=<pod-status>` Pod's status: `stopped`, `running`, `paused`, `exited`, `dead`, `created`, `degraded`.   - `network=<pod-network>` Name or full ID of network.   - `ctr-names=<pod-ctr-names>` Container name within the pod.   - `ctr-ids=<pod-ctr-ids>` Container ID within the pod.   - `ctr-status=<pod-ctr-status>` Container status within the pod.   - `ctr-number=<pod-ctr-number>` Number of containers in the pod.  |  |

### Return type

[**Vec<models::ListPodsReport>**](ListPodsReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_pause_libpod

> models::PodPauseReport pod_pause_libpod(name)
Pause a pod

Pause a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |

### Return type

[**models::PodPauseReport**](PodPauseReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_prune_libpod

> models::PodPruneReport pod_prune_libpod()
Prune unused pods

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PodPruneReport**](PodPruneReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_restart_libpod

> models::PodRestartReport pod_restart_libpod(name)
Restart a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |

### Return type

[**models::PodRestartReport**](PodRestartReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_start_libpod

> models::PodStartReport pod_start_libpod(name)
Start a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |

### Return type

[**models::PodStartReport**](PodStartReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_stats_all_libpod

> Vec<models::PodStatsReport> pod_stats_all_libpod(all, names_or_ids)
Statistics for one or more pods

Display a live stream of resource usage statistics for the containers in one or more pods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Provide statistics for all running pods. |  |
**names_or_ids** | Option<[**Vec<String>**](String.md)> | Names or IDs of pods. |  |

### Return type

[**Vec<models::PodStatsReport>**](PodStatsReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_stop_libpod

> models::PodStopReport pod_stop_libpod(name, t)
Stop a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |
**t** | Option<**i32**> | timeout |  |

### Return type

[**models::PodStopReport**](PodStopReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_top_libpod

> models::PodTopOkBody pod_top_libpod(name, stream, delay, ps_args)
List processes

List processes running inside a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of pod to query for processes | [required] |
**stream** | Option<**bool**> | when true, repeatedly stream the latest output (As of version 4.0) |  |
**delay** | Option<**i32**> | if streaming, delay in seconds between updates. Must be >1. (As of version 4.0) |  |[default to 5]
**ps_args** | Option<**String**> | arguments to pass to ps such as aux.  |  |

### Return type

[**models::PodTopOkBody**](PodTopOKBody.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_unpause_libpod

> models::PodUnpauseReport pod_unpause_libpod(name)
Unpause a pod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the name or ID of the pod | [required] |

### Return type

[**models::PodUnpauseReport**](PodUnpauseReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

