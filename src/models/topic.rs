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
pub struct Topic {
    #[serde(rename = "partitions")]
    pub partitions: i32,
    #[serde(rename = "replicationFactor")]
    pub replication_factor: i32,
    #[serde(rename = "kafkaProperties", skip_serializing_if = "Option::is_none")]
    pub kafka_properties: Option<::std::collections::HashMap<String, String>>,
}

impl Topic {
    pub fn new(partitions: i32, replication_factor: i32) -> Topic {
        Topic {
            partitions,
            replication_factor,
            kafka_properties: None,
        }
    }
}


