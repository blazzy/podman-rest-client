# ContainerHealthCheckConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**health_check_on_failure_action** | Option<**i64**> | HealthCheckOnFailureAction defines how Podman reacts when a container's health status turns unhealthy. | [optional]
**healthconfig** | Option<[**models::Schema2HealthConfig**](Schema2HealthConfig.md)> |  | [optional]
**startup_health_config** | Option<[**models::StartupHealthCheck**](StartupHealthCheck.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


