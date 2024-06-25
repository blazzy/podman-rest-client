# InspectPodData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cgroup_parent** | Option<**String**> | CgroupParent is the parent of the pod's Cgroup. | [optional]
**cgroup_path** | Option<**String**> | CgroupPath is the path to the pod's Cgroup. | [optional]
**containers** | Option<[**Vec<models::InspectPodContainerInfo>**](InspectPodContainerInfo.md)> | Containers gives a brief summary of all containers in the pod and their current status. | [optional]
**create_cgroup** | Option<**bool**> | CreateCgroup is whether this pod will create its own Cgroup to group containers under. | [optional]
**create_command** | Option<**Vec<String>**> | CreateCommand is the full command plus arguments of the process the container has been created with. | [optional]
**create_infra** | Option<**bool**> | CreateInfra is whether this pod will create an infra container to share namespaces. | [optional]
**created** | Option<**String**> | Created is the time when the pod was created. | [optional]
**exit_policy** | Option<**String**> | ExitPolicy of the pod. | [optional]
**hostname** | Option<**String**> | Hostname is the hostname that the pod will set. | [optional]
**id** | Option<**String**> | ID is the ID of the pod. | [optional]
**infra_config** | Option<[**models::InspectPodInfraConfig**](InspectPodInfraConfig.md)> |  | [optional]
**infra_container_id** | Option<**String**> | InfraContainerID is the ID of the pod's infra container, if one is present. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels is a set of key-value labels that have been applied to the pod. | [optional]
**lock_number** | Option<**i32**> | Number of the pod's Libpod lock. | [optional]
**name** | Option<**String**> | Name is the name of the pod. | [optional]
**namespace** | Option<**String**> | Namespace is the Libpod namespace the pod is placed in. | [optional]
**num_containers** | Option<**i32**> | NumContainers is the number of containers in the pod, including the infra container. | [optional]
**restart_policy** | Option<**String**> | RestartPolicy of the pod. | [optional]
**shared_namespaces** | Option<**Vec<String>**> | SharedNamespaces contains a list of namespaces that will be shared by containers within the pod. Can only be set if CreateInfra is true. | [optional]
**state** | Option<**String**> | State represents the current state of the pod. | [optional]
**blkio_weight** | Option<**i32**> | BlkioWeight contains the blkio weight limit for the pod | [optional]
**blkio_weight_device** | Option<[**Vec<models::InspectBlkioWeightDevice>**](InspectBlkioWeightDevice.md)> | BlkioWeightDevice contains the blkio weight device limits for the pod | [optional]
**cpu_period** | Option<**i32**> | CPUPeriod contains the CPU period of the pod | [optional]
**cpu_quota** | Option<**i64**> | CPUQuota contains the CPU quota of the pod | [optional]
**cpu_shares** | Option<**i32**> | CPUShares contains the cpu shares for the pod | [optional]
**cpuset_cpus** | Option<**String**> | CPUSetCPUs contains linux specific CPU data for the pod | [optional]
**cpuset_mems** | Option<**String**> | CPUSetMems contains linux specific CPU data for the pod | [optional]
**device_read_bps** | Option<[**Vec<models::InspectBlkioThrottleDevice>**](InspectBlkioThrottleDevice.md)> | BlkioDeviceReadBps contains the Read/Access limit for the pod's devices | [optional]
**device_write_bps** | Option<[**Vec<models::InspectBlkioThrottleDevice>**](InspectBlkioThrottleDevice.md)> | BlkioDeviceReadBps contains the Read/Access limit for the pod's devices | [optional]
**devices** | Option<[**Vec<models::InspectDevice>**](InspectDevice.md)> | Devices contains the specified host devices | [optional]
**memory_limit** | Option<**i32**> | MemoryLimit contains the specified cgroup memory limit for the pod | [optional]
**memory_swap** | Option<**i32**> | MemorySwap contains the specified memory swap limit for the pod | [optional]
**mounts** | Option<[**Vec<models::InspectMount>**](InspectMount.md)> | Mounts contains volume related information for the pod | [optional]
**security_opt** | Option<**Vec<String>**> | SecurityOpt contains the specified security labels and related SELinux information | [optional]
**volumes_from** | Option<**Vec<String>**> | VolumesFrom contains the containers that the pod inherits mounts from | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


