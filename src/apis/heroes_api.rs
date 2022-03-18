/*
 * OpenDota API
 *
 * # Introduction The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.  You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.  **Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more. 
 *
 * The version of the OpenAPI document: 18.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`heroes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeroesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`heroes_hero_id_durations_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeroesHeroIdDurationsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`heroes_hero_id_item_popularity_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeroesHeroIdItemPopularityGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`heroes_hero_id_matches_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeroesHeroIdMatchesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`heroes_hero_id_matchups_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeroesHeroIdMatchupsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`heroes_hero_id_players_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeroesHeroIdPlayersGetError {
    UnknownValue(serde_json::Value),
}


/// Get hero data
pub async fn heroes_get(configuration: &configuration::Configuration, ) -> Result<Vec<crate::models::HeroesResponse>, Error<HeroesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/heroes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeroesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get hero performance over a range of match durations
pub async fn heroes_hero_id_durations_get(configuration: &configuration::Configuration, hero_id: i32) -> Result<Vec<crate::models::HeroDurationsResponse>, Error<HeroesHeroIdDurationsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/heroes/{hero_id}/durations", local_var_configuration.base_path, hero_id=hero_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeroesHeroIdDurationsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get item popularity of hero categoried by start, early, mid and late game, analyzed from professional games
pub async fn heroes_hero_id_item_popularity_get(configuration: &configuration::Configuration, hero_id: i32) -> Result<crate::models::HeroItemPopularityResponse, Error<HeroesHeroIdItemPopularityGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/heroes/{hero_id}/itemPopularity", local_var_configuration.base_path, hero_id=hero_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeroesHeroIdItemPopularityGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get recent matches with a hero
pub async fn heroes_hero_id_matches_get(configuration: &configuration::Configuration, hero_id: i32) -> Result<Vec<crate::models::HeroMatchesResponse>, Error<HeroesHeroIdMatchesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/heroes/{hero_id}/matches", local_var_configuration.base_path, hero_id=hero_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeroesHeroIdMatchesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get results against other heroes for a hero
pub async fn heroes_hero_id_matchups_get(configuration: &configuration::Configuration, hero_id: i32) -> Result<Vec<crate::models::HeroMatchUpsResponse>, Error<HeroesHeroIdMatchupsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/heroes/{hero_id}/matchups", local_var_configuration.base_path, hero_id=hero_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeroesHeroIdMatchupsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get players who have played this hero
pub async fn heroes_hero_id_players_get(configuration: &configuration::Configuration, hero_id: i32) -> Result<Vec<Vec<serde_json::Value>>, Error<HeroesHeroIdPlayersGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/heroes/{hero_id}/players", local_var_configuration.base_path, hero_id=hero_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeroesHeroIdPlayersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

