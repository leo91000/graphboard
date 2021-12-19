use crate::errors::HttpError;
use crate::models::AddJobData;
use crate::repositories::{add_job, complete_jobs, find_jobs};
use actix_web::web::{scope, Data, HttpRequest, Json};
use actix_web::{get, post, HttpResponse, Scope};
use deadpool_postgres::Pool;
use serde::Deserialize;

pub fn jobs_service() -> Scope {
    scope("/jobs")
        .service(find_jobs_route)
        .service(add_job_route)
        .service(complete_jobs_route)
}

#[get("")]
pub async fn find_jobs_route(
    req: HttpRequest,
    pool: Data<Pool>,
) -> Result<HttpResponse, HttpError> {
    let params = serde_qs::from_str(req.query_string())?;
    let jobs = find_jobs(&pool.get().await?, params).await?;
    Ok(HttpResponse::Ok().json(jobs))
}

#[post("")]
pub async fn add_job_route(
    pool: Data<Pool>,
    data: Json<AddJobData>,
) -> Result<HttpResponse, HttpError> {
    let job = add_job(&pool.get().await?, data.0).await?;
    Ok(HttpResponse::Ok().json(job))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteJobBody {
    pub job_ids: Vec<i64>,
}

#[post("/complete")]
pub async fn complete_jobs_route(
    pool: Data<Pool>,
    body: Json<CompleteJobBody>,
) -> Result<HttpResponse, HttpError> {
    let completed_jobs = complete_jobs(&pool.get().await?, body.job_ids.as_slice()).await?;
    Ok(HttpResponse::Ok().json(completed_jobs))
}
