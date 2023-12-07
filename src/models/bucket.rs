/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.6.6
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "versioned")]
    pub versioned: bool,
    #[serde(rename = "encrypted")]
    pub encrypted: bool,
}

impl Bucket {
    pub fn new(versioned: bool, encrypted: bool) -> Bucket {
        Bucket {
            versioned,
            encrypted,
        }
    }
}


