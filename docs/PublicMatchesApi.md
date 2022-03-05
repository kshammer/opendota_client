# \PublicMatchesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**public_matches_get**](PublicMatchesApi.md#public_matches_get) | **GET** /publicMatches | GET /publicMatches



## public_matches_get

> Vec<crate::models::InlineResponse20017> public_matches_get(mmr_ascending, mmr_descending, less_than_match_id)
GET /publicMatches

Get list of randomly sampled public matches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mmr_ascending** | Option<**i32**> | Order by MMR ascending |  |
**mmr_descending** | Option<**i32**> | Order by MMR descending |  |
**less_than_match_id** | Option<**i32**> | Get matches with a match ID lower than this value |  |

### Return type

[**Vec<crate::models::InlineResponse20017>**](inline_response_200_17.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

