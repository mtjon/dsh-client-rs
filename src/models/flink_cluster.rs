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
pub struct FlinkCluster {
    /// Flink version
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "jobManager", skip_serializing_if = "Option::is_none")]
    pub job_manager: Option<Box<crate::models::FlinkJobManager>>,
    #[serde(rename = "taskManager", skip_serializing_if = "Option::is_none")]
    pub task_manager: Option<Box<crate::models::FlinkTaskManager>>,
    /// Network zone this cluster needs to run in. /components/schemas/Zone contains a list of available network zones in this platform.
    #[serde(rename = "zone")]
    pub zone: String,
}

impl FlinkCluster {
    pub fn new(version: String, zone: String) -> FlinkCluster {
        FlinkCluster {
            version,
            job_manager: None,
            task_manager: None,
            zone,
        }
    }
}


