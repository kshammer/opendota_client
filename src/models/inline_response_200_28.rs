/*
 * OpenDota API
 *
 * # Introduction The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.  You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.  **Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more. 
 *
 * The version of the OpenAPI document: 18.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse20028 {
    #[serde(rename = "start_game_items", skip_serializing_if = "Option::is_none")]
    pub start_game_items: Option<Box<crate::models::InlineResponse20028StartGameItems>>,
    #[serde(rename = "early_game_items", skip_serializing_if = "Option::is_none")]
    pub early_game_items: Option<Box<crate::models::InlineResponse20028EarlyGameItems>>,
    #[serde(rename = "mid_game_items", skip_serializing_if = "Option::is_none")]
    pub mid_game_items: Option<Box<crate::models::InlineResponse20028MidGameItems>>,
    #[serde(rename = "late_game_items", skip_serializing_if = "Option::is_none")]
    pub late_game_items: Option<Box<crate::models::InlineResponse20028LateGameItems>>,
}

impl InlineResponse20028 {
    pub fn new() -> InlineResponse20028 {
        InlineResponse20028 {
            start_game_items: None,
            early_game_items: None,
            mid_game_items: None,
            late_game_items: None,
        }
    }
}


