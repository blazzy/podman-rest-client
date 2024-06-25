# PortMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_port** | Option<**i32**> | ContainerPort is the port number that will be exposed from the container. Mandatory. | [optional]
**host_ip** | Option<**String**> | HostIP is the IP that we will bind to on the host. If unset, assumed to be 0.0.0.0 (all interfaces). | [optional]
**host_port** | Option<**i32**> | HostPort is the port number that will be forwarded from the host into the container. If omitted, a random port on the host (guaranteed to be over 1024) will be assigned. | [optional]
**protocol** | Option<**String**> | Protocol is the protocol forward. Must be either \"tcp\", \"udp\", and \"sctp\", or some combination of these separated by commas. If unset, assumed to be TCP. | [optional]
**range** | Option<**i32**> | Range is the number of ports that will be forwarded, starting at HostPort and ContainerPort and counting up. This is 1-indexed, so 1 is assumed to be a single port (only the Hostport:Containerport mapping will be added), 2 is two ports (both Hostport:Containerport and Hostport+1:Containerport+1), etc. If unset, assumed to be 1 (a single port). Both hostport + range and containerport + range must be less than 65536. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


