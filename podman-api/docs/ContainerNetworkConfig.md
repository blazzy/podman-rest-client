# ContainerNetworkConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**networks** | Option<[**std::collections::HashMap<String, models::PerNetworkOptions>**](PerNetworkOptions.md)> | Map of networks names or ids that the container should join. You can request additional settings for each network, you can set network aliases, static ips, static mac address  and the network interface name for this container on the specific network. If the map is empty and the bridge network mode is set the container will be joined to the default network. Optional. | [optional]
**base_hosts_file** | Option<**String**> | BaseHostsFile is the path to a hosts file, the entries from this file are added to the containers hosts file. As special value \"image\" is allowed which uses the /etc/hosts file from within the image and \"none\" which uses no base file at all. If it is empty we should default to the base_hosts_file configuration in containers.conf. Optional. | [optional]
**cni_networks** | Option<**Vec<String>**> | CNINetworks is a list of CNI networks to join the container to. If this list is empty, the default CNI network will be joined instead. If at least one entry is present, we will not join the default network (unless it is part of this list). Only available if NetNS is set to bridge. Optional. Deprecated: as of podman 4.0 use \"Networks\" instead. | [optional]
**dns_option** | Option<**Vec<String>**> | DNSOptions is a set of DNS options that will be used in the container's resolv.conf, replacing the host's DNS options which are used by default. Conflicts with UseImageResolvConf. Optional. | [optional]
**dns_search** | Option<**Vec<String>**> | DNSSearch is a set of DNS search domains that will be used in the container's resolv.conf, replacing the host's DNS search domains which are used by default. Conflicts with UseImageResolvConf. Optional. | [optional]
**dns_server** | Option<**Vec<String>**> | DNSServers is a set of DNS servers that will be used in the container's resolv.conf, replacing the host's DNS Servers which are used by default. Conflicts with UseImageResolvConf. Optional. | [optional]
**expose** | Option<[**serde_json::Value**](.md)> | Expose is a number of ports that will be forwarded to the container if PublishExposedPorts is set. Expose is a map of uint16 (port number) to a string representing protocol i.e map[uint16]string. Allowed protocols are \"tcp\", \"udp\", and \"sctp\", or some combination of the three separated by commas. If protocol is set to \"\" we will assume TCP. Only available if NetNS is set to Bridge or Slirp, and PublishExposedPorts is set. Optional. | [optional]
**hostadd** | Option<**Vec<String>**> | HostAdd is a set of hosts which will be added to the container's etc/hosts file. Conflicts with UseImageHosts. Optional. | [optional]
**netns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**network_options** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | NetworkOptions are additional options for each network Optional. | [optional]
**portmappings** | Option<[**Vec<models::PortMapping>**](PortMapping.md)> | PortBindings is a set of ports to map into the container. Only available if NetNS is set to bridge, slirp, or pasta. Optional. | [optional]
**publish_image_ports** | Option<**bool**> | PublishExposedPorts will publish ports specified in the image to random unused ports (guaranteed to be above 1024) on the host. This is based on ports set in Expose below, and any ports specified by the Image (if one is given). Only available if NetNS is set to Bridge or Slirp. Optional. | [optional]
**use_image_hosts** | Option<**bool**> | UseImageHosts indicates that /etc/hosts should not be managed by Podman, and instead sourced from the image. Conflicts with HostAdd. Optional. | [optional]
**use_image_resolve_conf** | Option<**bool**> | UseImageResolvConf indicates that resolv.conf should not be managed by Podman, but instead sourced from the image. Conflicts with DNSServer, DNSSearch, DNSOption. Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


