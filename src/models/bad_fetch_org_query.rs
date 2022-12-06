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
pub struct BadFetchOrgQuery {
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Vec<String>>,
    #[serde(rename = "page_number", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<Vec<String>>,
}

impl BadFetchOrgQuery {
    pub fn new() -> BadFetchOrgQuery {
        BadFetchOrgQuery {
            page_size: None,
            page_number: None,
        }
    }
}


