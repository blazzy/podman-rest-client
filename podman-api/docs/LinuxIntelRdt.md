# LinuxIntelRdt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clos_id** | Option<**String**> | The identity for RDT Class of Service | [optional]
**enable_cmt** | Option<**bool**> | EnableCMT is the flag to indicate if the Intel RDT CMT is enabled. CMT (Cache Monitoring Technology) supports monitoring of the last-level cache (LLC) occupancy for the container. | [optional]
**enable_mbm** | Option<**bool**> | EnableMBM is the flag to indicate if the Intel RDT MBM is enabled. MBM (Memory Bandwidth Monitoring) supports monitoring of total and local memory bandwidth for the container. | [optional]
**l3_cache_schema** | Option<**String**> | The schema for L3 cache id and capacity bitmask (CBM) Format: \"L3:<cache_id0>=<cbm0>;<cache_id1>=<cbm1>;...\" | [optional]
**mem_bw_schema** | Option<**String**> | The schema of memory bandwidth per L3 cache id Format: \"MB:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;...\" The unit of memory bandwidth is specified in \"percentages\" by default, and in \"MBps\" if MBA Software Controller is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


