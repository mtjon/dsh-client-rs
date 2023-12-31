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
pub struct ClientSecret {
    /// the secret value
    #[serde(rename = "value")]
    pub value: String,
    /// creation timestamp of the secret
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f32>,
}

impl ClientSecret {
    pub fn new(value: String) -> ClientSecret {
        ClientSecret {
            value,
            created_date: None,
        }
    }
}


