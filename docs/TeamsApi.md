# \TeamsApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**teams_get**](TeamsApi.md#teams_get) | **GET** /teams | GET /teams
[**teams_team_id_get**](TeamsApi.md#teams_team_id_get) | **GET** /teams/{team_id} | GET /teams/{team_id}
[**teams_team_id_heroes_get**](TeamsApi.md#teams_team_id_heroes_get) | **GET** /teams/{team_id}/heroes | GET /teams/{team_id}/heroes
[**teams_team_id_matches_get**](TeamsApi.md#teams_team_id_matches_get) | **GET** /teams/{team_id}/matches | GET /teams/{team_id}/matches
[**teams_team_id_players_get**](TeamsApi.md#teams_team_id_players_get) | **GET** /teams/{team_id}/players | GET /teams/{team_id}/players



## teams_get

> Vec<crate::models::InlineResponse20030> teams_get()
GET /teams

Get team data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20030>**](inline_response_200_30.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_get

> crate::models::InlineResponse20030 teams_team_id_get(team_id)
GET /teams/{team_id}

Get data for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | Team ID | [required] |

### Return type

[**crate::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_heroes_get

> crate::models::InlineResponse20032 teams_team_id_heroes_get(team_id)
GET /teams/{team_id}/heroes

Get heroes for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | Team ID | [required] |

### Return type

[**crate::models::InlineResponse20032**](inline_response_200_32.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_matches_get

> crate::models::InlineResponse20016 teams_team_id_matches_get(team_id)
GET /teams/{team_id}/matches

Get matches for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | Team ID | [required] |

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_players_get

> crate::models::InlineResponse20031 teams_team_id_players_get(team_id)
GET /teams/{team_id}/players

Get players who have played for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | Team ID | [required] |

### Return type

[**crate::models::InlineResponse20031**](inline_response_200_31.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

