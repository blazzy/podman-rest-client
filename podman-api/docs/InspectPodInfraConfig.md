# InspectPodInfraConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dns_option** | Option<**Vec<String>**> | DNSOption is a set of DNS options that will be used by the infra container's resolv.conf and shared with the remainder of the pod. | [optional]
**dns_search** | Option<**Vec<String>**> | DNSSearch is a set of DNS search domains that will be used by the infra container's resolv.conf and shared with the remainder of the pod. | [optional]
**dns_server** | Option<**Vec<String>**> | DNSServer is a set of DNS Servers that will be used by the infra container's resolv.conf and shared with the remainder of the pod. | [optional]
**host_add** | Option<**Vec<String>**> | HostAdd adds a number of hosts to the infra container's resolv.conf which will be shared with the rest of the pod. | [optional]
**host_network** | Option<**bool**> | HostNetwork is whether the infra container (and thus the whole pod) will use the host's network and not create a network namespace. | [optional]
**network_options** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | NetworkOptions are additional options for each network | [optional]
**networks** | Option<**Vec<String>**> | Networks is a list of networks the pod will join. | [optional]
**no_manage_hosts** | Option<**bool**> | NoManageHosts indicates that the pod will not manage /etc/hosts and instead each container will handle their own. | [optional]
**no_manage_resolv_conf** | Option<**bool**> | NoManageResolvConf indicates that the pod will not manage resolv.conf and instead each container will handle their own. | [optional]
**port_bindings** | Option<[**std::collections::HashMap<String, Vec<models::InspectHostPort>>**](Vec.md)> | PortBindings are ports that will be forwarded to the infra container and then shared with the pod. | [optional]
**static_ip** | Option<**String**> | StaticIP is a static IPv4 that will be assigned to the infra container and then used by the pod. | [optional]
**static_mac** | Option<**String**> | StaticMAC is a static MAC address that will be assigned to the infra container and then used by the pod. | [optional]
**cpu_period** | Option<**i32**> | CPUPeriod contains the CPU period of the pod | [optional]
**cpu_quota** | Option<**i64**> | CPUQuota contains the CPU quota of the pod | [optional]
**cpuset_cpus** | Option<**String**> | CPUSetCPUs contains linux specific CPU data for the container | [optional]
**pid_ns** | Option<**String**> | Pid is the PID namespace mode of the pod's infra container | [optional]
**userns** | Option<**String**> | UserNS is the usernamespace that all the containers in the pod will join. | [optional]
**uts_ns** | Option<**String**> | UtsNS is the uts namespace that all containers in the pod will join | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


