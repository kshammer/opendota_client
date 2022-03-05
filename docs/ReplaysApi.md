# \ReplaysApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**replays_get**](ReplaysApi.md#replays_get) | **GET** /replays | GET /replays



## replays_get

> Vec<crate::models::InlineResponse20033> replays_get(match_id)
GET /replays

Get data to construct a replay URL with

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_id** | **i32** | Match IDs (array) | [required] |

### Return type

[**Vec<crate::models::InlineResponse20033>**](inline_response_200_33.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

