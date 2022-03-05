# \SearchApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_get**](SearchApi.md#search_get) | **GET** /search | GET /search



## search_get

> Vec<crate::models::InlineResponse20021> search_get(q)
GET /search

Search players by personaname.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Search string | [required] |

### Return type

[**Vec<crate::models::InlineResponse20021>**](inline_response_200_21.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

