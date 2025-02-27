/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchOrgResponse {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "max_users", skip_serializing_if = "Option::is_none")]
    pub max_users: Option<i64>,
}

impl FetchOrgResponse {
    pub fn new(org_id: String, name: String) -> FetchOrgResponse {
        FetchOrgResponse {
            org_id,
            name,
            max_users: None,
        }
    }
}
