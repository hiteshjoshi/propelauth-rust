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
pub struct AuthTokenVerificationMetadata {
    #[serde(rename = "public_key_pem")]
    pub public_key_pem: String,
}

impl AuthTokenVerificationMetadata {
    pub fn new(public_key_pem: String) -> AuthTokenVerificationMetadata {
        AuthTokenVerificationMetadata {
            public_key_pem,
        }
    }
}


