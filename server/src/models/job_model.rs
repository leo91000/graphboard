use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Job {
    pub id: i64,
    pub queue_name: Option<String>,
    pub task_identifier: String,
    pub payload: Value,
    pub priority: i32,
    pub run_at: DateTime<Utc>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub key: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
    pub revision: i32,
    pub flags: Option<Value>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AddJobData {
    pub task_identifier: String,
    pub payload: Option<Value>,
    pub queue_name: Option<String>,
    pub run_at: Option<DateTime<Utc>>,
    pub max_attempts: Option<i32>,
    pub job_key: Option<String>,
    pub priority: Option<i32>,
    pub flags: Option<Vec<String>>,
    pub job_key_mode: Option<String>,
}
