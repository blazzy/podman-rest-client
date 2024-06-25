# HealthCheckResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failing_streak** | Option<**i64**> | FailingStreak is the number of consecutive failed healthchecks | [optional]
**log** | Option<[**Vec<models::HealthCheckLog>**](HealthCheckLog.md)> | Log describes healthcheck attempts and results | [optional]
**status** | Option<**String**> | Status starting, healthy or unhealthy | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


