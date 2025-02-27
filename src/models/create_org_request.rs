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
pub struct CreateOrgRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(
        rename = "enable_auto_joining_by_domain",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_auto_joining_by_domain: Option<bool>,
    #[serde(
        rename = "members_must_have_matching_domain",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_must_have_matching_domain: Option<bool>,
    #[serde(rename = "max_users", skip_serializing_if = "Option::is_none")]
    pub max_users: Option<i32>,
}

impl CreateOrgRequest {
    pub fn new(name: String) -> CreateOrgRequest {
        CreateOrgRequest {
            name,
            domain: None,
            enable_auto_joining_by_domain: None,
            members_must_have_matching_domain: None,
            max_users: None,
        }
    }
}
