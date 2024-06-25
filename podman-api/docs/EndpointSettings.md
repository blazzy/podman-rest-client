# EndpointSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aliases** | Option<**Vec<String>**> |  | [optional]
**dns_names** | Option<**Vec<String>**> | DNSNames holds all the (non fully qualified) DNS names associated to this endpoint. First entry is used to generate PTR records. | [optional]
**driver_opts** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**endpoint_id** | Option<**String**> |  | [optional]
**gateway** | Option<**String**> |  | [optional]
**global_ipv6_address** | Option<**String**> |  | [optional]
**global_ipv6_prefix_len** | Option<**i64**> |  | [optional]
**ipam_config** | Option<[**models::EndpointIpamConfig**](EndpointIPAMConfig.md)> |  | [optional]
**ip_address** | Option<**String**> |  | [optional]
**ip_prefix_len** | Option<**i64**> |  | [optional]
**ipv6_gateway** | Option<**String**> |  | [optional]
**links** | Option<**Vec<String>**> |  | [optional]
**mac_address** | Option<**String**> | MacAddress may be used to specify a MAC address when the container is created. Once the container is running, it becomes operational data (it may contain a generated address). | [optional]
**network_id** | Option<**String**> | Operational data | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


