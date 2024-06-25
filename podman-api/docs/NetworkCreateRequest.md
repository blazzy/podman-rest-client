# NetworkCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachable** | Option<**bool**> |  | [optional]
**check_duplicate** | Option<**bool**> | Deprecated: CheckDuplicate is deprecated since API v1.44, but it defaults to true when sent by the client package to older daemons. | [optional]
**config_from** | Option<[**models::ConfigReference**](ConfigReference.md)> |  | [optional]
**config_only** | Option<**bool**> |  | [optional]
**driver** | Option<**String**> |  | [optional]
**enable_ipv6** | Option<**bool**> |  | [optional]
**ipam** | Option<[**models::Ipam**](IPAM.md)> |  | [optional]
**ingress** | Option<**bool**> |  | [optional]
**internal** | Option<**bool**> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**scope** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


