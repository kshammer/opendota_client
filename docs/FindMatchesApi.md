# \FindMatchesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_matches_get**](FindMatchesApi.md#find_matches_get) | **GET** /findMatches | GET /



## find_matches_get

> serde_json::Value find_matches_get(team_a, team_b)
GET /

Finds recent matches by heroes played

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_a** | Option<**i32**> | Hero IDs on first team (array) |  |
**team_b** | Option<**i32**> | Hero IDs on second team (array) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

