/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.6.6
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Metrics : metrics endpoint which will be scraped by the platform.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    /// The TCP port for the metrics endpoint 
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The HTTP path for the metrics endpoint 
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl Metrics {
    /// metrics endpoint which will be scraped by the platform.
    pub fn new() -> Metrics {
        Metrics {
            port: None,
            path: None,
        }
    }
}

