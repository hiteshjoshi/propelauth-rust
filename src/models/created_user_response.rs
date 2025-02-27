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
pub struct CreatedUserResponse {
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl CreatedUserResponse {
    pub fn new(user_id: String) -> CreatedUserResponse {
        CreatedUserResponse { user_id }
    }
}
