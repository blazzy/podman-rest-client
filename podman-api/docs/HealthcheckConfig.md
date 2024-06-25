# HealthcheckConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]
**retries** | Option<**i64**> | Retries is the number of consecutive failures needed to consider a container as unhealthy. Zero means inherit. | [optional]
**start_interval** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]
**start_period** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]
**test** | Option<**Vec<String>**> | Test is the test to perform to check that the container is healthy. An empty slice means to inherit the default. The options are: {} : inherit healthcheck {\"NONE\"} : disable healthcheck {\"CMD\", args...} : exec arguments directly {\"CMD-SHELL\", command} : run command with system's default shell | [optional]
**timeout** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


