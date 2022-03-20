# Rust API client for OpenDota

# Introduction
The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.

You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.

**Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more.



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 18.0.0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `opendota_client` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://api.opendota.com/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BenchmarksApi* | [**benchmarks_get**](docs/BenchmarksApi.md#benchmarks_get) | **GET** /benchmarks | GET /benchmarks
*ConstantsApi* | [**constants_get**](docs/ConstantsApi.md#constants_get) | **GET** /constants | GET /constants
*ConstantsApi* | [**constants_resource_get**](docs/ConstantsApi.md#constants_resource_get) | **GET** /constants/{resource} | GET /constants
*DistributionsApi* | [**distributions_get**](docs/DistributionsApi.md#distributions_get) | **GET** /distributions | GET /distributions
*ExplorerApi* | [**explorer_get**](docs/ExplorerApi.md#explorer_get) | **GET** /explorer | GET /explorer
*FindMatchesApi* | [**find_matches_get**](docs/FindMatchesApi.md#find_matches_get) | **GET** /findMatches | GET /
*HealthApi* | [**health_get**](docs/HealthApi.md#health_get) | **GET** /health | GET /health
*HeroStatsApi* | [**hero_stats_get**](docs/HeroStatsApi.md#hero_stats_get) | **GET** /heroStats | GET /heroStats
*HeroesApi* | [**heroes_get**](docs/HeroesApi.md#heroes_get) | **GET** /heroes | GET /heroes
*HeroesApi* | [**heroes_hero_id_durations_get**](docs/HeroesApi.md#heroes_hero_id_durations_get) | **GET** /heroes/{hero_id}/durations | GET /heroes/{hero_id}/durations
*HeroesApi* | [**heroes_hero_id_item_popularity_get**](docs/HeroesApi.md#heroes_hero_id_item_popularity_get) | **GET** /heroes/{hero_id}/itemPopularity | GET /heroes/{hero_id}/itemPopularity
*HeroesApi* | [**heroes_hero_id_matches_get**](docs/HeroesApi.md#heroes_hero_id_matches_get) | **GET** /heroes/{hero_id}/matches | GET /heroes/{hero_id}/matches
*HeroesApi* | [**heroes_hero_id_matchups_get**](docs/HeroesApi.md#heroes_hero_id_matchups_get) | **GET** /heroes/{hero_id}/matchups | GET /heroes/{hero_id}/matchups
*HeroesApi* | [**heroes_hero_id_players_get**](docs/HeroesApi.md#heroes_hero_id_players_get) | **GET** /heroes/{hero_id}/players | GET /heroes/{hero_id}/players
*LeaguesApi* | [**leagues_get**](docs/LeaguesApi.md#leagues_get) | **GET** /leagues | GET /leagues
*LeaguesApi* | [**leagues_league_id_get**](docs/LeaguesApi.md#leagues_league_id_get) | **GET** /leagues/{league_id} | GET /leagues/{league_id}
*LeaguesApi* | [**leagues_league_id_matches_get**](docs/LeaguesApi.md#leagues_league_id_matches_get) | **GET** /leagues/{league_id}/matches | GET /leagues/{league_id}/matches
*LeaguesApi* | [**leagues_league_id_teams_get**](docs/LeaguesApi.md#leagues_league_id_teams_get) | **GET** /leagues/{league_id}/teams | GET /leagues/{league_id}/teams
*LiveApi* | [**live_get**](docs/LiveApi.md#live_get) | **GET** /live | GET /live
*MatchesApi* | [**matches_match_id_get**](docs/MatchesApi.md#matches_match_id_get) | **GET** /matches/{match_id} | GET /matches/{match_id}
*MetadataApi* | [**metadata_get**](docs/MetadataApi.md#metadata_get) | **GET** /metadata | GET /metadata
*ParsedMatchesApi* | [**parsed_matches_get**](docs/ParsedMatchesApi.md#parsed_matches_get) | **GET** /parsedMatches | GET /parsedMatches
*PlayersApi* | [**players_account_id_counts_get**](docs/PlayersApi.md#players_account_id_counts_get) | **GET** /players/{account_id}/counts | GET /players/{account_id}/counts
*PlayersApi* | [**players_account_id_get**](docs/PlayersApi.md#players_account_id_get) | **GET** /players/{account_id} | GET /players/{account_id}
*PlayersApi* | [**players_account_id_heroes_get**](docs/PlayersApi.md#players_account_id_heroes_get) | **GET** /players/{account_id}/heroes | GET /players/{account_id}/heroes
*PlayersApi* | [**players_account_id_histograms_field_get**](docs/PlayersApi.md#players_account_id_histograms_field_get) | **GET** /players/{account_id}/histograms/{field} | GET /players/{account_id}/histograms
*PlayersApi* | [**players_account_id_matches_get**](docs/PlayersApi.md#players_account_id_matches_get) | **GET** /players/{account_id}/matches | GET /players/{account_id}/matches
*PlayersApi* | [**players_account_id_peers_get**](docs/PlayersApi.md#players_account_id_peers_get) | **GET** /players/{account_id}/peers | GET /players/{account_id}/peers
*PlayersApi* | [**players_account_id_pros_get**](docs/PlayersApi.md#players_account_id_pros_get) | **GET** /players/{account_id}/pros | GET /players/{account_id}/pros
*PlayersApi* | [**players_account_id_rankings_get**](docs/PlayersApi.md#players_account_id_rankings_get) | **GET** /players/{account_id}/rankings | GET /players/{account_id}/rankings
*PlayersApi* | [**players_account_id_ratings_get**](docs/PlayersApi.md#players_account_id_ratings_get) | **GET** /players/{account_id}/ratings | GET /players/{account_id}/ratings
*PlayersApi* | [**players_account_id_recent_matches_get**](docs/PlayersApi.md#players_account_id_recent_matches_get) | **GET** /players/{account_id}/recentMatches | GET /players/{account_id}/recentMatches
*PlayersApi* | [**players_account_id_refresh_post**](docs/PlayersApi.md#players_account_id_refresh_post) | **POST** /players/{account_id}/refresh | POST /players/{account_id}/refresh
*PlayersApi* | [**players_account_id_totals_get**](docs/PlayersApi.md#players_account_id_totals_get) | **GET** /players/{account_id}/totals | GET /players/{account_id}/totals
*PlayersApi* | [**players_account_id_wardmap_get**](docs/PlayersApi.md#players_account_id_wardmap_get) | **GET** /players/{account_id}/wardmap | GET /players/{account_id}/wardmap
*PlayersApi* | [**players_account_id_wl_get**](docs/PlayersApi.md#players_account_id_wl_get) | **GET** /players/{account_id}/wl | GET /players/{account_id}/wl
*PlayersApi* | [**players_account_id_wordcloud_get**](docs/PlayersApi.md#players_account_id_wordcloud_get) | **GET** /players/{account_id}/wordcloud | GET /players/{account_id}/wordcloud
*PlayersByRankApi* | [**players_by_rank_get**](docs/PlayersByRankApi.md#players_by_rank_get) | **GET** /playersByRank | GET /playersByRank
*ProMatchesApi* | [**pro_matches_get**](docs/ProMatchesApi.md#pro_matches_get) | **GET** /proMatches | GET /proMatches
*ProPlayersApi* | [**pro_players_get**](docs/ProPlayersApi.md#pro_players_get) | **GET** /proPlayers | GET /proPlayers
*PublicMatchesApi* | [**public_matches_get**](docs/PublicMatchesApi.md#public_matches_get) | **GET** /publicMatches | GET /publicMatches
*RankingsApi* | [**rankings_get**](docs/RankingsApi.md#rankings_get) | **GET** /rankings | GET /rankings
*RecordsApi* | [**records_field_get**](docs/RecordsApi.md#records_field_get) | **GET** /records/{field} | GET /records/{field}
*ReplaysApi* | [**replays_get**](docs/ReplaysApi.md#replays_get) | **GET** /replays | GET /replays
*RequestApi* | [**request_job_id_get**](docs/RequestApi.md#request_job_id_get) | **GET** /request/{jobId} | GET /request/{jobId}
*RequestApi* | [**request_match_id_post**](docs/RequestApi.md#request_match_id_post) | **POST** /request/{match_id} | POST /request/{match_id}
*ScenariosApi* | [**scenarios_item_timings_get**](docs/ScenariosApi.md#scenarios_item_timings_get) | **GET** /scenarios/itemTimings | GET /scenarios/itemTimings
*ScenariosApi* | [**scenarios_lane_roles_get**](docs/ScenariosApi.md#scenarios_lane_roles_get) | **GET** /scenarios/laneRoles | GET /scenarios/laneRoles
*ScenariosApi* | [**scenarios_misc_get**](docs/ScenariosApi.md#scenarios_misc_get) | **GET** /scenarios/misc | GET /scenarios/misc
*SchemaApi* | [**schema_get**](docs/SchemaApi.md#schema_get) | **GET** /schema | GET /schema
*SearchApi* | [**search_get**](docs/SearchApi.md#search_get) | **GET** /search | GET /search
*StatusApi* | [**status_get**](docs/StatusApi.md#status_get) | **GET** /status | GET /status
*TeamsApi* | [**teams_get**](docs/TeamsApi.md#teams_get) | **GET** /teams | GET /teams
*TeamsApi* | [**teams_team_id_get**](docs/TeamsApi.md#teams_team_id_get) | **GET** /teams/{team_id} | GET /teams/{team_id}
*TeamsApi* | [**teams_team_id_heroes_get**](docs/TeamsApi.md#teams_team_id_heroes_get) | **GET** /teams/{team_id}/heroes | GET /teams/{team_id}/heroes
*TeamsApi* | [**teams_team_id_matches_get**](docs/TeamsApi.md#teams_team_id_matches_get) | **GET** /teams/{team_id}/matches | GET /teams/{team_id}/matches
*TeamsApi* | [**teams_team_id_players_get**](docs/TeamsApi.md#teams_team_id_players_get) | **GET** /teams/{team_id}/players | GET /teams/{team_id}/players


## Documentation For Models

 - [BenchmarksResponse](docs/BenchmarksResponse.md)
 - [BenchmarksResponseResult](docs/BenchmarksResponseResult.md)
 - [BenchmarksResponseResultGoldPerMin](docs/BenchmarksResponseResultGoldPerMin.md)
 - [DistributionsResponse](docs/DistributionsResponse.md)
 - [DistributionsResponseCountryMmr](docs/DistributionsResponseCountryMmr.md)
 - [DistributionsResponseCountryMmrFields](docs/DistributionsResponseCountryMmrFields.md)
 - [DistributionsResponseCountryMmrRows](docs/DistributionsResponseCountryMmrRows.md)
 - [DistributionsResponseMmr](docs/DistributionsResponseMmr.md)
 - [DistributionsResponseRanks](docs/DistributionsResponseRanks.md)
 - [DistributionsResponseRanksFields](docs/DistributionsResponseRanksFields.md)
 - [DistributionsResponseRanksRows](docs/DistributionsResponseRanksRows.md)
 - [DistributionsResponseRanksSum](docs/DistributionsResponseRanksSum.md)
 - [HeroDurationsResponse](docs/HeroDurationsResponse.md)
 - [HeroItemPopularityResponse](docs/HeroItemPopularityResponse.md)
 - [HeroItemPopularityResponseEarlyGameItems](docs/HeroItemPopularityResponseEarlyGameItems.md)
 - [HeroItemPopularityResponseLateGameItems](docs/HeroItemPopularityResponseLateGameItems.md)
 - [HeroItemPopularityResponseMidGameItems](docs/HeroItemPopularityResponseMidGameItems.md)
 - [HeroItemPopularityResponseStartGameItems](docs/HeroItemPopularityResponseStartGameItems.md)
 - [HeroMatchUpsResponse](docs/HeroMatchUpsResponse.md)
 - [HeroMatchesResponse](docs/HeroMatchesResponse.md)
 - [HeroStatsResponse](docs/HeroStatsResponse.md)
 - [HeroesResponse](docs/HeroesResponse.md)
 - [ItemTimingsResponse](docs/ItemTimingsResponse.md)
 - [LaneRolesResponse](docs/LaneRolesResponse.md)
 - [LeagueIdResponse](docs/LeagueIdResponse.md)
 - [LeagueMatchesResponse](docs/LeagueMatchesResponse.md)
 - [LeagueTeamsResponse](docs/LeagueTeamsResponse.md)
 - [LeaguesResponse](docs/LeaguesResponse.md)
 - [MatchResponse](docs/MatchResponse.md)
 - [MatchResponseBuybackLog](docs/MatchResponseBuybackLog.md)
 - [MatchResponseChat](docs/MatchResponseChat.md)
 - [MatchResponseConnectionLog](docs/MatchResponseConnectionLog.md)
 - [MatchResponseDraftTimings](docs/MatchResponseDraftTimings.md)
 - [MatchResponseKillsLog](docs/MatchResponseKillsLog.md)
 - [MatchResponsePlayers](docs/MatchResponsePlayers.md)
 - [MatchResponsePurchaseLog](docs/MatchResponsePurchaseLog.md)
 - [MatchResponseRunesLog](docs/MatchResponseRunesLog.md)
 - [MetadataResponse](docs/MetadataResponse.md)
 - [MetadataResponseCheese](docs/MetadataResponseCheese.md)
 - [MiscResponse](docs/MiscResponse.md)
 - [ParsedMatchesResponse](docs/ParsedMatchesResponse.md)
 - [PlayerCountsResponse](docs/PlayerCountsResponse.md)
 - [PlayerHeroRankingsResponse](docs/PlayerHeroRankingsResponse.md)
 - [PlayerHeroesResponse](docs/PlayerHeroesResponse.md)
 - [PlayerMatchesResponse](docs/PlayerMatchesResponse.md)
 - [PlayerPeersResponse](docs/PlayerPeersResponse.md)
 - [PlayerProsResponse](docs/PlayerProsResponse.md)
 - [PlayerRatingsResponse](docs/PlayerRatingsResponse.md)
 - [PlayerRecentMatchesResponse](docs/PlayerRecentMatchesResponse.md)
 - [PlayerResponse](docs/PlayerResponse.md)
 - [PlayerResponseMmrEstimate](docs/PlayerResponseMmrEstimate.md)
 - [PlayerResponseProfile](docs/PlayerResponseProfile.md)
 - [PlayerStatsResponse](docs/PlayerStatsResponse.md)
 - [PlayerWardMapResponse](docs/PlayerWardMapResponse.md)
 - [PlayerWinLossResponse](docs/PlayerWinLossResponse.md)
 - [PlayerWordCloudResponse](docs/PlayerWordCloudResponse.md)
 - [PlayersByRankResponse](docs/PlayersByRankResponse.md)
 - [ProMatchesResponse](docs/ProMatchesResponse.md)
 - [ProPlayerResponse](docs/ProPlayerResponse.md)
 - [PublicMatchesResponse](docs/PublicMatchesResponse.md)
 - [RankingsResponse](docs/RankingsResponse.md)
 - [RankingsResponseRankings](docs/RankingsResponseRankings.md)
 - [RecordsResponse](docs/RecordsResponse.md)
 - [ReplaysResponse](docs/ReplaysResponse.md)
 - [SchemaResponse](docs/SchemaResponse.md)
 - [SearchResponse](docs/SearchResponse.md)
 - [TeamHeroesResponse](docs/TeamHeroesResponse.md)
 - [TeamPlayersResponse](docs/TeamPlayersResponse.md)
 - [TeamsIdResponse](docs/TeamsIdResponse.md)
 - [TeamsMatchesResponse](docs/TeamsMatchesResponse.md)
 - [TeamsResponse](docs/TeamsResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



