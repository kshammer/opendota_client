# \ConstantsApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**constants_get**](ConstantsApi.md#constants_get) | **GET** /constants | GET /constants
[**constants_resource_get**](ConstantsApi.md#constants_resource_get) | **GET** /constants/{resource} | GET /constants



## constants_get

> Vec<String> constants_get()
GET /constants

Gets an array of available resources.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## constants_resource_get

> Vec<serde_json::Value> constants_resource_get(resource)
GET /constants

Get static game data mirrored from the dotaconstants repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** | Resource name e.g. `heroes`. [List of resources](https://github.com/odota/dotaconstants/tree/master/build) | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

