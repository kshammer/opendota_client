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
pub struct InlineResponse20020CountryMmrFields {
    /// name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// tableID
    #[serde(rename = "tableID", skip_serializing_if = "Option::is_none")]
    pub table_id: Option<i32>,
    /// columnID
    #[serde(rename = "columnID", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<i32>,
    /// dataTypeID
    #[serde(rename = "dataTypeID", skip_serializing_if = "Option::is_none")]
    pub data_type_id: Option<i32>,
    /// dataTypeSize
    #[serde(rename = "dataTypeSize", skip_serializing_if = "Option::is_none")]
    pub data_type_size: Option<i32>,
    /// dataTypeModifier
    #[serde(rename = "dataTypeModifier", skip_serializing_if = "Option::is_none")]
    pub data_type_modifier: Option<i32>,
    /// format
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl InlineResponse20020CountryMmrFields {
    pub fn new() -> InlineResponse20020CountryMmrFields {
        InlineResponse20020CountryMmrFields {
            name: None,
            table_id: None,
            column_id: None,
            data_type_id: None,
            data_type_size: None,
            data_type_modifier: None,
            format: None,
        }
    }
}


