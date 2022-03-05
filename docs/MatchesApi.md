# \MatchesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**matches_match_id_get**](MatchesApi.md#matches_match_id_get) | **GET** /matches/{match_id} | GET /matches/{match_id}



## matches_match_id_get

> crate::models::InlineResponse200 matches_match_id_get(match_id)
GET /matches/{match_id}

Match data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_id** | **i32** |  | [required] |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

