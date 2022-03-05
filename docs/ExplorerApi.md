# \ExplorerApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**explorer_get**](ExplorerApi.md#explorer_get) | **GET** /explorer | GET /explorer



## explorer_get

> serde_json::Value explorer_get(sql)
GET /explorer

Submit arbitrary SQL queries to the database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sql** | Option<**String**> | The PostgreSQL query as percent-encoded string. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

