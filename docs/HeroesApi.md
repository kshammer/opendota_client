# \HeroesApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**heroes_get**](HeroesApi.md#heroes_get) | **GET** /heroes | GET /heroes
[**heroes_hero_id_durations_get**](HeroesApi.md#heroes_hero_id_durations_get) | **GET** /heroes/{hero_id}/durations | GET /heroes/{hero_id}/durations
[**heroes_hero_id_item_popularity_get**](HeroesApi.md#heroes_hero_id_item_popularity_get) | **GET** /heroes/{hero_id}/itemPopularity | GET /heroes/{hero_id}/itemPopularity
[**heroes_hero_id_matches_get**](HeroesApi.md#heroes_hero_id_matches_get) | **GET** /heroes/{hero_id}/matches | GET /heroes/{hero_id}/matches
[**heroes_hero_id_matchups_get**](HeroesApi.md#heroes_hero_id_matchups_get) | **GET** /heroes/{hero_id}/matchups | GET /heroes/{hero_id}/matchups
[**heroes_hero_id_players_get**](HeroesApi.md#heroes_hero_id_players_get) | **GET** /heroes/{hero_id}/players | GET /heroes/{hero_id}/players



## heroes_get

> Vec<crate::models::InlineResponse20024> heroes_get()
GET /heroes

Get hero data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20024>**](inline_response_200_24.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heroes_hero_id_durations_get

> Vec<crate::models::InlineResponse20027> heroes_hero_id_durations_get(hero_id)
GET /heroes/{hero_id}/durations

Get hero performance over a range of match durations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hero_id** | **i32** | Hero ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse20027>**](inline_response_200_27.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heroes_hero_id_item_popularity_get

> crate::models::InlineResponse20028 heroes_hero_id_item_popularity_get(hero_id)
GET /heroes/{hero_id}/itemPopularity

Get item popularity of hero categoried by start, early, mid and late game, analyzed from professional games

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hero_id** | **i32** | Hero ID | [required] |

### Return type

[**crate::models::InlineResponse20028**](inline_response_200_28.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heroes_hero_id_matches_get

> Vec<crate::models::InlineResponse20016> heroes_hero_id_matches_get(hero_id)
GET /heroes/{hero_id}/matches

Get recent matches with a hero

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hero_id** | **i32** | Hero ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse20016>**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heroes_hero_id_matchups_get

> Vec<crate::models::InlineResponse20026> heroes_hero_id_matchups_get(hero_id)
GET /heroes/{hero_id}/matchups

Get results against other heroes for a hero

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hero_id** | **i32** | Hero ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse20026>**](inline_response_200_26.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heroes_hero_id_players_get

> Vec<Vec<serde_json::Value>> heroes_hero_id_players_get(hero_id)
GET /heroes/{hero_id}/players

Get players who have played this hero

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hero_id** | **i32** | Hero ID | [required] |

### Return type

[**Vec<Vec<serde_json::Value>>**](array.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

