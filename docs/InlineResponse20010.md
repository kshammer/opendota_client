# InlineResponse20010

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**leaver_status** | Option<[**serde_json::Value**](.md)> | Integer describing whether or not the player left the game. 0: didn't leave. 1: left safely. 2+: Abandoned | [optional]
**game_mode** | Option<[**serde_json::Value**](.md)> | Integer corresponding to game mode played. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/game_mode.json | [optional]
**lobby_type** | Option<[**serde_json::Value**](.md)> | Integer corresponding to lobby type of match. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/lobby_type.json | [optional]
**lane_role** | Option<[**serde_json::Value**](.md)> | lane_role | [optional]
**region** | Option<[**serde_json::Value**](.md)> | Integer corresponding to the region the game was played on | [optional]
**patch** | Option<[**serde_json::Value**](.md)> | patch | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


