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
pub struct Task {
    /// false or true depending on health checks (empty if no health checks) 
    #[serde(rename = "healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    /// The IP address of the host the task is running on (not the IP address of the task itself) 
    #[serde(rename = "host")]
    pub host: String,
    /// Optional link to the latest log dump for this task
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
    /// Staging time of the task
    #[serde(rename = "stagedAt")]
    pub staged_at: String,
    /// Start time of the task
    #[serde(rename = "startedAt")]
    pub started_at: String,
    /// Stopped time of the task
    #[serde(rename = "stoppedAt", skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<String>,
    /// Timestamp of the last time the task was updated
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<i64>,
    /// The state the task is in
    #[serde(rename = "state")]
    pub state: State,
}

impl Task {
    pub fn new(host: String, staged_at: String, started_at: String, state: State) -> Task {
        Task {
            healthy: None,
            host,
            logs: None,
            staged_at,
            started_at,
            stopped_at: None,
            last_update: None,
            state,
        }
    }
}

/// The state the task is in
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "DROPPED")]
    Dropped,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "FINISHED")]
    Finished,
    #[serde(rename = "GONE")]
    Gone,
    #[serde(rename = "GONE_BY_OPERATOR")]
    GoneByOperator,
    #[serde(rename = "KILLED")]
    Killed,
    #[serde(rename = "KILLING")]
    Killing,
    #[serde(rename = "LOST")]
    Lost,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STAGING")]
    Staging,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UNREACHABLE")]
    Unreachable,
}

impl Default for State {
    fn default() -> State {
        Self::Dropped
    }
}

