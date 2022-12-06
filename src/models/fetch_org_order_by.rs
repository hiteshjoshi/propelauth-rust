/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.8
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FetchOrgOrderBy {
    #[serde(rename = "CREATED_AT_ASC")]
    CreatedAtAsc,
    #[serde(rename = "CREATED_AT_DESC")]
    CreatedAtDesc,
    #[serde(rename = "NAME")]
    Name,

}

impl ToString for FetchOrgOrderBy {
    fn to_string(&self) -> String {
        match self {
            Self::CreatedAtAsc => String::from("CREATED_AT_ASC"),
            Self::CreatedAtDesc => String::from("CREATED_AT_DESC"),
            Self::Name => String::from("NAME"),
        }
    }
}

impl Default for FetchOrgOrderBy {
    fn default() -> FetchOrgOrderBy {
        Self::CreatedAtAsc
    }
}




