# \RankingsApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rankings_get**](RankingsApi.md#rankings_get) | **GET** /rankings | GET /rankings



## rankings_get

> crate::models::InlineResponse20022 rankings_get(hero_id)
GET /rankings

Top players by hero

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hero_id** | **String** | Hero ID | [required] |

### Return type

[**crate::models::InlineResponse20022**](inline_response_200_22.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

