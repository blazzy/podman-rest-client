# InspectAdditionalNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_mac_addresses** | Option<**Vec<String>**> | AdditionalMacAddresses is a set of additional MAC Addresses beyond the first. CNI may configure more than one interface for a single network, which can cause this. | [optional]
**aliases** | Option<**Vec<String>**> | Aliases are any network aliases the container has in this network. | [optional]
**driver_opts** | Option<**std::collections::HashMap<String, String>**> | DriverOpts is presently unused and maintained exclusively for compatibility. | [optional]
**endpoint_id** | Option<**String**> | EndpointID is unused, maintained exclusively for compatibility. | [optional]
**gateway** | Option<**String**> | Gateway is the IP address of the gateway this network will use. | [optional]
**global_ipv6_address** | Option<**String**> | GlobalIPv6Address is the global-scope IPv6 Address for this network. | [optional]
**global_ipv6_prefix_len** | Option<**i64**> | GlobalIPv6PrefixLen is the length of the subnet mask of this network. | [optional]
**ipam_config** | Option<**std::collections::HashMap<String, String>**> | IPAMConfig is presently unused and maintained exclusively for compatibility. | [optional]
**ip_address** | Option<**String**> | IPAddress is the IP address for this network. | [optional]
**ip_prefix_len** | Option<**i64**> | IPPrefixLen is the length of the subnet mask of this network. | [optional]
**ipv6_gateway** | Option<**String**> | IPv6Gateway is the IPv6 gateway this network will use. | [optional]
**links** | Option<**Vec<String>**> | Links is presently unused and maintained exclusively for compatibility. | [optional]
**mac_address** | Option<**String**> | MacAddress is the MAC address for the interface in this network. | [optional]
**network_id** | Option<**String**> | Name of the network we're connecting to. | [optional]
**secondary_ip_addresses** | Option<[**Vec<models::Address>**](Address.md)> | SecondaryIPAddresses is a list of extra IP Addresses that the container has been assigned in this network. | [optional]
**secondary_ipv6_addresses** | Option<[**Vec<models::Address>**](Address.md)> | SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the container has been assigned in this network. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


