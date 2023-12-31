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
pub struct AppCatalogAppConfiguration {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "manifestUrn")]
    pub manifest_urn: String,
    #[serde(rename = "stopped")]
    pub stopped: bool,
    /// configuration parameters to be used in AppCatalog manifest
    #[serde(rename = "configuration")]
    pub configuration: ::std::collections::HashMap<String, String>,
}

impl AppCatalogAppConfiguration {
    pub fn new(name: String, manifest_urn: String, stopped: bool, configuration: ::std::collections::HashMap<String, String>) -> AppCatalogAppConfiguration {
        AppCatalogAppConfiguration {
            name,
            manifest_urn,
            stopped,
            configuration,
        }
    }
}


