extern crate derive_more;

use actix_web::HttpResponse;
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use tokio_postgres::error::Error as PGError;

mod job_repository;

use crate::errors::HttpError;
pub use job_repository::*;

#[derive(Display, From, Debug)]
pub enum RepositoryError {
  NotFound,
  PGError(PGError),
  MappingError(SerdeError),
  PoolError(PoolError),
}

impl std::error::Error for RepositoryError {}

impl From<RepositoryError> for HttpError {
  fn from(error: RepositoryError) -> Self {
    match error {
      RepositoryError::NotFound => HttpError::not_found("NTFND", None),
      RepositoryError::PGError(ref err) => {
        #[cfg(debug_assertions)]
        {
          let mut error_data = serde_json::Map::new();
          error_data.insert("raw".to_string(), serde_json::Value::from(err.to_string()));
          return HttpError::internal_server_error(
            "POGER",
            Some(serde_json::Value::Object(error_data)),
          );
        }
        #[cfg(not(debug_assertions))]
        HttpError::internal_server_error("PGERR", None)
      }
      RepositoryError::MappingError(err) => {
        #[cfg(debug_assertions)]
        {
          let mut error_data = serde_json::Map::new();
          error_data.insert("raw".to_string(), serde_json::Value::from(err.to_string()));
          return HttpError::internal_server_error(
            "MAPER",
            Some(serde_json::Value::Object(error_data)),
          );
        }
        #[cfg(not(debug_assertions))]
        HttpError::internal_server_error("MPERR", None)
      }
      RepositoryError::PoolError(pool_error) => pool_error.into(),
    }
  }
}

impl From<RepositoryError> for HttpResponse {
  fn from(repository_error: RepositoryError) -> Self {
    let http_error: HttpError = repository_error.into();
    http_error.into()
  }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "direction", content = "field")]
#[serde(rename_all = "camelCase")]
pub enum RepositoryOrder<Field> {
  Asc(Field),
  Desc(Field),
}

pub trait ToSqlIdent {
  fn sql_ident(&self) -> String;
}

pub trait Order {
  fn order(&self) -> String;
}

impl<Field: ToSqlIdent> Order for RepositoryOrder<Field> {
  fn order(&self) -> String {
    match self {
      RepositoryOrder::Asc(field) => field.sql_ident() + " asc",
      RepositoryOrder::Desc(field) => field.sql_ident() + " desc",
    }
  }
}

impl<Field: Default> Default for RepositoryOrder<Field> {
  fn default() -> Self {
    RepositoryOrder::Asc(Field::default())
  }
}

impl<Field: ToSqlIdent + Default> Order for Option<RepositoryOrder<Field>> {
  fn order(&self) -> String {
    if let Some(order) = self {
      return order.order();
    }
    let default_repo_order: RepositoryOrder<Field> = RepositoryOrder::default();
    default_repo_order.order()
  }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryPagination {
  pub items_per_page: Option<u64>,
  pub page: Option<u64>,
}

pub trait Pagination {
  fn limit(&self) -> u64;
  fn offset(&self) -> u64;
}

impl Pagination for RepositoryPagination {
  fn limit(&self) -> u64 {
    self.items_per_page.unwrap_or(20).max(1).min(100)
  }

  fn offset(&self) -> u64 {
    (self.page.unwrap_or(1).max(1) - 1) * self.limit()
  }
}

impl Default for RepositoryPagination {
  fn default() -> Self {
    RepositoryPagination {
      items_per_page: None,
      page: None,
    }
  }
}

impl Pagination for Option<RepositoryPagination> {
  fn limit(&self) -> u64 {
    if let Some(pagination) = self {
      return pagination.limit();
    }
    20
  }

  fn offset(&self) -> u64 {
    if let Some(pagination) = self {
      return pagination.offset();
    }
    0
  }
}
