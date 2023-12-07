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
pub struct AppCatalogManifest {
    /// creation timestamp of the secret
    #[serde(rename = "lastModified")]
    pub last_modified: f32,
    #[serde(rename = "payload")]
    pub payload: String,
    #[serde(rename = "draft")]
    pub draft: bool,
}

impl AppCatalogManifest {
    pub fn new(last_modified: f32, payload: String, draft: bool) -> AppCatalogManifest {
        AppCatalogManifest {
            last_modified,
            payload,
            draft,
        }
    }
}

