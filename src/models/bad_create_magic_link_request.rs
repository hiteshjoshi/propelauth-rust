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
pub struct BadCreateMagicLinkRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(rename = "redirect_to_url", skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<Vec<String>>,
    #[serde(rename = "expires_in_hours", skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<Vec<String>>,
    #[serde(
        rename = "create_new_user_if_one_doesnt_exist",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_new_user_if_one_doesnt_exist: Option<Vec<String>>,
}

impl BadCreateMagicLinkRequest {
    pub fn new() -> BadCreateMagicLinkRequest {
        BadCreateMagicLinkRequest {
            email: None,
            redirect_to_url: None,
            expires_in_hours: None,
            create_new_user_if_one_doesnt_exist: None,
        }
    }
}
