# \PlayersApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**players_account_id_counts_get**](PlayersApi.md#players_account_id_counts_get) | **GET** /players/{account_id}/counts | GET /players/{account_id}/counts
[**players_account_id_get**](PlayersApi.md#players_account_id_get) | **GET** /players/{account_id} | GET /players/{account_id}
[**players_account_id_heroes_get**](PlayersApi.md#players_account_id_heroes_get) | **GET** /players/{account_id}/heroes | GET /players/{account_id}/heroes
[**players_account_id_histograms_field_get**](PlayersApi.md#players_account_id_histograms_field_get) | **GET** /players/{account_id}/histograms/{field} | GET /players/{account_id}/histograms
[**players_account_id_matches_get**](PlayersApi.md#players_account_id_matches_get) | **GET** /players/{account_id}/matches | GET /players/{account_id}/matches
[**players_account_id_peers_get**](PlayersApi.md#players_account_id_peers_get) | **GET** /players/{account_id}/peers | GET /players/{account_id}/peers
[**players_account_id_pros_get**](PlayersApi.md#players_account_id_pros_get) | **GET** /players/{account_id}/pros | GET /players/{account_id}/pros
[**players_account_id_rankings_get**](PlayersApi.md#players_account_id_rankings_get) | **GET** /players/{account_id}/rankings | GET /players/{account_id}/rankings
[**players_account_id_ratings_get**](PlayersApi.md#players_account_id_ratings_get) | **GET** /players/{account_id}/ratings | GET /players/{account_id}/ratings
[**players_account_id_recent_matches_get**](PlayersApi.md#players_account_id_recent_matches_get) | **GET** /players/{account_id}/recentMatches | GET /players/{account_id}/recentMatches
[**players_account_id_refresh_post**](PlayersApi.md#players_account_id_refresh_post) | **POST** /players/{account_id}/refresh | POST /players/{account_id}/refresh
[**players_account_id_totals_get**](PlayersApi.md#players_account_id_totals_get) | **GET** /players/{account_id}/totals | GET /players/{account_id}/totals
[**players_account_id_wardmap_get**](PlayersApi.md#players_account_id_wardmap_get) | **GET** /players/{account_id}/wardmap | GET /players/{account_id}/wardmap
[**players_account_id_wl_get**](PlayersApi.md#players_account_id_wl_get) | **GET** /players/{account_id}/wl | GET /players/{account_id}/wl
[**players_account_id_wordcloud_get**](PlayersApi.md#players_account_id_wordcloud_get) | **GET** /players/{account_id}/wordcloud | GET /players/{account_id}/wordcloud



## players_account_id_counts_get

> crate::models::InlineResponse20010 players_account_id_counts_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/counts

Counts in categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**crate::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_get

> crate::models::InlineResponse2002 players_account_id_get(account_id)
GET /players/{account_id}

Player data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_heroes_get

> Vec<crate::models::InlineResponse2006> players_account_id_heroes_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/heroes

Heroes played

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**Vec<crate::models::InlineResponse2006>**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_histograms_field_get

> Vec<serde_json::Value> players_account_id_histograms_field_get(account_id, field, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/histograms

Distribution of matches in a single stat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**field** | **String** | Field to aggregate on | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_matches_get

> Vec<crate::models::InlineResponse2005> players_account_id_matches_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort, project)
GET /players/{account_id}/matches

Matches played

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |
**project** | Option<**String**> | Fields to project (array) |  |

### Return type

[**Vec<crate::models::InlineResponse2005>**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_peers_get

> Vec<crate::models::InlineResponse2007> players_account_id_peers_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/peers

Players played with

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**Vec<crate::models::InlineResponse2007>**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_pros_get

> Vec<crate::models::InlineResponse2008> players_account_id_pros_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/pros

Pro players played with

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**Vec<crate::models::InlineResponse2008>**](inline_response_200_8.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_rankings_get

> Vec<crate::models::InlineResponse20014> players_account_id_rankings_get(account_id)
GET /players/{account_id}/rankings

Player hero rankings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse20014>**](inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_ratings_get

> Vec<crate::models::InlineResponse20013> players_account_id_ratings_get(account_id)
GET /players/{account_id}/ratings

Player rating history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse20013>**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_recent_matches_get

> Vec<crate::models::InlineResponse2004> players_account_id_recent_matches_get(account_id)
GET /players/{account_id}/recentMatches

Recent matches played

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |

### Return type

[**Vec<crate::models::InlineResponse2004>**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_refresh_post

> serde_json::Value players_account_id_refresh_post(account_id)
POST /players/{account_id}/refresh

Refresh player match history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_totals_get

> Vec<crate::models::InlineResponse2009> players_account_id_totals_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/totals

Totals in stats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**Vec<crate::models::InlineResponse2009>**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_wardmap_get

> crate::models::InlineResponse20011 players_account_id_wardmap_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/wardmap

Wards placed in matches played

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_wl_get

> crate::models::InlineResponse2003 players_account_id_wl_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/wl

Win/Loss count

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_account_id_wordcloud_get

> crate::models::InlineResponse20012 players_account_id_wordcloud_get(account_id, limit, offset, win, patch, game_mode, lobby_type, region, date, lane_role, hero_id, is_radiant, included_account_id, excluded_account_id, with_hero_id, against_hero_id, significant, having, sort)
GET /players/{account_id}/wordcloud

Words said/read in matches played

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | Steam32 account ID | [required] |
**limit** | Option<**i32**> | Number of matches to limit to |  |
**offset** | Option<**i32**> | Number of matches to offset start by |  |
**win** | Option<**i32**> | Whether the player won |  |
**patch** | Option<**i32**> | Patch ID |  |
**game_mode** | Option<**i32**> | Game Mode ID |  |
**lobby_type** | Option<**i32**> | Lobby type ID |  |
**region** | Option<**i32**> | Region ID |  |
**date** | Option<**i32**> | Days previous |  |
**lane_role** | Option<**i32**> | Lane Role ID |  |
**hero_id** | Option<**i32**> | Hero ID |  |
**is_radiant** | Option<**i32**> | Whether the player was radiant |  |
**included_account_id** | Option<**i32**> | Account IDs in the match (array) |  |
**excluded_account_id** | Option<**i32**> | Account IDs not in the match (array) |  |
**with_hero_id** | Option<**i32**> | Hero IDs on the player's team (array) |  |
**against_hero_id** | Option<**i32**> | Hero IDs against the player's team (array) |  |
**significant** | Option<**i32**> | Whether the match was significant for aggregation purposes. Defaults to 1 (true), set this to 0 to return data for non-standard modes/matches. |  |
**having** | Option<**i32**> | The minimum number of games played, for filtering hero stats |  |
**sort** | Option<**String**> | The field to return matches sorted by in descending order |  |

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

