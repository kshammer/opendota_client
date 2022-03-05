# InlineResponse2004

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**match_id** | Option<**i32**> | Match ID | [optional]
**player_slot** | Option<**i32**> | Which slot the player is in. 0-127 are Radiant, 128-255 are Dire | [optional]
**radiant_win** | Option<**bool**> | Boolean indicating whether Radiant won the match | [optional]
**duration** | Option<**i32**> | Duration of the game in seconds | [optional]
**game_mode** | Option<**i32**> | Integer corresponding to game mode played. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/game_mode.json | [optional]
**lobby_type** | Option<**i32**> | Integer corresponding to lobby type of match. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/lobby_type.json | [optional]
**hero_id** | Option<**i32**> | The ID value of the hero played | [optional]
**start_time** | Option<**i32**> | Start time of the match in seconds elapsed since 1970 | [optional]
**version** | Option<**i32**> | version | [optional]
**kills** | Option<**i32**> | Total kills the player had at the end of the match | [optional]
**deaths** | Option<**i32**> | Total deaths the player had at the end of the match | [optional]
**assists** | Option<**i32**> | Total assists the player had at the end of the match | [optional]
**skill** | Option<**i32**> | Skill bracket assigned by Valve (Normal, High, Very High). If the skill is unknown, will return null. | [optional]
**xp_per_min** | Option<**i32**> | Experience Per Minute obtained by the player | [optional]
**gold_per_min** | Option<**i32**> | Average gold per minute of the player | [optional]
**hero_damage** | Option<**i32**> | Total hero damage to enemy heroes | [optional]
**hero_healing** | Option<**i32**> | Total healing of ally heroes | [optional]
**last_hits** | Option<**i32**> | Total last hits the player had at the end of the match | [optional]
**lane** | Option<**i32**> | Integer corresponding to which lane the player laned in for the match | [optional]
**lane_role** | Option<**i32**> | lane_role | [optional]
**is_roaming** | Option<**bool**> | Boolean describing whether or not the player roamed | [optional]
**cluster** | Option<**i32**> | cluster | [optional]
**leaver_status** | Option<**i32**> | Integer describing whether or not the player left the game. 0: didn't leave. 1: left safely. 2+: Abandoned | [optional]
**party_size** | Option<**i32**> | Size of the players party. If not in a party, will return 1. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


