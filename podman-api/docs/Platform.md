# Platform

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | Architecture field specifies the CPU architecture, for example `amd64` or `ppc64le`. | [optional]
**os** | Option<**String**> | OS specifies the operating system, for example `linux` or `windows`. | [optional]
**os_period_features** | Option<**Vec<String>**> | OSFeatures is an optional field specifying an array of strings, each listing a required OS feature (for example on Windows `win32k`). | [optional]
**os_period_version** | Option<**String**> | OSVersion is an optional field specifying the operating system version, for example on Windows `10.0.14393.1066`. | [optional]
**variant** | Option<**String**> | Variant is an optional field specifying a variant of the CPU, for example `v7` to specify ARMv7 when architecture is `arm`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


