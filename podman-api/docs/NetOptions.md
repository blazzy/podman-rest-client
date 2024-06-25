# NetOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dns_option** | Option<**Vec<String>**> |  | [optional]
**dns_search** | Option<**Vec<String>**> |  | [optional]
**dns_server** | Option<**Vec<String>**> |  | [optional]
**hostadd** | Option<**Vec<String>**> |  | [optional]
**netns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**network_alias** | Option<**Vec<String>**> |  | [optional]
**network_options** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | NetworkOptions are additional options for each network | [optional]
**networks** | Option<[**std::collections::HashMap<String, models::PerNetworkOptions>**](PerNetworkOptions.md)> |  | [optional]
**no_manage_hosts** | Option<**bool**> |  | [optional]
**no_manage_resolv_conf** | Option<**bool**> |  | [optional]
**portmappings** | Option<[**Vec<models::PortMapping>**](PortMapping.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


