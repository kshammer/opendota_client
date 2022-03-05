# \ProMatchesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pro_matches_get**](ProMatchesApi.md#pro_matches_get) | **GET** /proMatches | GET /proMatches



## pro_matches_get

> Vec<crate::models::InlineResponse20016> pro_matches_get(less_than_match_id)
GET /proMatches

Get list of pro matches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**less_than_match_id** | Option<**i32**> | Get matches with a match ID lower than this value |  |

### Return type

[**Vec<crate::models::InlineResponse20016>**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

