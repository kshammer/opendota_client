# \RecordsApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**records_field_get**](RecordsApi.md#records_field_get) | **GET** /records/{field} | GET /records/{field}



## records_field_get

> Vec<crate::models::InlineResponse20034> records_field_get(field)
GET /records/{field}

Get top performances in a stat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field name to query | [required] |

### Return type

[**Vec<crate::models::InlineResponse20034>**](inline_response_200_34.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

