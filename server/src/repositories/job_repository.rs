use crate::{
  models::{AddJobData, Job},
  repositories::{
    Order, Pagination, RepositoryError, RepositoryOrder, RepositoryPagination, ToSqlIdent,
  },
  CONFIG,
};
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use tracing::error;

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
  queue_name: Option<String>,
}

impl Default for FindJobsFilters {
  fn default() -> Self {
    FindJobsFilters {
      task_identifier: None,
      queue_name: None,
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
    let job_result = {
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
    };
    if let Err(error) = &job_result {
      error!(
        error = format!("{:?}", error).as_str(),
        "Error while deserializing row to Job"
      );
    }
    job_result
  }
}

pub async fn find_jobs(
  client: &Client,
  params: FindJobsParams,
) -> Result<FindJobsResult, RepositoryError> {
  let query = format!(
    r#"select j.* from {}.jobs j
        where ($1::text is null or $1::text = '' or j.task_identifier ilike concat('%', $1::text, '%')) and
              ($2::text is null or $2::text = '' or j.queue_name ilike concat('%', $2::text, '%'))"#,
    (*CONFIG).graphile_worker_schema
  );

  let stmt = format!(
    "select coalesce((select json_agg(data)::text from ({query} order by {} limit {} offset {}) \
     data), '[]') as jobs, coalesce((select count(c.*) from ({query}) c), 0) as count",
    params.order.order(),
    params.pagination.limit(),
    params.pagination.offset(),
    query = query,
  );

  let filters = params.filters();
  let result = client
    .query_one(
      stmt.as_str(),
      &[&filters.task_identifier, &filters.queue_name],
    )
    .await?
    .try_into()?;
  Ok(result)
}

pub async fn add_job(client: &Client, data: AddJobData) -> Result<Job, RepositoryError> {
  let query = format!(
    "select j.* from {}.add_job($1::text, $2::json, $3::text, $4::timestamptz, $5::integer, \
     $6::text, $7::integer, $8::text[], $9::text) j",
    (*CONFIG).graphile_worker_schema
  );

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
    (*CONFIG).graphile_worker_schema
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
  let query = format!(
    "select json_agg(f)::text permanently_failed_jobs from {}.permanently_fail_jobs($1::bigint[], \
     $2::text) f",
    (*CONFIG).graphile_worker_schema
  );

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
  let query = format!(
    "select json_agg(r)::text rescheduled_jobs from {}.reschedule_jobs($1::bigint[], \
     $2::timestamptz, $3::integer, $4::integer, $5::integer) r",
    (*CONFIG).graphile_worker_schema
  );

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveJobsResult {
  pub removed_job: Option<Job>,
}

impl TryFrom<Row> for RemoveJobsResult {
  type Error = RepositoryError;

  fn try_from(row: Row) -> Result<Self, Self::Error> {
    let raw_removed_job: Option<&str> = row.try_get("removed_job")?;
    match raw_removed_job {
      Some(data) => Ok(RemoveJobsResult {
        removed_job: Some(serde_json::from_str(data)?),
      }),
      None => Ok(RemoveJobsResult { removed_job: None }),
    }
  }
}

pub async fn remove_job<K: AsRef<str>>(
  client: &Client,
  job_key: K,
) -> Result<RemoveJobsResult, RepositoryError> {
  let query = format!(
    "select case when j is null then null else row_to_json(j)::text end removed_job from \
     {}.remove_job($1::text) j",
    (*CONFIG).graphile_worker_schema
  );

  let result = client
    .query_one(&query, &[&job_key.as_ref()])
    .await?
    .try_into()?;

  Ok(result)
}
