# PodBasicConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exit_policy** | Option<**String**> | ExitPolicy determines the pod's exit and stop behaviour. | [optional]
**hostname** | Option<**String**> | Hostname is the pod's hostname. If not set, the name of the pod will be used (if a name was not provided here, the name auto-generated for the pod will be used). This will be used by the infra container and all containers in the pod as long as the UTS namespace is shared. Optional. | [optional]
**infra_command** | Option<**Vec<String>**> | InfraCommand sets the command that will be used to start the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional. | [optional]
**infra_conmon_pid_file** | Option<**String**> | InfraConmonPidFile is a custom path to store the infra container's conmon PID. | [optional]
**infra_image** | Option<**String**> | InfraImage is the image that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional. | [optional]
**infra_name** | Option<**String**> | InfraName is the name that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional. | [optional]
**ipcns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels are key-value pairs that are used to add metadata to pods. Optional. | [optional]
**name** | Option<**String**> | Name is the name of the pod. If not provided, a name will be generated when the pod is created. Optional. | [optional]
**no_infra** | Option<**bool**> | NoInfra tells the pod not to create an infra container. If this is done, many networking-related options will become unavailable. Conflicts with setting any options in PodNetworkConfig, and the InfraCommand and InfraImages in this struct. Optional. | [optional]
**pidns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**pod_create_command** | Option<**Vec<String>**> |  | [optional]
**pod_devices** | Option<**Vec<String>**> | Devices contains user specified Devices to be added to the Pod | [optional]
**restart_policy** | Option<**String**> | RestartPolicy is the pod's restart policy - an action which will be taken when one or all the containers in the pod exits. If not given, the default policy will be set to Always, which restarts the containers in the pod when they exit indefinitely. Optional. | [optional]
**restart_tries** | Option<**i32**> | RestartRetries is the number of attempts that will be made to restart the container. Only available when RestartPolicy is set to \"on-failure\". Optional. | [optional]
**share_parent** | Option<**bool**> | PodCreateCommand is the command used to create this pod. This will be shown in the output of Inspect() on the pod, and may also be used by some tools that wish to recreate the pod (e.g. `podman generate systemd --new`). Optional. ShareParent determines if all containers in the pod will share the pod's cgroup as the cgroup parent | [optional]
**shared_namespaces** | Option<**Vec<String>**> | SharedNamespaces instructs the pod to share a set of namespaces. Shared namespaces will be joined (by default) by every container which joins the pod. If not set and NoInfra is false, the pod will set a default set of namespaces to share. Conflicts with NoInfra=true. Optional. | [optional]
**sysctl** | Option<**std::collections::HashMap<String, String>**> | Sysctl sets kernel parameters for the pod | [optional]
**userns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**utsns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


