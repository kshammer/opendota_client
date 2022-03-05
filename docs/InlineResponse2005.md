# InlineResponse2005

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
**start_time** | Option<**i32**> | Time the game started in seconds since 1970 | [optional]
**version** | Option<**i32**> | version | [optional]
**kills** | Option<**i32**> | Total kills the player had at the end of the game | [optional]
**deaths** | Option<**i32**> | Total deaths the player had at the end of the game | [optional]
**assists** | Option<**i32**> | Total assists the player had at the end of the game | [optional]
**skill** | Option<**i32**> | Skill bracket assigned by Valve (Normal, High, Very High) | [optional]
**leaver_status** | Option<**i32**> | Integer describing whether or not the player left the game. 0: didn't leave. 1: left safely. 2+: Abandoned | [optional]
**party_size** | Option<**i32**> | Size of the player's party | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


