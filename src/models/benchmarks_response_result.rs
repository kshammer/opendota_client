/*
 * OpenDota API
 *
 * # Introduction The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.  You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.  **Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more. 
 *
 * The version of the OpenAPI document: 18.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BenchmarksResponseResult : result



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BenchmarksResponseResult {
    #[serde(rename = "gold_per_min", skip_serializing_if = "Option::is_none")]
    pub gold_per_min: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
    #[serde(rename = "xp_per_min", skip_serializing_if = "Option::is_none")]
    pub xp_per_min: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
    #[serde(rename = "kills_per_min", skip_serializing_if = "Option::is_none")]
    pub kills_per_min: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
    #[serde(rename = "last_hits_per_min", skip_serializing_if = "Option::is_none")]
    pub last_hits_per_min: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
    #[serde(rename = "hero_damage_per_min", skip_serializing_if = "Option::is_none")]
    pub hero_damage_per_min: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
    #[serde(rename = "hero_healing_per_min", skip_serializing_if = "Option::is_none")]
    pub hero_healing_per_min: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
    #[serde(rename = "tower_damage", skip_serializing_if = "Option::is_none")]
    pub tower_damage: Option<Vec<crate::models::BenchmarksResponseResultGoldPerMin>>,
}

impl BenchmarksResponseResult {
    /// result
    pub fn new() -> BenchmarksResponseResult {
        BenchmarksResponseResult {
            gold_per_min: None,
            xp_per_min: None,
            kills_per_min: None,
            last_hits_per_min: None,
            hero_damage_per_min: None,
            hero_healing_per_min: None,
            tower_damage: None,
        }
    }
}


