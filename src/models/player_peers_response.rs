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
pub struct PlayerPeersResponse {
    /// account_id
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,
    /// last_played
    #[serde(rename = "last_played", skip_serializing_if = "Option::is_none")]
    pub last_played: Option<i32>,
    /// win
    #[serde(rename = "win", skip_serializing_if = "Option::is_none")]
    pub win: Option<i32>,
    /// games
    #[serde(rename = "games", skip_serializing_if = "Option::is_none")]
    pub games: Option<i32>,
    /// with_win
    #[serde(rename = "with_win", skip_serializing_if = "Option::is_none")]
    pub with_win: Option<i32>,
    /// with_games
    #[serde(rename = "with_games", skip_serializing_if = "Option::is_none")]
    pub with_games: Option<i32>,
    /// against_win
    #[serde(rename = "against_win", skip_serializing_if = "Option::is_none")]
    pub against_win: Option<i32>,
    /// against_games
    #[serde(rename = "against_games", skip_serializing_if = "Option::is_none")]
    pub against_games: Option<i32>,
    /// with_gpm_sum
    #[serde(rename = "with_gpm_sum", skip_serializing_if = "Option::is_none")]
    pub with_gpm_sum: Option<i32>,
    /// with_xpm_sum
    #[serde(rename = "with_xpm_sum", skip_serializing_if = "Option::is_none")]
    pub with_xpm_sum: Option<i32>,
    /// personaname
    #[serde(rename = "personaname", skip_serializing_if = "Option::is_none")]
    pub personaname: Option<String>,
    /// name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// is_contributor
    #[serde(rename = "is_contributor", skip_serializing_if = "Option::is_none")]
    pub is_contributor: Option<bool>,
    /// last_login
    #[serde(rename = "last_login", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// avatar
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// avatarfull
    #[serde(rename = "avatarfull", skip_serializing_if = "Option::is_none")]
    pub avatarfull: Option<String>,
}

impl PlayerPeersResponse {
    pub fn new() -> PlayerPeersResponse {
        PlayerPeersResponse {
            account_id: None,
            last_played: None,
            win: None,
            games: None,
            with_win: None,
            with_games: None,
            against_win: None,
            against_games: None,
            with_gpm_sum: None,
            with_xpm_sum: None,
            personaname: None,
            name: None,
            is_contributor: None,
            last_login: None,
            avatar: None,
            avatarfull: None,
        }
    }
}

