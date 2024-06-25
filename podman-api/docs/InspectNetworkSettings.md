# InspectNetworkSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_mac_addresses** | Option<**Vec<String>**> | AdditionalMacAddresses is a set of additional MAC Addresses beyond the first. CNI may configure more than one interface for a single network, which can cause this. | [optional]
**bridge** | Option<**String**> |  | [optional]
**endpoint_id** | Option<**String**> | EndpointID is unused, maintained exclusively for compatibility. | [optional]
**gateway** | Option<**String**> | Gateway is the IP address of the gateway this network will use. | [optional]
**global_ipv6_address** | Option<**String**> | GlobalIPv6Address is the global-scope IPv6 Address for this network. | [optional]
**global_ipv6_prefix_len** | Option<**i64**> | GlobalIPv6PrefixLen is the length of the subnet mask of this network. | [optional]
**hairpin_mode** | Option<**bool**> |  | [optional]
**ip_address** | Option<**String**> | IPAddress is the IP address for this network. | [optional]
**ip_prefix_len** | Option<**i64**> | IPPrefixLen is the length of the subnet mask of this network. | [optional]
**ipv6_gateway** | Option<**String**> | IPv6Gateway is the IPv6 gateway this network will use. | [optional]
**link_local_ipv6_address** | Option<**String**> |  | [optional]
**link_local_ipv6_prefix_len** | Option<**i64**> |  | [optional]
**mac_address** | Option<**String**> | MacAddress is the MAC address for the interface in this network. | [optional]
**networks** | Option<[**std::collections::HashMap<String, models::InspectAdditionalNetwork>**](InspectAdditionalNetwork.md)> | Networks contains information on non-default networks this container has joined. It is a map of network name to network information. | [optional]
**ports** | Option<[**std::collections::HashMap<String, Vec<models::InspectHostPort>>**](Vec.md)> |  | [optional]
**sandbox_id** | Option<**String**> |  | [optional]
**sandbox_key** | Option<**String**> |  | [optional]
**secondary_ip_addresses** | Option<[**Vec<models::Address>**](Address.md)> | SecondaryIPAddresses is a list of extra IP Addresses that the container has been assigned in this network. | [optional]
**secondary_ipv6_addresses** | Option<[**Vec<models::Address>**](Address.md)> | SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the container has been assigned in this network. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


