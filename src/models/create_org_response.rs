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
pub struct CreateOrgResponse {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateOrgResponse {
    pub fn new(org_id: String, name: String) -> CreateOrgResponse {
        CreateOrgResponse {
            org_id,
            name,
        }
    }
}


