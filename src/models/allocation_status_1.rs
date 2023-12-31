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
pub struct AllocationStatus1 {
    /// pointer to the parent allocation or limit that caused this allocation to be implicitly created 
    #[serde(rename = "derivedFrom", skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<String>,
    /// indicates whether configuration and actual state match
    #[serde(rename = "provisioned")]
    pub provisioned: bool,
    #[serde(rename = "notifications")]
    pub notifications: Vec<crate::models::Notification1>,
}

impl AllocationStatus1 {
    pub fn new(provisioned: bool, notifications: Vec<crate::models::Notification1>) -> AllocationStatus1 {
        AllocationStatus1 {
            derived_from: None,
            provisioned,
            notifications,
        }
    }
}


