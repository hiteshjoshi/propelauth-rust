/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.8
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BadUpdateOrgRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
}

impl BadUpdateOrgRequest {
    pub fn new() -> BadUpdateOrgRequest {
        BadUpdateOrgRequest {
            name: None,
        }
    }
}


