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
pub struct BucketWatch {
    #[serde(rename = "bucket")]
    pub bucket: String,
}

impl BucketWatch {
    pub fn new(bucket: String) -> BucketWatch {
        BucketWatch {
            bucket,
        }
    }
}


