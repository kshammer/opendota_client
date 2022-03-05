# \RequestApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**request_job_id_get**](RequestApi.md#request_job_id_get) | **GET** /request/{jobId} | GET /request/{jobId}
[**request_match_id_post**](RequestApi.md#request_match_id_post) | **POST** /request/{match_id} | POST /request/{match_id}



## request_job_id_get

> serde_json::Value request_job_id_get(job_id)
GET /request/{jobId}

Get parse request state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The job ID to query. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_match_id_post

> serde_json::Value request_match_id_post(match_id)
POST /request/{match_id}

Submit a new parse request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

