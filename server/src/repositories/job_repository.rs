use crate::config::GRAPHILE_WORKER_SCHEMA;
use crate::models::{AddJobData, Job};
use crate::repositories::{
    Order, Pagination, RepositoryError, RepositoryOrder, RepositoryPagination, ToSqlIdent,
};
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum JobOrderField {
    TaskIdentifier,
    RunAt,
}

impl ToSqlIdent for JobOrderField {
    fn sql_ident(&self) -> String {
        match self {
            JobOrderField::TaskIdentifier => String::from("task_identifier"),
            JobOrderField::RunAt => String::from("run_at"),
        }
    }
}

impl Default for JobOrderField {
    fn default() -> Self {
        JobOrderField::TaskIdentifier
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FindJobsFilters {
    task_identifier: Option<String>,
}

impl Default for FindJobsFilters {
    fn default() -> Self {
        FindJobsFilters {
            task_identifier: None,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FindJobsParams {
    pub order: Option<RepositoryOrder<JobOrderField>>,
    pub pagination: Option<RepositoryPagination>,
    pub filters: Option<FindJobsFilters>,
}

impl FindJobsParams {
    pub fn filters(self) -> FindJobsFilters {
        self.filters.unwrap_or(FindJobsFilters::default())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FindJobsResult {
    jobs: Vec<Job>,
    count: i64,
}

impl TryFrom<Row> for FindJobsResult {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(FindJobsResult {
            jobs: serde_json::from_str(row.try_get("jobs")?)?,
            count: row.try_get("count")?,
        })
    }
}

impl TryFrom<Row> for Job {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Job {
            id: row.try_get("id")?,
            queue_name: row.try_get("queue_name")?,
            task_identifier: row.try_get("task_identifier")?,
            payload: row.try_get("payload")?,
            priority: row.try_get("priority")?,
            run_at: row.try_get("run_at")?,
            attempts: row.try_get("attempts")?,
            max_attempts: row.try_get("max_attempts")?,
            last_error: row.try_get("last_error")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            key: row.try_get("key")?,
            locked_at: row.try_get("locked_at")?,
            locked_by: row.try_get("locked_by")?,
            revision: row.try_get("revision")?,
            flags: row.try_get("flags")?,
        })
    }
}

pub async fn find_jobs(
    client: &Client,
    params: FindJobsParams,
) -> Result<FindJobsResult, RepositoryError> {
    let query = format!(
        "select j.* from {}.jobs j where j.task_identifier ilike concat('%', $1::text, '%')",
        *GRAPHILE_WORKER_SCHEMA
    );

    let stmt = format!(
        "select coalesce((select json_agg(data)::text from ({query} order by {} limit {} offset {}) data), '[]') as jobs, coalesce((select count(c.*) from ({query}) c), 0) as count",
        params.order.order(),
        params.pagination.limit(),
        params.pagination.offset(),
        query = query,
    );

    let result = client
        .query_one(
            stmt.as_str(),
            &[&params
                .filters()
                .task_identifier
                .unwrap_or("".to_string())
                .as_str()],
        )
        .await?
        .try_into()?;
    Ok(result)
}

pub async fn add_job(client: &Client, data: AddJobData) -> Result<Job, RepositoryError> {
    let query = format!("select j.* from {}.add_job($1::text, $2::json, $3::text, $4::timestamptz, $5::integer, $6::text, $7::integer, $8::text[], $9::text) j", *GRAPHILE_WORKER_SCHEMA);

    let job = client
        .query_one(
            &query,
            &[
                &data.task_identifier,
                &data.payload,
                &data.queue_name,
                &data.run_at,
                &data.max_attempts,
                &data.job_key,
                &data.priority,
                &data.flags,
                &data.job_key_mode,
            ],
        )
        .await?
        .try_into()?;

    Ok(job)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteJobsResult {
    completed_jobs: Vec<Job>,
}

impl TryFrom<Row> for CompleteJobsResult {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(CompleteJobsResult {
            completed_jobs: serde_json::from_str(row.try_get("completed_jobs")?)?,
        })
    }
}

pub async fn complete_jobs<I: AsRef<[i64]>>(
    client: &Client,
    job_ids: I,
) -> Result<CompleteJobsResult, RepositoryError> {
    let query = format!(
        "select json_agg(cj)::text completed_jobs from {}.complete_jobs($1::bigint[]) cj",
        *GRAPHILE_WORKER_SCHEMA
    );

    let results = client
        .query_one(&query, &[&job_ids.as_ref()])
        .await?
        .try_into()?;
    Ok(results)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermanentlyFailJobsResult {
    permanently_failed_jobs: Vec<Job>,
}

impl TryFrom<Row> for PermanentlyFailJobsResult {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(PermanentlyFailJobsResult {
            permanently_failed_jobs: serde_json::from_str(row.try_get("permanently_failed_jobs")?)?,
        })
    }
}

pub async fn permanently_fail_jobs<I: AsRef<[i64]>, E: AsRef<str>>(
    client: &Client,
    job_ids: I,
    error_messages: E,
) -> Result<PermanentlyFailJobsResult, RepositoryError> {
    let query = format!("select json_agg(f)::text permanently_failed_jobs from {}.permanently_fail_jobs($1::bigint[], $2::text) f", *GRAPHILE_WORKER_SCHEMA);

    let jobs = client
        .query_one(&query, &[&job_ids.as_ref(), &error_messages.as_ref()])
        .await?
        .try_into()?;

    Ok(jobs)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RescheduleJobsData {
    pub job_ids: Vec<i64>,
    pub run_at: Option<DateTime<Utc>>,
    pub priority: Option<i32>,
    pub attempts: Option<u32>,
    pub max_attempts: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RescheduleJobsResult {
    pub rescheduled_jobs: Vec<Job>,
}

impl TryFrom<Row> for RescheduleJobsResult {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(RescheduleJobsResult {
            rescheduled_jobs: serde_json::from_str(row.try_get("rescheduled_jobs")?)?,
        })
    }
}

pub async fn reschedule_jobs(
    client: &Client,
    data: RescheduleJobsData,
) -> Result<RescheduleJobsResult, RepositoryError> {
    let query = format!("select json_agg(r)::text rescheduled_jobs from {}.reschedule_jobs($1::bigint[], $2::timestamptz, $3::integer, $4::integer, $5::integer) r", *GRAPHILE_WORKER_SCHEMA);

    let result = client
        .query_one(
            &query,
            &[
                &data.job_ids,
                &data.run_at,
                &data.priority,
                &data.attempts.map(|attempts| attempts as i32),
                &data.max_attempts.map(|max_attempts| max_attempts as i32),
            ],
        )
        .await?
        .try_into()?;

    Ok(result)
}
