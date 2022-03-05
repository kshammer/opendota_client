# \LeaguesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**leagues_get**](LeaguesApi.md#leagues_get) | **GET** /leagues | GET /leagues
[**leagues_league_id_get**](LeaguesApi.md#leagues_league_id_get) | **GET** /leagues/{league_id} | GET /leagues/{league_id}
[**leagues_league_id_matches_get**](LeaguesApi.md#leagues_league_id_matches_get) | **GET** /leagues/{league_id}/matches | GET /leagues/{league_id}/matches
[**leagues_league_id_teams_get**](LeaguesApi.md#leagues_league_id_teams_get) | **GET** /leagues/{league_id}/teams | GET /leagues/{league_id}/teams



## leagues_get

> Vec<crate::models::InlineResponse20029> leagues_get()
GET /leagues

Get league data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20029>**](inline_response_200_29.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leagues_league_id_get

> Vec<crate::models::InlineResponse20029> leagues_league_id_get(league_id)
GET /leagues/{league_id}

Get data for a league

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**league_id** | **i32** | League ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse20029>**](inline_response_200_29.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leagues_league_id_matches_get

> crate::models::InlineResponse20016 leagues_league_id_matches_get(league_id)
GET /leagues/{league_id}/matches

Get matches for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**league_id** | **i32** | League ID | [required] |

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leagues_league_id_teams_get

> crate::models::InlineResponse20030 leagues_league_id_teams_get(league_id)
GET /leagues/{league_id}/teams

Get teams for a league

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**league_id** | **i32** | League ID | [required] |

### Return type

[**crate::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

