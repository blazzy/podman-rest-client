# Network

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | Created contains the timestamp when this network was created. | [optional]
**dns_enabled** | Option<**bool**> | DNSEnabled is whether name resolution is active for container on this Network. Only supported with the bridge driver. | [optional]
**driver** | Option<**String**> | Driver for this Network, e.g. bridge, macvlan... | [optional]
**id** | Option<**String**> | ID of the Network. | [optional]
**internal** | Option<**bool**> | Internal is whether the Network should not have external routes to public or other Networks. | [optional]
**ipam_options** | Option<**std::collections::HashMap<String, String>**> | IPAMOptions contains options used for the ip assignment. | [optional]
**ipv6_enabled** | Option<**bool**> | IPv6Enabled if set to true an ipv6 subnet should be created for this net. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels is a set of key-value labels that have been applied to the Network. | [optional]
**name** | Option<**String**> | Name of the Network. | [optional]
**network_dns_servers** | Option<**Vec<String>**> | List of custom DNS server for podman's DNS resolver at network level, all the containers attached to this network will consider resolvers configured at network level. | [optional]
**network_interface** | Option<**String**> | NetworkInterface is the network interface name on the host. | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | Options is a set of key-value options that have been applied to the Network. | [optional]
**routes** | Option<[**Vec<models::Route>**](Route.md)> | Routes to use for this network. | [optional]
**subnets** | Option<[**Vec<models::Subnet>**](Subnet.md)> | Subnets to use for this network. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


