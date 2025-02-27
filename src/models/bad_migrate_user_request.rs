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
pub struct BadMigrateUserRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(rename = "existing_user_id", skip_serializing_if = "Option::is_none")]
    pub existing_user_id: Option<Vec<String>>,
    #[serde(
        rename = "existing_password_hash",
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_password_hash: Option<Vec<String>>,
    #[serde(
        rename = "existing_mfa_base32_encoded_secret",
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_mfa_base32_encoded_secret: Option<Vec<String>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<Vec<String>>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Vec<String>>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Vec<String>>,
}

impl BadMigrateUserRequest {
    pub fn new() -> BadMigrateUserRequest {
        BadMigrateUserRequest {
            email: None,
            existing_user_id: None,
            existing_password_hash: None,
            existing_mfa_base32_encoded_secret: None,
            username: None,
            first_name: None,
            last_name: None,
        }
    }
}
