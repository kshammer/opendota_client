# MatchResponsePlayers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**match_id** | Option<**i32**> | Match ID | [optional]
**player_slot** | Option<**i32**> | Which slot the player is in. 0-127 are Radiant, 128-255 are Dire | [optional]
**ability_upgrades_arr** | Option<**Vec<i32>**> | An array describing how abilities were upgraded | [optional]
**ability_uses** | Option<[**serde_json::Value**](.md)> | Object containing information on how many times the played used their abilities | [optional]
**ability_targets** | Option<[**serde_json::Value**](.md)> | Object containing information on who the player used their abilities on | [optional]
**damage_targets** | Option<[**serde_json::Value**](.md)> | Object containing information on how and how much damage the player dealt to other heroes | [optional]
**account_id** | Option<**i32**> | account_id | [optional]
**actions** | Option<[**serde_json::Value**](.md)> | Object containing information on how many and what type of actions the player issued to their hero | [optional]
**additional_units** | Option<[**serde_json::Value**](.md)> | Object containing information on additional units the player had under their control | [optional]
**assists** | Option<**i32**> | Number of assists the player had | [optional]
**backpack_0** | Option<**i32**> | Item in backpack slot 0 | [optional]
**backpack_1** | Option<**i32**> | Item in backpack slot 1 | [optional]
**backpack_2** | Option<**i32**> | Item in backpack slot 2 | [optional]
**buyback_log** | Option<[**Vec<crate::models::MatchResponseBuybackLog>**](MatchResponse_buyback_log.md)> | Array containing information about buybacks | [optional]
**camps_stacked** | Option<**i32**> | Number of camps stacked | [optional]
**connection_log** | Option<[**Vec<crate::models::MatchResponseConnectionLog>**](MatchResponse_connection_log.md)> | Array containing information about the player's disconnections and reconnections | [optional]
**creeps_stacked** | Option<**i32**> | Number of creeps stacked | [optional]
**damage** | Option<[**serde_json::Value**](.md)> | Object containing information about damage dealt by the player to different units | [optional]
**damage_inflictor** | Option<[**serde_json::Value**](.md)> | Object containing information about about the sources of this player's damage to heroes | [optional]
**damage_inflictor_received** | Option<[**serde_json::Value**](.md)> | Object containing information about the sources of damage received by this player from heroes | [optional]
**damage_taken** | Option<[**serde_json::Value**](.md)> | Object containing information about from whom the player took damage | [optional]
**deaths** | Option<**i32**> | Number of deaths | [optional]
**denies** | Option<**i32**> | Number of denies | [optional]
**dn_t** | Option<**Vec<i32>**> | Array containing number of denies at different times of the match | [optional]
**gold** | Option<**i32**> | Gold at the end of the game | [optional]
**gold_per_min** | Option<**i32**> | Gold Per Minute obtained by this player | [optional]
**gold_reasons** | Option<[**serde_json::Value**](.md)> | Object containing information on how the player gainined gold over the course of the match | [optional]
**gold_spent** | Option<**i32**> | How much gold the player spent | [optional]
**gold_t** | Option<**Vec<i32>**> | Array containing total gold at different times of the match | [optional]
**hero_damage** | Option<**i32**> | Hero Damage Dealt | [optional]
**hero_healing** | Option<**i32**> | Hero Healing Done | [optional]
**hero_hits** | Option<[**serde_json::Value**](.md)> | Object containing information on how many ticks of damages the hero inflicted with different spells and damage inflictors | [optional]
**hero_id** | Option<**i32**> | The ID value of the hero played | [optional]
**item_0** | Option<**i32**> | Item in the player's first slot | [optional]
**item_1** | Option<**i32**> | Item in the player's second slot | [optional]
**item_2** | Option<**i32**> | Item in the player's third slot | [optional]
**item_3** | Option<**i32**> | Item in the player's fourth slot | [optional]
**item_4** | Option<**i32**> | Item in the player's fifth slot | [optional]
**item_5** | Option<**i32**> | Item in the player's sixth slot | [optional]
**item_uses** | Option<[**serde_json::Value**](.md)> | Object containing information about how many times a player used items | [optional]
**kill_streaks** | Option<[**serde_json::Value**](.md)> | Object containing information about the player's killstreaks | [optional]
**killed** | Option<[**serde_json::Value**](.md)> | Object containing information about what units the player killed | [optional]
**killed_by** | Option<[**serde_json::Value**](.md)> | Object containing information about who killed the player | [optional]
**kills** | Option<**i32**> | Number of kills | [optional]
**kills_log** | Option<[**Vec<crate::models::MatchResponseKillsLog>**](MatchResponse_kills_log.md)> | Array containing information on which hero the player killed at what time | [optional]
**lane_pos** | Option<[**serde_json::Value**](.md)> | Object containing information on lane position | [optional]
**last_hits** | Option<**i32**> | Number of last hits | [optional]
**leaver_status** | Option<**i32**> | Integer describing whether or not the player left the game. 0: didn't leave. 1: left safely. 2+: Abandoned | [optional]
**level** | Option<**i32**> | Level at the end of the game | [optional]
**lh_t** | Option<**Vec<i32>**> | Array describing last hits at each minute in the game | [optional]
**life_state** | Option<[**serde_json::Value**](.md)> | life_state | [optional]
**max_hero_hit** | Option<[**serde_json::Value**](.md)> | Object with information on the highest damage instance the player inflicted | [optional]
**multi_kills** | Option<[**serde_json::Value**](.md)> | Object with information on the number of the number of multikills the player had | [optional]
**obs** | Option<[**serde_json::Value**](.md)> | Object with information on where the player placed observer wards. The location takes the form (outer number, inner number) and are from ~64-192. | [optional]
**obs_left_log** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | obs_left_log | [optional]
**obs_log** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Object containing information on when and where the player placed observer wards | [optional]
**obs_placed** | Option<**i32**> | Total number of observer wards placed | [optional]
**party_id** | Option<**i32**> | party_id | [optional]
**permanent_buffs** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array describing permanent buffs the player had at the end of the game. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/permanent_buffs.json | [optional]
**pings** | Option<**i32**> | Total number of pings | [optional]
**purchase** | Option<[**serde_json::Value**](.md)> | Object containing information on the items the player purchased | [optional]
**purchase_log** | Option<[**Vec<crate::models::MatchResponsePurchaseLog>**](MatchResponse_purchase_log.md)> | Object containing information on when items were purchased | [optional]
**rune_pickups** | Option<**i32**> | Number of runes picked up | [optional]
**runes** | Option<**::std::collections::HashMap<String, i32>**> | Object with information about which runes the player picked up | [optional]
**runes_log** | Option<[**Vec<crate::models::MatchResponseRunesLog>**](MatchResponse_runes_log.md)> | Array with information on when runes were picked up | [optional]
**sen** | Option<[**serde_json::Value**](.md)> | Object with information on where sentries were placed. The location takes the form (outer number, inner number) and are from ~64-192. | [optional]
**sen_left_log** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array containing information on when and where the player placed sentries | [optional]
**sen_log** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array with information on when and where sentries were placed by the player | [optional]
**sen_placed** | Option<**i32**> | How many sentries were placed by the player | [optional]
**stuns** | Option<**f32**> | Total stun duration of all stuns by the player | [optional]
**times** | Option<**Vec<i32>**> | Time in seconds corresponding to the time of entries of other arrays in the match. | [optional]
**tower_damage** | Option<**i32**> | Total tower damage done by the player | [optional]
**xp_per_min** | Option<**i32**> | Experience Per Minute obtained by the player | [optional]
**xp_reasons** | Option<[**serde_json::Value**](.md)> | Object containing information on the sources of this player's experience | [optional]
**xp_t** | Option<**Vec<i32>**> | Experience at each minute of the game | [optional]
**personaname** | Option<**String**> | personaname | [optional]
**name** | Option<**String**> | name | [optional]
**last_login** | Option<**String**> | Time of player's last login | [optional]
**radiant_win** | Option<**bool**> | Boolean indicating whether Radiant won the match | [optional]
**start_time** | Option<**i32**> | Start time of the match in seconds since 1970 | [optional]
**duration** | Option<**i32**> | Duration of the game in seconds | [optional]
**cluster** | Option<**i32**> | cluster | [optional]
**lobby_type** | Option<**i32**> | Integer corresponding to lobby type of match. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/lobby_type.json | [optional]
**game_mode** | Option<**i32**> | Integer corresponding to game mode played. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/game_mode.json | [optional]
**patch** | Option<**i32**> | Integer representing the patch the game was played on | [optional]
**region** | Option<**i32**> | Integer corresponding to the region the game was played on | [optional]
**is_radiant** | Option<**bool**> | Boolean for whether or not the player is on Radiant | [optional]
**win** | Option<**i32**> | Binary integer representing whether or not the player won | [optional]
**lose** | Option<**i32**> | Binary integer representing whether or not the player lost | [optional]
**total_gold** | Option<**i32**> | Total gold at the end of the game | [optional]
**total_xp** | Option<**i32**> | Total experience at the end of the game | [optional]
**kills_per_min** | Option<**f32**> | Number of kills per minute | [optional]
**kda** | Option<**f32**> | kda | [optional]
**abandons** | Option<**i32**> | abandons | [optional]
**neutral_kills** | Option<**i32**> | Total number of neutral creeps killed | [optional]
**tower_kills** | Option<**i32**> | Total number of tower kills the player had | [optional]
**courier_kills** | Option<**i32**> | Total number of courier kills the player had | [optional]
**lane_kills** | Option<**i32**> | Total number of lane creeps killed by the player | [optional]
**hero_kills** | Option<**i32**> | Total number of heroes killed by the player | [optional]
**observer_kills** | Option<**i32**> | Total number of observer wards killed by the player | [optional]
**sentry_kills** | Option<**i32**> | Total number of sentry wards killed by the player | [optional]
**roshan_kills** | Option<**i32**> | Total number of roshan kills (last hit on roshan) the player had | [optional]
**necronomicon_kills** | Option<**i32**> | Total number of Necronomicon creeps killed by the player | [optional]
**ancient_kills** | Option<**i32**> | Total number of Ancient creeps killed by the player | [optional]
**buyback_count** | Option<**i32**> | Total number of buyback the player used | [optional]
**observer_uses** | Option<**i32**> | Number of observer wards used | [optional]
**sentry_uses** | Option<**i32**> | Number of sentry wards used | [optional]
**lane_efficiency** | Option<**f32**> | lane_efficiency | [optional]
**lane_efficiency_pct** | Option<**f32**> | lane_efficiency_pct | [optional]
**lane** | Option<**i32**> | Integer referring to which lane the hero laned in | [optional]
**lane_role** | Option<**i32**> | lane_role | [optional]
**is_roaming** | Option<**bool**> | Boolean referring to whether or not the player roamed | [optional]
**purchase_time** | Option<[**serde_json::Value**](.md)> | Object with information on when the player last purchased an item | [optional]
**first_purchase_time** | Option<[**serde_json::Value**](.md)> | Object with information on when the player first puchased an item | [optional]
**item_win** | Option<[**serde_json::Value**](.md)> | Object with information on whether or not the item won | [optional]
**item_usage** | Option<[**serde_json::Value**](.md)> | Object containing binary integers the tell whether the item was purchased by the player (note: this is always 1) | [optional]
**purchase_tpscroll** | Option<[**serde_json::Value**](.md)> | Total number of TP scrolls purchased by the player | [optional]
**actions_per_min** | Option<**i32**> | Actions per minute | [optional]
**life_state_dead** | Option<**i32**> | life_state_dead | [optional]
**rank_tier** | Option<**i32**> | The rank tier of the player. Tens place indicates rank, ones place indicates stars. | [optional]
**cosmetics** | Option<**Vec<i32>**> | cosmetics | [optional]
**benchmarks** | Option<[**serde_json::Value**](.md)> | Object containing information on certain benchmarks like GPM, XPM, KDA, tower damage, etc | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

