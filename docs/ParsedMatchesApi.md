# \ParsedMatchesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**parsed_matches_get**](ParsedMatchesApi.md#parsed_matches_get) | **GET** /parsedMatches | GET /parsedMatches



## parsed_matches_get

> Vec<crate::models::InlineResponse20018> parsed_matches_get(less_than_match_id)
GET /parsedMatches

Get list of parsed match IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**less_than_match_id** | Option<**i32**> | Get matches with a match ID lower than this value |  |

### Return type

[**Vec<crate::models::InlineResponse20018>**](inline_response_200_18.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

