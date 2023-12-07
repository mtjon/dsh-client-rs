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
pub struct ThirdPartyBucketConcessionStatus {
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<crate::models::ThirdPartyBucketConcessionConfiguration>>,
    #[serde(rename = "actual", skip_serializing_if = "Option::is_none")]
    pub actual: Option<Box<crate::models::ThirdPartyBucketConcession>>,
    #[serde(rename = "status")]
    pub status: Box<crate::models::AllocationStatus>,
}

impl ThirdPartyBucketConcessionStatus {
    pub fn new(status: crate::models::AllocationStatus) -> ThirdPartyBucketConcessionStatus {
        ThirdPartyBucketConcessionStatus {
            configuration: None,
            actual: None,
            status: Box::new(status),
        }
    }
}

