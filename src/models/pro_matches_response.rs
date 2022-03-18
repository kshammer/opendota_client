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
pub struct ProMatchesResponse {
    /// Used to identify individual matches, e.g. 3703866531
    #[serde(rename = "match_id", skip_serializing_if = "Option::is_none")]
    pub match_id: Option<i32>,
    /// Length of the match
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Unix timestamp of when the match began
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i32>,
    /// The Radiant's team_id
    #[serde(rename = "radiant_team_id", skip_serializing_if = "Option::is_none")]
    pub radiant_team_id: Option<i32>,
    /// The Radiant's team name
    #[serde(rename = "radiant_name", skip_serializing_if = "Option::is_none")]
    pub radiant_name: Option<String>,
    /// The Dire's team_id
    #[serde(rename = "dire_team_id", skip_serializing_if = "Option::is_none")]
    pub dire_team_id: Option<i32>,
    /// The Dire's team name
    #[serde(rename = "dire_name", skip_serializing_if = "Option::is_none")]
    pub dire_name: Option<String>,
    /// Identifier for the league the match took place in
    #[serde(rename = "leagueid", skip_serializing_if = "Option::is_none")]
    pub leagueid: Option<i32>,
    /// Name of league the match took place in
    #[serde(rename = "league_name", skip_serializing_if = "Option::is_none")]
    pub league_name: Option<String>,
    /// Identifier for the series of the match
    #[serde(rename = "series_id", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<i32>,
    /// Type of series the match was
    #[serde(rename = "series_type", skip_serializing_if = "Option::is_none")]
    pub series_type: Option<i32>,
    /// Number of kills the Radiant team had when the match ended
    #[serde(rename = "radiant_score", skip_serializing_if = "Option::is_none")]
    pub radiant_score: Option<i32>,
    /// Number of kills the Dire team had when the match ended
    #[serde(rename = "dire_score", skip_serializing_if = "Option::is_none")]
    pub dire_score: Option<i32>,
    /// Whether or not the Radiant won the match
    #[serde(rename = "radiant_win", skip_serializing_if = "Option::is_none")]
    pub radiant_win: Option<bool>,
    /// Whether the team/player/hero was on Radiant
    #[serde(rename = "radiant", skip_serializing_if = "Option::is_none")]
    pub radiant: Option<bool>,
}

impl ProMatchesResponse {
    pub fn new() -> ProMatchesResponse {
        ProMatchesResponse {
            match_id: None,
            duration: None,
            start_time: None,
            radiant_team_id: None,
            radiant_name: None,
            dire_team_id: None,
            dire_name: None,
            leagueid: None,
            league_name: None,
            series_id: None,
            series_type: None,
            radiant_score: None,
            dire_score: None,
            radiant_win: None,
            radiant: None,
        }
    }
}

