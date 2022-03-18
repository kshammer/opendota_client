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
pub struct LeagueIdResponse {
    /// leagueid
    #[serde(rename = "leagueid", skip_serializing_if = "Option::is_none")]
    pub leagueid: Option<i32>,
    /// ticket
    #[serde(rename = "ticket", skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    /// banner
    #[serde(rename = "banner", skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// tier
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LeagueIdResponse {
    pub fn new() -> LeagueIdResponse {
        LeagueIdResponse {
            leagueid: None,
            ticket: None,
            banner: None,
            tier: None,
            name: None,
        }
    }
}


