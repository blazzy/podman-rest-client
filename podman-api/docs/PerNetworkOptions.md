# PerNetworkOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aliases** | Option<**Vec<String>**> | Aliases contains a list of names which the dns server should resolve to this container. Should only be set when DNSEnabled is true on the Network. If aliases are set but there is no dns support for this network the network interface implementation should ignore this and NOT error. Optional. | [optional]
**interface_name** | Option<**String**> | InterfaceName for this container. Required in the backend. Optional in the frontend. Will be filled with ethX (where X is a integer) when empty. | [optional]
**static_ips** | Option<**Vec<String>**> | StaticIPs for this container. Optional. | [optional]
**static_mac** | Option<**String**> | StaticMac for this container. Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


